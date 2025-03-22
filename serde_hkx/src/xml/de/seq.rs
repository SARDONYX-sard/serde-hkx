//! Deserializing each element in an `Array`
use std::borrow::Cow;

use super::XmlDeserializer;
use super::parser::{
    comment_multispace0,
    tag::{end_tag, start_tag},
};

use crate::errors::de::{Error, Result};
use crate::tri;

use havok_serde::de::{DeserializeSeed, SeqAccess};
use winnow::combinator::alt;

/// A structure for deserializing each element in an `Array`.
///
/// Since XML Array has strings to be parsed before and after parsing the value, the methods of this structure process
/// them and call `deserialize` to parse and return the internal value.
pub struct SeqDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut XmlDeserializer<'de>,
}

impl<'a, 'de> SeqDeserializer<'a, 'de> {
    /// Create a new seq deserializer
    #[inline]
    pub fn new(de: &'a mut XmlDeserializer<'de>) -> Self {
        Self { de }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de> SeqAccess<'de> for SeqDeserializer<'_, 'de> {
    type Error = Error;

    #[inline]
    fn class_ptr(&self) -> Result<Cow<'de, str>> {
        let index = self
            .de
            .class_index
            .as_ref()
            .ok_or(Error::NotFoundClassPtr)?;
        Ok(index.clone().into_inner())
    }

    /// # Expected XML Examples
    ///
    /// - `hkArray<hkInt8>`(same as ptr, hkReal, etc...)
    /// ```xml
    /// <hkparam name="key" numelements="20">
    ///     0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
    ///     16 17 18 19 20
    /// </hkparam>
    /// ```
    fn next_primitive_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Space is required before every element except the first.
        tri!(self.de.parse_next(comment_multispace0));

        // Check if there are no more elements.
        if self.de.input.is_empty() || self.de.parse_peek(end_tag("hkparam")).is_ok() {
            return Ok(None);
        };

        seed.deserialize(&mut *self.de).map(Some) // Deserialize an array element.
    }

    fn next_class_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        // NOTE: If there is no empty confirmation in this location, the test or partial parsing will result in an infinite loop.
        if self.de.input.is_empty()
            || self
                .de
                .parse_peek(alt((end_tag("hkparam"), end_tag("hksection"))))
                .is_ok()
        {
            return Ok(None);
        };

        seed.deserialize(&mut *self.de).map(Some) // Deserialize an array element.
    }

    /// - `hkArray<Vector4>`
    /// ```xml
    /// <hkparam name="key" numelements="2">
    ///     (0.000000 0.000000 0.000000 0.000000)
    ///     (0.000000 0.000000 0.000000 0.000000)
    /// </hkparam>
    /// ```
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
        self.next_stringptr_element_seed(seed)
    }

    /// struct S {
    ///    key: StringPtr
    /// }
    ///
    /// ```xml
    /// <hkparam name="key">StringPtr1</hkparam>
    /// ```
    ///
    /// - `hkArray<hkStringPtr>`
    /// ```xml
    /// <hkparam name="key" numelements="3">
    ///     <hkcstring>StringPtr1</hkcstring>
    ///     <hkcstring>StringPtr2</hkcstring>
    ///     <hkcstring>StringPtr3</hkcstring>
    /// </hkparam>
    /// ```
    fn next_stringptr_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        // NOTE: If there is no empty confirmation in this location, the test or partial parsing will result in an infinite loop.
        if self.de.input.is_empty() || self.de.parse_peek(end_tag("hkparam")).is_ok() {
            return Ok(None);
        };

        tri!(self.de.parse_next(start_tag("hkcstring")));
        let ret = seed.deserialize(&mut *self.de).map(Some)?;
        tri!(self.de.parse_next(end_tag("hkcstring")));
        Ok(ret)
    }
}
