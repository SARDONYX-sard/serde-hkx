//! Deserializing each field element in an `Struct`
use super::BytesDeserializer;
use crate::errors::de::Error;
use havok_serde::de::{DeserializeSeed, MapAccess};
use havok_types::Pointer;

/// A structure for deserializing each element in an `Struct`.
#[derive(Debug)]
pub struct MapDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut BytesDeserializer<'de>,

    #[allow(unused)]
    /// Field index currently being processed
    field_index: usize,
    #[allow(unused)]
    /// Field length currently being processed
    fields: &'static [&'static str],
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Create a new map deserializer
    #[inline]
    pub const fn new(de: &'a mut BytesDeserializer<'de>, fields: &'static [&'static str]) -> Self {
        Self {
            de,
            field_index: 0,
            fields,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer<'_, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&mut self) -> Option<Pointer> {
        self.de.takable_class_index.take()
    }

    fn pad(&mut self, x86_pad: usize, x64_pad: usize) -> Result<(), Self::Error> {
        let pad = if self.de.is_x86 { x86_pad } else { x64_pad };

        if pad <= self.de.input.len() {
            self.de.current_position += pad;

            #[cfg(feature = "tracing")]
            tracing::trace!(
                "padding: {pad} -> current position: {:#x}",
                self.de.current_position
            );
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
    #[cfg_attr(not(feature = "tracing"), inline)]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        #[cfg(feature = "tracing")]
        {
            if self.field_index == 0 {
                tracing::trace!("fields: {:?}", self.fields);
            };

            if let Some(field_name) = self.fields.get(self.field_index) {
                tracing::trace!(
                    "deserialize {}th field({:#x}): {field_name}",
                    self.field_index,
                    self.de.current_position
                );
            } else {
                tracing::warn!(
                    "invalid field index: {} of {}",
                    self.field_index,
                    self.fields.len()
                );
            };
            self.field_index += 1;
        }

        seed.deserialize(&mut *self.de)
    }

    /// This method is now only used for XML.
    #[inline]
    fn skip_value_seed(&mut self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    // Do the same thing as `next_value`, but separate them so that the current field name
    // to be deserialized is logged correctly.
    //
    // This is the method separation for this purpose. (For now).
    #[inline]
    fn parent_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
