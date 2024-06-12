use havok_types::StringPtr;

use super::{Deserialize, Deserializer, Error, Visitor};
use crate::lib::*;

////////////////////////////////////////////////////////////////////////////////

struct StrVisitor;

impl<'a> Visitor<'a> for StrVisitor {
    type Value = StringPtr<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a borrowed string")
    }

    fn visit_stringptr<E>(self, v: StringPtr<'a>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for std::borrow::Cow<'a, str> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(deserializer
            .deserialize_stringptr(StrVisitor)
            .unwrap()
            .into_inner()
            .unwrap())
    }
}

////////////////////////////////////////////////////////////////////////////////
impl<'de: 'a, 'a> Deserialize<'de> for i32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrimitiveVisitor;

        impl<'de> Visitor<'de> for PrimitiveVisitor {
            type Value = i32;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("i32")
            }

            fn visit_int32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(v)
            }
        }

        deserializer.deserialize_int32(PrimitiveVisitor)
    }
}
