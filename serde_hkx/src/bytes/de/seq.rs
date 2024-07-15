//! Deserializing each element in an `Array`
use super::BytesDeserializer;
use crate::errors::de::{Error, Result};
use havok_serde::de::{DeserializeSeed, SeqAccess};

/// A structure for deserializing each element in an `Array`.
///
/// Since XML Array has strings to be parsed before and after parsing the value, the methods of this structure process
/// them and call `deserialize` to parse and return the internal value.
pub struct SeqDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut BytesDeserializer<'de>,

    /// Array length
    size: usize,

    /// Array length
    index: usize,
}

impl<'a, 'de> SeqDeserializer<'a, 'de> {
    /// Create a new seq deserializer
    pub fn new(de: &'a mut BytesDeserializer<'de>, size: usize) -> Self {
        Self { de, size, index: 0 }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for SeqDeserializer<'a, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&self) -> Result<usize, Self::Error> {
        if self.de.class_index == 0 {
            Err(Error::NotFoundClassPtr)
        } else {
            Ok(self.de.class_index)
        }
    }

    fn next_primitive_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.size == 0 || self.index == self.size {
            #[cfg(feature = "tracing")]
            tracing::debug!("seq end. index: {}, size: {}", self.index, self.size);
            return Ok(None);
        }

        self.index += 1;
        #[cfg(feature = "tracing")]
        tracing::debug!(self.index);

        seed.deserialize(&mut *self.de).map(Some)
    }

    #[inline]
    fn next_class_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.next_primitive_element_seed(seed)
    }

    #[inline]
    fn next_math_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.next_primitive_element_seed(seed)
    }

    #[inline]
    fn next_cstring_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.next_primitive_element_seed(seed)
    }

    #[inline]
    fn next_stringptr_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.next_primitive_element_seed(seed)
    }
}
