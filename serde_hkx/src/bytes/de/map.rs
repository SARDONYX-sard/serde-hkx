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
    /// To check fields length
    index: usize,
    ptr_name: Option<Pointer>,
    fields: &'static [&'static str],
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(
        de: &'a mut BytesDeserializer<'de>,
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
        if self.fields.len() == self.index {
            return Ok(None);
        }

        let key = seed.deserialize(&mut *self.de).map(Some); // Deserialize a map key.
        self.index += 1;

        key
    }

    // Parse e.g. `<hkparam name="worldUpWS">`
    fn next_array_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.fields.len() == self.index {
            return Ok(None);
        }
        let key = seed.deserialize(&mut *self.de).map(Some);
        self.index += 1;

        key
    }

    // Parse e.g. `(0.000000 0.000000 1.000000 0.000000)</hkparam>`
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(tri!(seed.deserialize(&mut *self.de)))
    }
}
