//! Deserializing each field element in an `Struct`
use super::BytesDeserializer;
use crate::{errors::de::Error, tri};
use havok_serde::de::{DeserializeSeed, MapAccess};
use havok_types::Pointer;

/// A structure for deserializing each element in an `Struct`.
#[derive(Debug)]
pub struct MapDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut BytesDeserializer<'de>,

    /// Field index currently being processed
    field_index: usize,
    /// Field length currently being processed
    fields: &'static [&'static str],
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(de: &'a mut BytesDeserializer<'de>, fields: &'static [&'static str]) -> Self {
        Self {
            de,
            field_index: 0,
            fields,
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
            Some(Pointer::new(self.de.class_index))
        }
    }

    #[inline]
    fn pad(&mut self, x86_pad: usize, x64_pad: usize) -> Result<(), Self::Error> {
        let pad = if self.de.is_x86 { x86_pad } else { x64_pad };

        if pad <= self.de.input.len() {
            self.de.current_position += pad;
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }

    // Parse
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.fields.len() == self.field_index {
            return Ok(None);
        }

        let res = seed.deserialize(&mut *self.de).map(Some); // Expected to call `deserialize_key` with `impl Deserializer`
        self.field_index += 1;
        self.de.field_index = self.field_index;
        res
    }

    // Parse e.g. `(0.000000 0.000000 1.000000 0.000000)</hkparam>`
    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(tri!(seed.deserialize(&mut *self.de)))
    }
}
