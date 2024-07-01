//! Deserializing each element in an `Array`
use super::BytesDeserializer;
use crate::errors::de::{Error, Result};
use crate::tri;
use havok_serde::de::{DeserializeSeed, SeqAccess};

/// A structure for deserializing each element in an `Array`.
///
/// Since XML Array has strings to be parsed before and after parsing the value, the methods of this structure process
/// them and call `deserialize` to parse and return the internal value.
pub struct SeqDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut BytesDeserializer<'de>,
    /// Flag to determine if primitives are space-separated when parsing.
    ///
    /// Currently, this flag is not used for anything other than primitives.
    first: bool,
}

impl<'a, 'de> SeqDeserializer<'a, 'de> {
    /// Create a new seq deserializer
    pub fn new(de: &'a mut BytesDeserializer<'de>) -> Self {
        Self { de, first: true }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for SeqDeserializer<'a, 'de> {
    type Error = Error;

    fn next_primitive_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.input.is_empty() {
            return Ok(None);
        };

        let value = seed.deserialize(&mut *self.de).map(Some)?; // Deserialize an array element.
        Ok(value)
    }

    fn next_class_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.input.is_empty() {
            return Ok(None);
        };
        self.first = false;

        seed.deserialize(&mut *self.de).map(Some) // Deserialize an array element.
    }

    fn next_math_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.input.is_empty() {
            return Ok(None);
        };
        self.first = false;

        Ok(tri!(seed.deserialize(&mut *self.de).map(Some)))
    }

    #[inline]
    fn next_cstring_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.next_stringptr_element_seed(seed)
    }

    fn next_stringptr_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.input.is_empty() {
            return Ok(None);
        };
        self.first = false;

        let ret = seed.deserialize(&mut *self.de).map(Some)?;
        Ok(ret)
    }
}
