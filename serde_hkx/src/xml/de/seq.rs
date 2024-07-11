//! Deserializing each element in an `Array`
use super::parser::tag::{end_tag, start_tag};
use super::XmlDeserializer;

use crate::errors::de::{Error, Result};
use crate::tri;
use crate::xml::de::parser::{comment_multispace0, comment_multispace1};

use havok_serde::de::{DeserializeSeed, SeqAccess};
use winnow::combinator::alt;
use winnow::error::{StrContext, StrContextValue};
use winnow::Parser;

/// A structure for deserializing each element in an `Array`.
///
/// Since XML Array has strings to be parsed before and after parsing the value, the methods of this structure process
/// them and call `deserialize` to parse and return the internal value.
pub struct SeqDeserializer<'a, 'de: 'a> {
    /// Deserializer
    de: &'a mut XmlDeserializer<'de>,
    /// Flag to determine if primitives are space-separated when parsing.
    ///
    /// Currently, this flag is not used for anything other than primitives.
    first: bool,
}

impl<'a, 'de> SeqDeserializer<'a, 'de> {
    /// Create a new seq deserializer
    pub fn new(de: &'a mut XmlDeserializer<'de>) -> Self {
        Self { de, first: true }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for SeqDeserializer<'a, 'de> {
    type Error = Error;

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
        // Check if there are no more elements.
        if self.de.parse_peek(end_tag("hkparam")).is_ok() {
            return Ok(None);
        };

        // Space is required before every element except the first.
        if !self.first {
            tri!(self.de.parse(
                comment_multispace1()
                    .context(StrContext::Expected(StrContextValue::CharLiteral(' ')))
            ));
        };
        self.first = false;

        // NOTE:
        // If there is no value after leaving whitespace and it is the end, it is necessary
        // to check here to avoid an infinite loop.
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
        // NOTE: If there is no empty confirmation in this location, the test or partial parsing will result in an infinite loop.
        if self.de.input.is_empty()
            || self
                .de
                .parse_peek(alt((end_tag("hkparam"), end_tag("hksection"))))
                .is_ok()
        {
            tracing::debug!(self.de.input);
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
        // NOTE: If there is no empty confirmation in this location, the test or partial parsing will result in an infinite loop.
        if self.de.input.is_empty() || self.de.parse_peek(end_tag("hkparam")).is_ok() {
            return Ok(None);
        };
        self.first = false;

        let value = tri!(seed.deserialize(&mut *self.de).map(Some));
        tri!(self.de.parse(comment_multispace0()));
        Ok(value)
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
        self.first = false;

        tri!(self.de.parse(start_tag("hkcstring")));
        let ret = seed.deserialize(&mut *self.de).map(Some)?;
        tri!(self.de.parse(end_tag("hkcstring")));
        Ok(ret)
    }
}
