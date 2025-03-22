use crate::errors::ser::{Error, Result};
use havok_serde::{Serialize, ser::SerializeFlags};

use super::super::ByteSerializer;

impl SerializeFlags for &mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    /// Ignore this method because it is an XML method.
    #[inline]
    fn serialize_field<T>(&mut self, _key: &str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    // This method is for bytes only.
    #[inline]
    fn serialize_bits<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
