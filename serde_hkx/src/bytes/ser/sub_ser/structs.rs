use crate::{lib::*, tri};

use super::super::ByteSerializer;
use crate::errors::ser::{Error, Result};
use havok_serde::ser::{Serialize, SerializeStruct, Serializer, TypeSize};
use havok_types::Ulong;
use std::io::Write as _;

/// For bytes struct serializer.
///
/// # Why separate `ByteSerializer`?
/// Avoid mixing `local_fixups` for each field by creating local variables with separate Serializer.
pub struct StructSerializer<'a> {
    ser: &'a mut ByteSerializer,
    is_root: bool,
}

impl<'a> StructSerializer<'a> {
    pub fn new(ser: &'a mut ByteSerializer, is_root: bool) -> Self {
        Self { ser, is_root }
    }

    /// Common processing of fixed_array(e.g. `[bool; 3]`) and array(`hkArray`).
    fn serialize_array_common<V, T>(
        &mut self,
        array: V,
        size: TypeSize,
        local_src: u32,
        current_ser_pos: u64,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let len = array.as_ref().len() as u32;

        // push nest
        let mut is_nested = false;
        if !self.is_root {
            match size {
                TypeSize::Struct {
                    size_x86,
                    size_x86_64,
                } => {
                    let size = if self.ser.is_x86 {
                        size_x86
                    } else {
                        size_x86_64
                    };

                    let write_pointed_pos = current_ser_pos + ((len as u64) * size);
                    self.ser.nested_pos.push(write_pointed_pos); // To write inner member type.
                    is_nested = true;
                }
                TypeSize::String => {
                    let size = if self.ser.is_x86 { 4 } else { 8 };
                    let write_pointed_pos = current_ser_pos + ((len as u64) * size);
                    self.ser.nested_pos.push(write_pointed_pos); // To write pointed string data.
                    is_nested = true;
                }
                TypeSize::NonPtr => {}
            }
        }

        let pointed_pos = tri!(self.ser.goto_local_dst());
        self.ser.write_local_fixup_pair(local_src, pointed_pos)?;
        tri!(array.serialize(&mut *self.ser));
        if is_nested {
            self.ser.current_last_pos = self.ser.output.position();
            let _ = self.ser.nested_pos.pop();
        }
        self.ser.output.set_position(current_ser_pos); // back to previous position.
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
        let current_abs = self.ser.output.position();

        self.serialize_array_common(value, size, start_relative, current_abs)
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
        key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "serialize Array field({:#x}): {key}",
            self.ser.output.position()
        );

        // Write Array meta field
        let start_relative = tri!(self.ser.relative_position()); // Ptr type need to pointing data position(local.dst).
        let current_abs = self.ser.output.position();
        tri!(self.ser.serialize_ulong(Ulong::new(0))); // ptr size
        let len = value.as_ref().len() as u32;
        tri!(self.ser.serialize_uint32(len)); // array size
        tri!(self.ser.serialize_uint32(len | 1 << 31)); // Capacity(same as size) | Owned flag(32nd bit)

        if len == 0 {
            return Ok(());
        }
        self.serialize_array_common(value, size, start_relative, current_abs)
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
            tracing::trace!("nested_pos:({:#x?})", self.ser.nested_pos);
            self.ser.nested_pos.clear();
            self.ser.output.set_position(self.ser.current_last_pos);
        } else {
            let _ = self.ser.nested_pos.pop();
        }
        Ok(())
    }
}
