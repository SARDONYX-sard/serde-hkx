use crate::lib::*;

use super::super::ByteSerializer;
use crate::bytes::ser::trait_impls::Align as _;
use crate::errors::ser::{Error, Result};
use havok_serde::ser::{Serialize, SerializeStruct, Serializer};
use havok_types::{CString, StringPtr};
use std::io::Write;

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
            "serialize CString field({:#x}): {key}({value})",
            self.output.position()
        );
        if value.should_write_binary() {
            let str_start = self.relative_position()?;
            self.local_fixups_name_src.insert(key, str_start);
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("skip serializing CString {key}.");
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
            "serialize StringPtr field({:#x}): {key}({value})",
            self.output.position()
        );
        if value.should_write_binary() {
            let str_start = self.relative_position()?;
            self.local_fixups_name_src.insert(key, str_start);
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("skip serializing StringPtr {key}.");
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
        let str_dst = self.relative_position()?;
        self.write_local_fixup_pair(key, str_dst)?;

        value.serialize(&mut **self)
    }

    /// Write `T` of `T* m_data`.
    #[inline]
    fn serialize_stringptr_field(
        &mut self,
        key: &'static str,
        value: &StringPtr,
    ) -> Result<(), Self::Error> {
        let str_dst = self.relative_position()?;
        self.write_local_fixup_pair(key, str_dst)?;

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
        value.serialize(&mut **self)
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

        value.serialize(&mut **self)
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
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "serialize pads({:#x}): x86({})/x64({})",
            self.output.position(),
            x86_pads.as_ref().len(),
            x64_pads.as_ref().len(),
        );
        match self.is_x86 {
            true => {
                if x86_pads.as_ref().is_empty() {
                    return Ok(());
                };
                self.output.write(x86_pads.as_ref())
            }
            false => {
                if x64_pads.as_ref().is_empty() {
                    return Ok(());
                };
                self.output.write(x64_pads.as_ref())
            }
        }?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        // NOTE: The offset map pointing to the pointer for field is different for each struct and must be reset here.
        self.local_fixups_name_src.clear();
        self.output.zero_fill_align(16)?;
        Ok(())
    }
}
