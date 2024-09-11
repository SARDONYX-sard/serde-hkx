use crate::{lib::*, tri};

use super::super::ByteSerializer;
use crate::bytes::ser::trait_impls::{Align as _, LocalFixupsWriter as _};
use crate::errors::ser::{Error, MissingLocalFixupSnafu, Result};
use havok_serde::ser::{Serialize, SerializeStruct, Serializer};
use havok_types::{CString, StringPtr, Ulong};
use std::collections::HashMap;
use std::io::Write;

/// For bytes struct serializer.
///
/// # Why separate `ByteSerializer`?
/// Avoid mixing `local_fixups` for each field by creating local variables with separate Serializer.
pub struct StructSerializer<'a> {
    ser: &'a mut ByteSerializer,

    // ---- local fixup information
    /// - key: struct field name
    /// - value: local_fixup.src (i.e., the start position where the pointer size is written)
    local_fixups_name_src: HashMap<&'static str, u32>,
}

impl<'a> StructSerializer<'a> {
    pub fn new(ser: &'a mut ByteSerializer) -> Self {
        Self {
            ser,
            local_fixups_name_src: HashMap::new(),
        }
    }

    fn write_iter_local_fixups(&mut self) -> Result<()> {
        // NOTE: The strings contained in the classes are written after serialization of all classes in the array is completed.
        if let Some(strings) = self.ser.str_array_buf.take() {
            // NOTE: avoid ownership errors by using `zip(self.local_fixups_iter_src)` and accessing with index.
            for (index, string) in strings.iter().enumerate() {
                let local_dst = self.ser.relative_position()?;
                self.write_iter_local_fixup_pair(index, local_dst)?;
                self.ser.output.write_all(string.as_bytes_with_nul())?;
                self.ser.output.zero_fill_align(16)?;
            }
        };
        self.ser.local_fixups_iter_src.clear();
        Ok(())
    }

    /// Write a pair of local_fixups to the temporary local_fixups buffer
    ///
    /// # Info
    /// When dst is known, src is already known and can be written.
    /// # Note
    fn write_local_fixup_pair(&mut self, key: &str, local_dst: u32) -> Result<()> {
        match self.local_fixups_name_src.get(key) {
            Some(local_src) => {
                #[cfg(feature = "tracing")]
                {
                    let src_abs = self.ser.abs_data_offset + local_src;
                    let dst_abs = self.ser.abs_data_offset + local_dst;
                    tracing::trace!("local_fixup of {key}: src({local_src}/abs: {src_abs:#x}), dst({local_dst}/abs: {dst_abs:#x})");
                }

                self.ser.local_fixups.write_local_fixups(
                    *local_src,
                    local_dst,
                    self.ser.is_little_endian,
                )?;
                Ok(())
            }
            None => {
                #[cfg(feature = "tracing")]
                tracing::debug!("Skip because there is no corresponding `local_fixup.src`. {key} -> dst({local_dst})");
                Ok(())
            }
        }
    }

    /// Write a pair of local_fixups to the temporary local_fixups buffer
    ///
    /// # Info
    /// When dst is known, src is already known and can be written.
    fn write_iter_local_fixup_pair(&mut self, index: usize, local_dst: u32) -> Result<()> {
        match self.ser.local_fixups_iter_src.get(index) {
            Some(&local_src) => {
                #[cfg(feature = "tracing")]
                {
                    let src_abs = self.ser.abs_data_offset + local_src;
                    let dst_abs = self.ser.abs_data_offset + local_dst;
                    tracing::trace!("local_fixup_iter of {index}th: src({local_src}/abs: {src_abs:#x}), dst({local_dst}/abs: {dst_abs:#x})");
                }

                self.ser.local_fixups.write_local_fixups(
                    local_src,
                    local_dst,
                    self.ser.is_little_endian,
                )?;
                Ok(())
            }
            None => MissingLocalFixupSnafu { dst: local_dst }.fail(),
        }
    }

    /// Write `T` of `T* m_data`.(`CString` & `StringPtr`)
    fn serialize_string<V>(&mut self, key: &'static str, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        // NOTE: This indicates that when it is a class array, the write position is not yet determined even here
        // (because class arrays write the pointed data after all their array serialization is complete),
        // so it is temporarily written to `str_array_buf`, otherwise it is written in place This is indicated by the following.
        if self.ser.str_array_buf.is_none() {
            self.ser.output.zero_fill_align(16)?; // NOTE: The field string is written after align16.
            let str_dst = self.ser.relative_position()?;
            self.write_local_fixup_pair(key, str_dst)?;
            self.ser.output.zero_fill_align(16)?;
        }
        value.serialize(&mut *self.ser)
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
    fn skip_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("serialize field({:#x}): {_key}", self.ser.output.position());
        value.serialize(&mut *self.ser)
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
    // CString

    fn serialize_cstring_meta_field(
        &mut self,
        key: &'static str,
        value: &CString,
    ) -> Result<(), Self::Error> {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Serialize `CString` field({:#x}): {key}(\"{value}\")",
            self.ser.output.position()
        );

        if value.should_write_binary() {
            let str_start = self.ser.relative_position()?;
            if self.ser.str_array_buf.is_some() {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup_iter_src: {str_start}");
                self.ser.local_fixups_iter_src.push(str_start)
            } else {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup_name_src: {str_start}");
                self.local_fixups_name_src.insert(key, str_start);
            };
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("Skip serializing `CString` field.");
        };

        // Write meta fields
        self.ser.serialize_ulong(Ulong::new(0)) // ptr size
    }

    /// This skip is for XML. Binary data must be written as usual.
    #[inline]
    fn skip_cstring_meta_field(&mut self, key: &'static str, value: &CString) -> Result<()> {
        self.serialize_cstring_meta_field(key, value)
    }

    #[inline]
    fn serialize_cstring_field(&mut self, key: &'static str, value: &CString) -> Result<()> {
        self.serialize_string(key, value)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // StringPtr

    /// In the binary serialization of hkx, we are at this stage writing each field of the structure.
    /// ptr type writes only the size of C++ `StringPtr` here, since the data pointed to by the pointer
    /// will be written later.
    ///
    /// That is, ptr(x86: 4bytes, x64: 8bytes).
    fn serialize_stringptr_meta_field(
        &mut self,
        key: &'static str,
        value: &StringPtr,
    ) -> Result<(), Self::Error> {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Serialize `StringPtr` field({:#x}): {key}(\"{value}\")",
            self.ser.output.position()
        );

        if value.should_write_binary() {
            let str_start = self.ser.relative_position()?;
            if self.ser.str_array_buf.is_some() {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup_iter_src: {str_start}");
                self.ser.local_fixups_iter_src.push(str_start)
            } else {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup_name_src: {str_start}");
                self.local_fixups_name_src.insert(key, str_start);
            };
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("Skip serializing StringPtr {key}.");
        };

        // Write meta fields
        self.ser.serialize_ulong(Ulong::new(0)) // ptr size
    }

    /// This skip is for XML. Binary data must be written as usual.
    #[inline]
    fn skip_stringptr_meta_field(&mut self, key: &'static str, value: &StringPtr) -> Result<()> {
        self.serialize_stringptr_meta_field(key, value)
    }

    #[inline]
    fn serialize_stringptr_field(&mut self, key: &'static str, value: &StringPtr) -> Result<()> {
        self.serialize_string(key, value)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Fixed Array

    #[inline]
    fn serialize_fixed_array_field<V, T>(&mut self, _key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "serialize FixedArray field({:#x}): {_key}",
            self.ser.output.position()
        );

        // At this point, the data pointed to by the pointer is written to the temporary save
        // area. (Merged into output at the end of the array.
        if self.ser.str_array_buf.is_none() {
            self.ser.str_array_buf = Some(Vec::new());
        }
        tri!(value.serialize(&mut *self.ser));
        self.write_iter_local_fixups()
    }

    #[inline]
    fn skip_fixed_array_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.serialize_fixed_array_field(key, value)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Array

    /// In the binary serialization of hkx, we are at this stage writing each field of the structure.
    /// ptr type writes only the size of C++ `Array` here, since the data pointed to by the pointer
    /// will be written later.
    ///
    /// That is, ptr(x86: 12bytes, x64: 16bytes).
    fn serialize_array_meta_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let value = value.as_ref();

        #[cfg(feature = "tracing")]
        tracing::trace!(
            "serialize Array field({:#x}): {key}",
            self.ser.output.position()
        );
        if !value.is_empty() {
            // Ptr type need to pointing data position(local.dst).
            let array_start = self.ser.relative_position()?;
            self.local_fixups_name_src.insert(key, array_start);
        };

        // Write Array meta field
        let size = value.len() as u32;
        self.ser.serialize_ulong(Ulong::new(0))?; // ptr size
        self.ser.serialize_uint32(size)?; // array size
        self.ser.serialize_uint32(size | 1 << 31) // Capacity(same as size) | Owned flag(32nd bit)
    }

    #[inline]
    fn skip_array_meta_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.serialize_array_meta_field(key, value)
    }

    fn serialize_array_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        if value.as_ref().is_empty() {
            return Ok(());
        }

        // The data pointed to by the Array pointer (`T* m_data`) must first be aligned 16 bytes before it is written.
        self.ser.output.zero_fill_align(16)?;

        // The actual data location, i.e., the data position pointed to by ptr. It is local_fixup.dst.
        let array_dst = self.ser.relative_position()?;
        self.write_local_fixup_pair(key, array_dst)?;

        // NOTE: Please note the following!
        // - To avoid the malfunction of using `str_array_buf` when it is not a class array as a field, Vec is initialized here, where the array field processing is performed.
        // - If it is a class array as a field, the strings contained inside are written after all serialization in the array is completed.
        if self.ser.str_array_buf.is_none() {
            self.ser.str_array_buf = Some(Vec::new());
        }
        tri!(value.serialize(&mut *self.ser));
        self.write_iter_local_fixups()
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}
