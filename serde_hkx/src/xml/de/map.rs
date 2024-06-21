//! Deserializing each field element in an `Struct`

use super::{
    parser::tag::{end_tag, field_start_close_tag, field_start_open_tag},
    XmlDeserializer,
};
use crate::{de_error::DeError, tri};
use havok_serde::de::MapAccess;

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
pub struct MapDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut XmlDeserializer<'de>,
    index: usize,
    fields: &'static [&'static str],
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(de: &'a mut XmlDeserializer<'de>, fields: &'static [&'static str]) -> Self {
        Self {
            de,
            index: 0,
            fields,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: havok_serde::de::DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.parse_peek(end_tag("hkobject")).is_ok() && self.fields.len() == self.index {
            return Ok(None);
        }

        tri!(self.de.parse(field_start_open_tag()));
        let key = seed.deserialize(&mut *self.de).map(Some); // Deserialize a map key.
        tri!(self.de.parse(field_start_close_tag()));
        self.index += 1;

        key
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: havok_serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de) // Deserialize a map value.
    }
}
