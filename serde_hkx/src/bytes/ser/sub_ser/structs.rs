use crate::{lib::*, tri};

use super::super::ByteSerializer;
use crate::bytes::ser::trait_impls::Align as _;
use crate::errors::ser::{Error, Result};
use havok_serde::ser::{Serialize, SerializeStruct, Serializer};
use havok_types::{CString, StringPtr};
use std::io::Write;

impl ByteSerializer {
    fn write_iter_local_fixups(&mut self) -> Result<()> {
        // NOTE: The strings contained in the classes are written after serialization of all classes in the array is completed.
        if let Some(strings) = self.str_array_buf.take() {
            #[cfg(feature = "tracing")]
            tracing::debug!("local_fixups_iter_src = {:?}", self.local_fixups_iter_src);

            // NOTE: avoid ownership errors by using `zip(self.local_fixups_iter_src)` and accessing with index.
            for (index, string) in strings.iter().enumerate() {
                let local_dst = self.relative_position()?;
                self.write_iter_local_fixup_pair(index, local_dst)?;
                self.output.write_all(string.as_bytes_with_nul())?;
                self.output.zero_fill_align(16)?;
            }
        };
        self.local_fixups_iter_src.clear();
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_cstring_meta_field(
        &mut self,
        key: &'static str,
        value: &CString,
    ) -> Result<(), Self::Error> {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "Serialize `CString` field({:#x}): {key}({value})",
            self.output.position()
        );

        if value.should_write_binary() {
            let str_start = self.relative_position()?;
            if self.str_array_buf.is_some() {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup.src: {str_start}({value})");
                self.local_fixups_iter_src.push(str_start)
            } else {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup_iter.src: {str_start}({value})");
                self.local_fixups_name_src.insert(key, str_start);
            };
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("Skip serializing `CString` field.");
        };

        // Write meta fields
        self.serialize_ulong(0) // ptr size
    }

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
            self.output.position()
        );

        if value.should_write_binary() {
            let str_start = self.relative_position()?;
            if self.str_array_buf.is_some() {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup.src: {str_start}({value})");
                self.local_fixups_iter_src.push(str_start)
            } else {
                #[cfg(feature = "tracing")]
                tracing::trace!("Add local_fixup_iter.src: {str_start}({value})");
                self.local_fixups_name_src.insert(key, str_start);
            };
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("Skip serializing StringPtr {key}.");
        };

        // Write meta fields
        self.serialize_ulong(0) // ptr size
    }

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
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "serialize Array field({:#x}): {key}",
            self.output.position()
        );
        if !value.as_ref().is_empty() {
            // Ptr type need to pointing data position(local.dst).
            let array_start = self.relative_position()?;
            self.local_fixups_name_src.insert(key, array_start);
        };

        // Write Array meta field
        let size = value.as_ref().len() as u32;
        self.serialize_ulong(0)?; // ptr size
        self.serialize_uint32(size)?; // array size
        self.serialize_uint32(size | 1 << 31) // Capacity(same as size) | Owned flag(32nd bit)
    }

    #[inline]
    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("serialize field({:#x}): {_key}", self.output.position());
        value.serialize(&mut **self)
    }

    /// Write `T` of `T* m_data`.
    #[inline]
    fn serialize_cstring_field(
        &mut self,
        key: &'static str,
        value: &CString,
    ) -> Result<(), Self::Error> {
        // NOTE: This indicates that when it is a class array, the write position is not yet determined even here
        // (because class arrays write the pointed data after all their array serialization is complete),
        // so it is temporarily written to `str_array_buf`, otherwise it is written in place This is indicated by the following.
        if self.str_array_buf.is_none() {
            let str_dst = self.relative_position()?;
            self.write_local_fixup_pair(key, str_dst)?;
        }
        value.serialize(&mut **self)
    }

    /// Write `T` of `T* m_data`.
    #[inline]
    fn serialize_stringptr_field(
        &mut self,
        key: &'static str,
        value: &StringPtr,
    ) -> Result<(), Self::Error> {
        // NOTE: This indicates that when it is a class array, the write position is not yet determined even here
        // (because class arrays write the pointed data after all their array serialization is complete),
        // so it is temporarily written to `str_array_buf`, otherwise it is written in place This is indicated by the following.
        if self.str_array_buf.is_none() {
            let str_dst = self.relative_position()?;
            self.write_local_fixup_pair(key, str_dst)?;
            self.output.zero_fill_align(16)?;
        }
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_fixed_array_field<V, T>(&mut self, _key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            "serialize FixedArray field({:#x}): {_key}",
            self.output.position()
        );

        // At this point, the data pointed to by the pointer is written to the temporary save
        // area. (Merged into output at the end of the array.
        if self.str_array_buf.is_none() {
            self.str_array_buf = Some(Vec::new());
        }
        tri!(value.serialize(&mut **self));
        self.write_iter_local_fixups()
    }

    fn serialize_array_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        // The data pointed to by the Array pointer (`T* m_data`) must first be aligned 16 bytes before it is written.
        self.output.zero_fill_align(16)?;

        if !value.as_ref().is_empty() {
            // The actual data location, i.e., the data position pointed to by ptr. It is local_fixup.dst.
            let array_dst = self.relative_position()?;
            self.write_local_fixup_pair(key, array_dst)?;
        }

        // NOTE: Please note the following!
        // - To avoid the malfunction of using `str_array_buf` when it is not a class array as a field, Vec is initialized here, where the array field processing is performed.
        // - If it is a class array as a field, the strings contained inside are written after all serialization in the array is completed.
        if self.str_array_buf.is_none() {
            self.str_array_buf = Some(Vec::new());
        }
        tri!(value.serialize(&mut **self));
        self.write_iter_local_fixups()
    }

    /// Even if it is skipped on XML, it is not skipped because it exists in binary data.
    #[inline]
    fn skip_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("serialize field({:#x}): {_key}", self.output.position());
        value.serialize(&mut **self)
    }

    /// This skip is for XML. Binary data must be written as usual.
    #[inline]
    fn skip_cstring_meta_field(&mut self, key: &'static str, value: &CString) -> Result<()> {
        self.serialize_cstring_meta_field(key, value)
    }

    /// This skip is for XML. Binary data must be written as usual.
    #[inline]
    fn skip_stringptr_meta_field(&mut self, key: &'static str, value: &StringPtr) -> Result<()> {
        self.serialize_stringptr_meta_field(key, value)
    }

    #[inline]
    fn skip_array_meta_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.serialize_array_meta_field(key, value)
    }

    fn pad_field<T>(&mut self, x86_pads: &T, x64_pads: &T) -> Result<()>
    where
        T: ?Sized + AsRef<[u8]>,
    {
        match self.is_x86 {
            true => {
                if x86_pads.as_ref().is_empty() {
                    return Ok(());
                };
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "padding: {} -> current position: {:#x}",
                    x86_pads.as_ref().len(),
                    self.output.position(),
                );

                self.output.write(x86_pads.as_ref())
            }
            false => {
                if x64_pads.as_ref().is_empty() {
                    return Ok(());
                };
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "padding: {} -> current position: {:#x}",
                    x64_pads.as_ref().len(),
                    self.output.position(),
                );

                self.output.write(x64_pads.as_ref())
            }
        }?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        // NOTE: The offset map pointing to the pointer for field is different for each struct and must be reset here.
        self.local_fixups_name_src.clear();
        Ok(())
    }
}
