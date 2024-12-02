//! Deserializing `enum` or `bitflags`
use super::XmlDeserializer;
use crate::errors::de::{Error, Result};
use havok_serde::de::{DeserializeSeed, EnumAccess, VariantAccess};

/// A enum for deserializing each variant in an `enum`.
///
/// # Expected XML
/// - Note that `</hkparam>` confirms the presence but does not consume.
/// ```xml
/// EVENT_MODE_IGNORE_FROM_GENERATOR</hkparam>
/// ```
#[derive(Debug)]
pub struct EnumDeserializer<'a, 'de: 'a> {
    de: &'a mut XmlDeserializer<'de>,
}

impl<'a, 'de> EnumDeserializer<'a, 'de> {
    #[inline]
    pub fn new(de: &'a mut XmlDeserializer<'de>) -> Self {
        EnumDeserializer { de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de> EnumAccess<'de> for EnumDeserializer<'_, 'de> {
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
impl<'de> VariantAccess<'de> for EnumDeserializer<'_, 'de> {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
