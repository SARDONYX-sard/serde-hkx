//! Deserializing each field element in an `Struct`
use super::BytesDeserializer;
use crate::errors::de::Error;
use havok_serde::de::{ClassIndexAccess, DeserializeSeed};

/// Order in which they are called.: `SeqAccess::next_class_element` -> `next_key` -> `next_value`
#[derive(Debug)]
pub struct BytesClassIndexMapDeserializer<'a, 'de: 'a> {
    /// Root Deserializer
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
    fn next_key(&mut self) -> Result<&'de str, Self::Error> {
        let mut start_offset = 0;

        if let Some((virtual_src, (_section_index, name_start_offset))) = &self
            .de
            .data_fixups
            .virtual_fixups
            .get_index(self.de.class_index)
        {
            // NOTE: First increment class_index(XML: `#0000`) to 1 based index notation.
            self.de.class_index += 1;
            start_offset = *name_start_offset;

            if let Some(name) = self.de.classnames.get(name_start_offset) {
                self.de.current_position =
                    (*virtual_src + self.de.data_header.absolute_data_start) as usize;

                #[cfg(feature = "tracing")]
                tracing::debug!(name, self.de.class_index);
                return Ok(*name);
            };
        };

        Err(Error::NotFoundClass {
            index: self.de.class_index,
            start_offset,
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
