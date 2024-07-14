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
    ptr_name: Option<Pointer>,

    #[allow(unused)]
    /// Field index currently being processed
    field_index: usize,
    #[allow(unused)]
    /// Field index currently being processed
    /// Field length currently being processed
    fields: &'static [&'static str],

    fields_ptr: usize,
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
            field_index: 0,
            fields,
            ptr_name,
            fields_ptr: fields.as_ptr() as usize,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&mut self) -> Option<Pointer> {
        self.ptr_name.take()
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

    // NOTE: This method is never used with bytes because the number of times is controlled by the for loop.
    #[cold]
    fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        unimplemented!("Use only `next_value` for Deserialize Bytes.")
    }

    // Parse field
    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        #[cfg(feature = "tracing")]
        {
            if self.fields_ptr != self.fields.as_ptr() as usize {
                self.field_index = 0;
            };

            if let Some(field_name) = self.fields.get(self.field_index) {
                tracing::trace!(
                    "current deserialize field: {field_name} of {:?}",
                    self.fields
                )
            } else {
                tracing::warn!(
                    "invalid field index: {} of {}",
                    self.field_index,
                    self.fields.len()
                )
            };
            self.field_index += 1;
        }

        Ok(tri!(seed.deserialize(&mut *self.de)))
    }
}
