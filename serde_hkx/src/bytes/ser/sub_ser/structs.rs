use crate::{align, lib::*, tri};

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
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let next_src_pos = self.ser.output.position();

        {
            let pointed_pos = tri!(self.ser.goto_local_dst());
            self.ser.write_local_fixup_pair(local_src, pointed_pos)?;
        }
        let array_base_pos = self.ser.current_last_pos;

        // push nest
        let len = array.as_ref().len() as u32;
        match size {
            TypeSize::Struct {
                size_x86,
                size_x86_64,
            } => {
                let write_pointed_pos = {
                    let one_size = if self.ser.is_x86 {
                        size_x86
                    } else {
                        size_x86_64
                    };
                    array_base_pos + (one_size * (len as u64))
                }; // `local_dst` starting position of class.
                self.ser.pointed_pos.push(write_pointed_pos); // To write inner member type.
            }
            TypeSize::String => {
                let write_pointed_pos = {
                    let one_size = if self.ser.is_x86 { 4 } else { 8 };
                    array_base_pos + (one_size * (len as u64))
                }; // `local_dst` starting position of string.

                self.ser.pointed_pos.push(write_pointed_pos); // To write pointed string data.
            }
            TypeSize::NonPtr => {}
        }
        #[cfg(feature = "tracing")]
        tracing::trace!("pointed_pos:({:#x?})", self.ser.pointed_pos);

        tri!(array.serialize(&mut *self.ser));

        if size == TypeSize::NonPtr {
            let next_pointed_ser_pos = align!(self.ser.output.position(), 16_u64);
            self.ser.current_last_pos = next_pointed_ser_pos;
            if let Some(last) = self.ser.pointed_pos.last_mut() {
                *last = next_pointed_ser_pos; // Update to serialize the next pointed data.
            };
        } else {
            // HACK: unused first value to update;
            let pos = self.ser.pointed_pos.pop().unwrap();
            if let Some(last) = self.ser.pointed_pos.last_mut() {
                *last = pos;
            };
            self.ser.current_last_pos = pos;
        }

        self.ser.output.set_position(next_src_pos); // Go to the next field serialization position.
        Ok(())
    }

    /// Common processing of fixed_array(e.g. `[bool; 3]`)
    fn serialize_array_fixed<V, T>(
        &mut self,
        array: V,
        size: TypeSize,
        local_src: u32,
    ) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let need_local_jump = size != TypeSize::NonPtr;
        if need_local_jump {
            let pointed_pos = tri!(self.ser.goto_local_dst());
            let array_base_pos = self.ser.current_last_pos;
            self.ser.write_local_fixup_pair(local_src, pointed_pos)?;

            // push nest
            let len = array.as_ref().len() as u32;
            match size {
                TypeSize::Struct {
                    size_x86,
                    size_x86_64,
                } => {
                    let write_pointed_pos = {
                        let one_size = if self.ser.is_x86 {
                            size_x86
                        } else {
                            size_x86_64
                        };
                        array_base_pos + (one_size * (len as u64))
                    }; // `local_dst` starting position of class.
                    self.ser.pointed_pos.push(write_pointed_pos); // To write inner member type.
                }
                TypeSize::String => {
                    let write_pointed_pos = {
                        let one_size = if self.ser.is_x86 { 4 } else { 8 };
                        array_base_pos + (one_size * (len as u64))
                    }; // `local_dst` starting position of string.

                    self.ser.pointed_pos.push(write_pointed_pos); // To write pointed string data.
                }
                TypeSize::NonPtr => {}
            }
        };

        #[cfg(feature = "tracing")]
        tracing::trace!("pointed_pos:({:#x?})", self.ser.pointed_pos);

        #[cfg(feature = "tracing")]
        tracing::trace!("current position: {:#x}", self.ser.output.position());

        tri!(array.serialize(&mut *self.ser));

        if size != TypeSize::NonPtr {
            // HACK: unused first value to update;
            let pos = self.ser.pointed_pos.pop().unwrap();
            if let Some(last) = self.ser.pointed_pos.last_mut() {
                *last = pos;
            };
        }
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
        self.serialize_array_fixed(value, size, start_relative)
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

        let local_src = tri!(self.ser.relative_position()); // Ptr type need to pointing data position(local.dst).
        tri!(self.ser.serialize_ulong(Ulong::new(0))); // ptr size
        let len = value.as_ref().len() as u32;
        tri!(self.ser.serialize_uint32(len)); // array size
        tri!(self.ser.serialize_uint32(len | 1 << 31)); // Capacity(same as size) | Owned flag(32nd bit)

        if len == 0 {
            return Ok(());
        }
        self.serialize_array_common(value, size, local_src)
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
            self.ser.pointed_pos.clear();
            #[cfg(feature = "tracing")]
            tracing::trace!("current_last_pos:({:#x?})", self.ser.current_last_pos);
            self.ser.output.set_position(self.ser.current_last_pos);
        }
        Ok(())
    }
}
