//! Deserializing each field element in an `Struct`
use super::BytesDeserializer;
use crate::errors::de::Error;
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
pub struct BytesClassIndexMapDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut BytesDeserializer<'de>,
}

impl<'a, 'de> BytesClassIndexMapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(de: &'a mut BytesDeserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> ClassIndexAccess<'de> for BytesClassIndexMapDeserializer<'a, 'de> {
    type Error = Error;

    // Call constructor by class name
    fn next_key(&self) -> Result<&'de str, Self::Error> {
        let mut class_name = None;
        let mut start_offset = 0;

        // TODO: create new class name(virtual_fixup.src, class_name)
        for (_section_index, name_start_offset) in self.de.data_fixups.virtual_fixups.values() {
            if let Some(name) = self.de.classnames.get(name_start_offset) {
                class_name = Some(*name);
                start_offset = *name_start_offset;
                break;
            };
        }

        class_name.ok_or(Error::Message {
            msg: format!("Couldn't find class by this name_offset: {start_offset}"),
        })
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
