//! Deserializing each field element in an `Struct`
use super::{parser::tag::class_start_tag, XmlDeserializer};
use crate::{errors::de::Error, tri};
use havok_serde::de::{ClassIndexAccess, DeserializeSeed};

/// A structure for deserializing each element in an `Struct`.
///
/// # Expected XML
/// ```xml
/// <hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7"> <!-- <-Parsed by `deserialize_struct` -->
///   <!-- memSizeAndFlags SERIALIZE_IGNORED -->
///   <!-- referenceCount SERIALIZE_IGNORED -->
///   <hkparam name="worldUpWS">(0.000000 0.000000 1.000000 0.000000)</hkparam>
///   <hkparam name="stringData">#0009</hkparam>
///   <hkparam name="defaultEventMode">EVENT_MODE_IGNORE_FROM_GENERATOR</hkparam>
/// </hkobject>
/// ```
#[derive(Debug)]
pub struct ClassIndexMapDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut XmlDeserializer<'de>,
}

impl<'a, 'de> ClassIndexMapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    #[inline]
    pub fn new(de: &'a mut XmlDeserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> ClassIndexAccess<'de> for ClassIndexMapDeserializer<'a, 'de> {
    type Error = Error;

    fn next_key(&mut self) -> Result<&'de str, Self::Error> {
        let (_ptr, class_name, _signature) = tri!(self.de.parse_peek(class_start_tag()));
        Ok(class_name)
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.de);
        self.de.in_struct = false;
        value
    }
}
