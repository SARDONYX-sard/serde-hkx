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
    ptr_name: Option<Pointer>,
    class_name: &'static str,
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(
        de: &'a mut XmlDeserializer<'de>,
        ptr_name: Option<Pointer>,
        class_name: &'static str,
    ) -> Self {
        Self {
            de,
            ptr_name,
            class_name,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&mut self) -> Option<Pointer> {
        self.ptr_name.take()
    }

    // Parse e.g. `<hkparam name="worldUpWS"`, `<hkparam name="boneWeights"`
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.parse_peek(end_tag("hkobject")).is_ok() {
            return Ok(None);
        }
        // Avoid infinite loops by checking the end of XML.
        if self.de.input.is_empty() {
            return Err(Error::Eof);
        }

        tri!(self.de.parse_next(field_start_open_tag(self.class_name))); // Parse `<hkparam name=`
        seed.deserialize(&mut *self.de).map(Some) // Parse `"string"`
    }

    // Parse e.g. `>(0.000000 0.000000 1.000000 0.000000)</hkparam>`
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // HACK: If the `strict` feature is disabled, fall back to the default value
        // if there is an error retrieving the value in the `havok_classes` crate, so check
        // for `>` here and omit the implementation of the `/>` shorthand notation.
        let _len = tri!(self.de.parse_next(field_start_close_tag())); // Parse `>` or ` numelements="3">`
        #[cfg(feature = "tracing")]
        if let Some(numelements) = _len {
            tracing::debug!(numelements);
        }

        let value = tri!(seed.deserialize(&mut *self.de));
        tri!(self.de.parse_next(end_tag("hkparam")));
        Ok(value)
    }

    #[cold]
    fn parent_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        unreachable!("Using the wrong API: This method is not used in `havok_classes` in XML because it is for bytes.")
    }
}
