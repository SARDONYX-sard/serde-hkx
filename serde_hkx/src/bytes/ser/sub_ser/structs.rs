use crate::{align, errors::ser::NotFoundPointedPositionSnafu, lib::*, tri};

use super::super::ByteSerializer;
use crate::errors::ser::{Error, Result};
use havok_serde::ser::{Serialize, SerializeStruct, Serializer, TypeSize};
use havok_types::Ulong;
use snafu::OptionExt as _;
use std::io::Write as _;

/// For bytes struct serializer.
///
/// # Why separate `ByteSerializer`?
/// Avoid mixing `local_fixups` for each field by creating local variables with separate Serializer.
pub struct StructSerializer<'a> {
    ser: &'a mut ByteSerializer,
    /// Is it a virtual fixups class? (This indicates it is not a class within a field)
    is_root: bool,
}

impl<'a> StructSerializer<'a> {
    pub const fn new(ser: &'a mut ByteSerializer, is_root: bool) -> Self {
        Self { ser, is_root }
    }

    /// serialize elements of `hkArray`.
    fn serialize_array_elements<V, T>(
        &mut self,
        array: V,
        size: TypeSize,
        local_src: u32,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let next_src_pos = self.ser.output.position();

        let (current_abs, last_local_dst) = tri!(self.ser.goto_latest_local_dst(true));
        self.ser.write_local_fixup_pair(local_src, last_local_dst)?;

        let len = array.as_ref().len() as u32;

        match size {
            TypeSize::Struct {
                size_x86,
                size_x86_64,
            } => {
                self.ser.is_in_str_array = false;
                #[rustfmt::skip]
                let one_size = if self.ser.is_x86 { size_x86 } else { size_x86_64 };

                let write_pointed_pos =
                    calc_array_elements_write_pos(current_abs, len, one_size, "Struct");
                self.ser.pointed_pos.push(write_pointed_pos);
            }
            TypeSize::String => {
                self.ser.is_in_str_array = true;
                let one_size = if self.ser.is_x86 { 4 } else { 8 };

                let write_pointed_pos =
                    calc_array_elements_write_pos(current_abs, len, one_size, "String");
                self.ser.pointed_pos.push(write_pointed_pos);
            }
            TypeSize::NonPtr => {}
        }
        #[cfg(feature = "tracing")]
        tracing::trace!("pointed_pos:({:#x?})", self.ser.pointed_pos);

        tri!(array.serialize(&mut *self.ser)); // serialize elements all
        self.ser.is_in_str_array = false;

        let updated_last_local_dst = if matches!(size, TypeSize::NonPtr) {
            self.ser.output.position()
        } else {
            // Deletes the ptr write destination for the String/Struct pushed by this function.
            tri!(
                self.ser
                    .pointed_pos
                    .pop()
                    .context(NotFoundPointedPositionSnafu)
            )
        };
        tri!(
            self.ser
                .update_last_local_dst(align!(updated_last_local_dst, 16_u64))
        );

        self.ser.output.set_position(next_src_pos); // Go back to the next field serialization position.
        Ok(())
    }

    /// Serialize elements of fixed_array(e.g. `[bool; 3]`)
    ///
    /// # NOTE
    /// `hkbGeneratorSyncInfo.syncPoints: [hkbGeneratorSyncInfoSyncPoint; 8]` and this is 64 bytes.
    /// In other words, embed everything except String types directly into the array.
    ///
    /// # Undefined behavior
    /// If `hkArray` is encountered, undefined behavior occurs.
    /// However, currently there are no classes in hk2010 that place `[hkArray<T>; N]`.
    fn serialize_fixed_array_elements<V, T>(
        &mut self,
        array: V,
        size: TypeSize,
        local_src: u32,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        // NOTE: In C++, structs within arrays reside on the stack, not the heap. Unlike hkArray,
        //       they are allocated in place because their size affects the allocation.
        if matches!(size, TypeSize::NonPtr | TypeSize::Struct { .. }) {
            tri!(array.serialize(&mut *self.ser)); // serialize [T; N] all.
            return Ok(());
        };
        // ---------------------------------------------------------------------------------
        // TODO: `[hkStringPtr; N]` does not currently exist, so We're not sure if this is correct.

        let next_src_pos = self.ser.output.position();

        let (current_abs, last_local_dst) = tri!(self.ser.goto_latest_local_dst(true));
        self.ser.write_local_fixup_pair(local_src, last_local_dst)?;

        let len = array.as_ref().len() as u32;

        self.ser.is_in_str_array = true;
        let one_size = if self.ser.is_x86 { 4 } else { 8 };

        let write_pointed_pos = calc_array_elements_write_pos(current_abs, len, one_size, "String");
        self.ser.pointed_pos.push(write_pointed_pos);

        tri!(array.serialize(&mut *self.ser)); // serialize [T; N] all.

        // Deletes the ptr write destination for the String/Struct pushed by this function.
        let updated_last_local_dst = tri!(
            self.ser
                .pointed_pos
                .pop()
                .context(NotFoundPointedPositionSnafu)
        );
        tri!(
            self.ser
                .update_last_local_dst(align!(updated_last_local_dst, 16_u64))
        );
        self.ser.output.set_position(next_src_pos); // Go back to the next field serialization position.

        Ok(())
    }
}

impl<'a> SerializeStruct for StructSerializer<'a> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("serialize field({:#x}): {_key}", self.ser.output.position());
        value.serialize(&mut *self.ser)
    }

    /// Even if it is skipped on XML, it is not skipped because it exists in binary data.
    #[inline]
    fn skip_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_field(key, value)
    }

    fn pad_field<T>(&mut self, x86_pads: &T, x64_pads: &T) -> Result<()>
    where
        T: ?Sized + AsRef<[u8]>,
    {
        let pads = match self.ser.is_x86 {
            true => x86_pads.as_ref(),
            false => x64_pads.as_ref(),
        };

        if pads.is_empty() {
            return Ok(());
        };
        self.ser.output.write_all(pads)?;
        #[cfg(feature = "tracing")]
        {
            let pads_len = pads.len();
            let current_position = self.ser.output.position();
            tracing::trace!("padding: {pads_len} -> current position: {current_position:#x}");
        }
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Fixed Array

    #[inline]
    fn serialize_fixed_array_field<V, T>(
        &mut self,
        _key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "serialize FixedArray field({:#x}): {_key}",
            self.ser.output.position()
        );

        if value.as_ref().is_empty() {
            return Ok(());
        }
        let start_relative = tri!(self.ser.relative_position()); // Ptr type need to pointing data position(local.dst).
        self.serialize_fixed_array_elements(value, size, start_relative)
    }

    #[inline]
    fn skip_fixed_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.serialize_fixed_array_field(key, value, size)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Array

    /// In the binary serialization of hkx, we are at this stage writing each field of the structure.
    /// ptr type writes only the size of C++ `Array` here, since the data pointed to by the pointer
    /// will be written later.
    ///
    /// That is, ptr(x86: 12bytes, x64: 16bytes).
    fn serialize_array_field<V, T>(
        &mut self,
        _key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "serialize Array field({:#x}): {_key}",
            self.ser.output.position()
        );

        let local_src = tri!(self.ser.relative_position()); // Ptr type need to pointing data position(local.dst).
        tri!(self.ser.serialize_ulong(Ulong::new(0))); // ptr size
        let len = value.as_ref().len() as u32;
        tri!(self.ser.serialize_uint32(len)); // array size
        tri!(self.ser.serialize_uint32(len | (1 << 31))); // Capacity(same as size) | Owned flag(32nd bit)

        if len == 0 {
            return Ok(());
        }
        self.serialize_array_elements(value, size, local_src)
    }

    #[inline]
    fn skip_array_field<V, T>(&mut self, key: &'static str, value: V, size: TypeSize) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.serialize_array_field(key, value, size)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #[inline]
    fn end(self) -> Result<()> {
        if self.is_root {
            #[cfg(feature = "tracing")]
            tracing::trace!("pointed_pos:({:#x?})", self.ser.pointed_pos);
            // NOTE Write the next virtual fixup after the contents pointed to by the ptr.
            // Otherwise, we'll overwrite it!
            tri!(self.ser.goto_latest_local_dst(true));
            self.ser.pointed_pos.clear();
        }
        Ok(())
    }
}

/// Calculate the write position for an array element,
///
/// applying align16 if twice nested.(The reason is unknown. However, it has been determined through hkx binary analysis.)
fn calc_array_elements_write_pos(
    current_abs_pos: u64,
    len: u32,
    one_size: u64,
    _type_kind: &'static str,
) -> u64 {
    let write_pointed_pos = current_abs_pos + (one_size * len as u64);

    #[cfg(feature = "tracing")]
    tracing::trace!(
        "Calculate hkArray<{_type_kind}> next local dst: array_base_pos({current_abs_pos:#x}) + one_size({one_size}) * len({len}) = {write_pointed_pos:#x}"
    );

    write_pointed_pos
}
