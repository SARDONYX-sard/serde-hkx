use super::super::ByteSerializer;
use crate::errors::ser::{Error, Result};
use havok_serde::{ser::SerializeSeq, Serialize};
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
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_stringptr_element(&mut self, value: &StringPtr) -> Result<()> {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}
