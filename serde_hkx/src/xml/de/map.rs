//! Deserializing each field element in an `Struct`
use super::{
    parser::tag::{end_tag, field_start_close_tag, field_start_open_tag},
    XmlDeserializer,
};
use crate::{errors::de::Error, tri, xml::de::parser::tag::array_field_start_close_tag};
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
    /// To check fields length
    index: usize,
    ptr_name: Option<Pointer>,
    fields: &'static [&'static str],
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
            index: 0,
            ptr_name,
            fields,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&self) -> Option<Pointer> {
        self.ptr_name
    }

    // Parse e.g. `<hkparam name="worldUpWS">`
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.parse_peek(end_tag("hkobject")).is_ok() && self.fields.len() == self.index {
            return Ok(None);
        }

        tri!(self.de.parse(field_start_open_tag())); // Parse `<hkparam name=`
        let key = seed.deserialize(&mut *self.de).map(Some); // Deserialize a map key.
        tri!(self.de.parse(field_start_close_tag())); // Parse `>`
        self.index += 1;

        key
    }

    // Parse e.g. `<hkparam name="worldUpWS">`
    fn next_array_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.parse_peek(end_tag("hkobject")).is_ok() && self.fields.len() == self.index {
            return Ok(None);
        }
        // Parse `<hkparam name=`
        tri!(self.de.parse(field_start_open_tag()));
        //  `Deserialize` of the struct -> `deserialize_key` in `XmlDeserializer` -> Parse `"`, key, `"`
        let key = seed.deserialize(&mut *self.de).map(Some);
        // Parse ` numelements="3">`
        let _len = tri!(self.de.parse(array_field_start_close_tag()));
        #[cfg(feature = "tracing")]
        tracing::debug!(_len);
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
