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
    ptr_name: Option<Pointer>,
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    pub fn new(de: &'a mut BytesDeserializer<'de>, ptr_name: Option<Pointer>) -> Self {
        Self { de, ptr_name }
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&self) -> Option<Pointer> {
        self.ptr_name
    }

    #[inline]
    fn pad(&mut self, x86_pad: usize, x64_pad: usize) -> Result<(), Self::Error> {
        let pad = match self.de.is_x86 {
            true => x86_pad,
            false => x64_pad,
        };

        if self.de.input.len() >= pad {
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
        if self.de.field_length == self.de.field_index {
            return Ok(None);
        }

        // Now call `deserialize_key` with `impl Deserialize` and increment `self.de.field_index`.
        seed.deserialize(&mut *self.de).map(Some)
    }

    // Parse e.g. `<hkparam name="worldUpWS">`
    fn next_array_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.field_length == self.de.field_index {
            return Ok(None);
        }
        // Now call `deserialize_key` with `impl Deserialize` and increment `self.de.field_index`.
        seed.deserialize(&mut *self.de).map(Some)
    }

    // Parse e.g. `(0.000000 0.000000 1.000000 0.000000)</hkparam>`
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(tri!(seed.deserialize(&mut *self.de)))
    }
}
