//! Deserializing `enum` or `bitflags`
use super::BytesDeserializer;
use crate::errors::de::{Error, Result};
use havok_serde::de::{DeserializeSeed, EnumAccess, VariantAccess};

/// A enum for deserializing each variant in an `enum`.
///
/// # Expected bytes
/// ```log
/// 08
/// ```
#[derive(Debug)]
pub struct EnumDeserializer<'a, 'de: 'a> {
    de: &'a mut BytesDeserializer<'de>,
}

impl<'a, 'de> EnumDeserializer<'a, 'de> {
    #[inline]
    pub fn new(de: &'a mut BytesDeserializer<'de>) -> Self {
        EnumDeserializer { de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for EnumDeserializer<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    #[inline]
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

/// `VariantAccess` is provided to the `Visitor` to give it the ability to see
/// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for EnumDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
