use std::io::Write;

use super::super::ByteSerializer;
use crate::{
    bytes::ser::trait_impls::Align as _,
    errors::ser::{Error, Result},
};
use havok_serde::{
    ser::{SerializeSeq, Serializer as _},
    Serialize,
};
use havok_types::{CString, StringPtr};

impl<'a> SerializeSeq for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_primitive_element<T>(
        &mut self,
        value: &T,
        _index: usize,
        _len: usize,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    /// This method is called on HavokClasses array.(Write start)
    ///
    /// Therefore, it is necessary to record the write position of this in virtual_fixup.
    #[inline]
    fn serialize_class_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_math_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_cstring_element(&mut self, value: &CString) -> Result<()> {
        let iter_src = self.relative_position()?;
        self.local_fixups_iter_src.push(iter_src);
        self.serialize_ulong(0)?; // ptr size

        // At this point, the data pointed to by the pointer is written to the temporary save
        // area. (Merged into output at the end of the array.
        if self.str_array_buf.is_none() {
            self.str_array_buf = Some(Vec::new());
        }
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_stringptr_element(&mut self, value: &StringPtr) -> Result<()> {
        let iter_src = self.relative_position()?;
        self.local_fixups_iter_src.push(iter_src);
        self.serialize_ulong(0)?; // ptr size

        // At this point, the data pointed to by the pointer is written to the temporary save
        // area. (Merged into output at the end of the array.
        if self.str_array_buf.is_none() {
            self.str_array_buf = Some(Vec::new());
        }
        value.serialize(&mut **self)
    }

    /// In Byte Serializer, [`SerializeSeq`] is called only when writing the data pointed to by the pointer.
    /// When the data has been written, if it is a StringPtr, the actual state of the data must be written here.
    fn end(self) -> Result<()> {
        if let Some(strings) = self.str_array_buf.take() {
            for (index, string) in strings.iter().enumerate() {
                self.write_iter_local_fixup_pair(index, self.relative_position()?)?;

                self.output.write_all(string.as_bytes_with_nul())?;
                self.output.zero_fill_align(16)?;
            }
        };
        self.local_fixups_iter_src.clear();
        Ok(())
    }
}
