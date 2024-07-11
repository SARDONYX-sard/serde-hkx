//! Deserializing each field element in an `Struct`

use crate::tri;

use super::{
    parser::tag::{end_tag, field_start_close_tag, field_start_open_tag},
    XmlDeserializer,
};
use crate::errors::de::{Error, Result};
use havok_serde::de::{DeserializeSeed, MapAccess};
use havok_types::Pointer;

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
pub struct MapDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut XmlDeserializer<'de>,
    fields: &'static [&'static str],

    /// To check fields length
    index: usize,
    ptr_name: Option<Pointer>,
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(
        de: &'a mut XmlDeserializer<'de>,
        ptr_name: Option<Pointer>,
        fields: &'static [&'static str],
    ) -> Self {
        Self {
            de,
            fields,
            index: 0,
            ptr_name,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&mut self) -> Option<Pointer> {
        if self.de.in_struct {
            None
        } else {
            self.ptr_name
        }
    }

    // Parse e.g. `<hkparam name="worldUpWS">`, `<hkparam name="boneWeights" numelements="90">`
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.fields.len() == self.index {
            return Ok(None);
        }

        tri!(self.de.parse(field_start_open_tag())); // Parse `<hkparam name=`
        let key = seed.deserialize(&mut *self.de).map(Some); // Parse `"string"`
        tri!(self.de.parse(field_start_close_tag())); // Parse `>` or ` numelements="3">`
        self.index += 1;

        key
    }

    // Parse e.g. `(0.000000 0.000000 1.000000 0.000000)</hkparam>`
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = tri!(seed.deserialize(&mut *self.de));
        tri!(self.de.parse(end_tag("hkparam")));
        Ok(value)
    }
}
