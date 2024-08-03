use super::super::ByteSerializer;
use crate::errors::ser::{Error, Result};
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
        if value.should_write_binary() {
            let iter_src = self.relative_position()?;
            self.local_fixups_iter_src.push(iter_src);
            self.serialize_ulong(0)?; // ptr size
            #[cfg(feature = "tracing")]
            tracing::debug!("local_fixup_iter_src = {iter_src}");
        };
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_stringptr_element(&mut self, value: &StringPtr) -> Result<()> {
        if value.should_write_binary() {
            let iter_src = self.relative_position()?;
            self.local_fixups_iter_src.push(iter_src);
            self.serialize_ulong(0)?; // ptr size
            #[cfg(feature = "tracing")]
            tracing::debug!("local_fixup_iter_src = {iter_src}");
        };

        value.serialize(&mut **self)
    }

    // NOTE: If we write with `Seq` `end`, we will not be able to use `SerializeStruct` to write
    //       the pointer destination at the end of writing the field if there
    //       is an `Array<String>` in the `Array<Class>`.
    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}
