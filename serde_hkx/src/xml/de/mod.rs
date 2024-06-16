pub mod parser;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{lib::*, tri};

use havok_serde::de::{self, Deserialize, DeserializeSeed, SeqAccess, Visitor};
use havok_types::*;
use winnow::ascii::{dec_int, dec_uint, multispace0, multispace1};
use winnow::error::StrContext;
use winnow::Parser;

use parser::error::ReadableError;
use parser::tag::{array_start_tag, end_tag, start_tag};
use parser::type_kind::*;

use crate::de_error::{DeError as Error, Result};

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de str,

    // This is readonly for error report. not used.
    original: &'de str,

    in_array: bool,
}

impl<'de> Deserializer<'de> {
    /// from xml string
    pub fn from_str(input: &'de str) -> Self {
        Deserializer {
            input,
            original: input,
            in_array: false,
        }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> Deserializer<'de> {
    /// Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    /// Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    /// Parse by argument parser.
    ///
    /// If an error occurs, it is converted to [`ReadableError`] and returned.
    fn try_parse<O>(
        &mut self,
        mut parser: impl Parser<&'de str, O, winnow::error::ContextError>,
    ) -> Result<O> {
        let res = parser
            .parse_next(&mut self.input)
            .map_err(|err| Error::ReadableError {
                source: ReadableError::from_context(
                    err,
                    &self.original,
                    self.original.len() - self.input.len(),
                ),
            })?;
        Ok(res)
    }

    /// Parse by argument parser no consume.
    ///
    /// If an error occurs, it is converted to [`ReadableError`] and returned.
    fn try_parse_peek<O>(
        &mut self,
        mut parser: impl Parser<&'de str, O, winnow::error::ContextError>,
    ) -> Result<O> {
        let (_, res) = parser
            .parse_peek(&self.input)
            .map_err(|err| Error::ReadableError {
                source: ReadableError::from_context(
                    err,
                    &self.original,
                    self.original.len() - self.input.len(),
                ),
            })?;
        Ok(res)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_void<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_void(())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(tri!(self.try_parse(boolean())))
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.next_char()?)
    }

    #[inline]
    fn deserialize_int8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int8(tri!(self.try_parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint8(tri!(self.try_parse(dec_uint)))
    }

    #[inline]
    fn deserialize_int16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int16(tri!(self.try_parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint16(tri!(self.try_parse(dec_uint)))
    }

    #[inline]
    fn deserialize_int32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int32(tri!(self.try_parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint32(tri!(self.try_parse(dec_uint)))
    }

    #[inline]
    fn deserialize_int64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int64(tri!(self.try_parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint64(tri!(self.try_parse(dec_uint)))
    }

    fn deserialize_real<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_real(tri!(self.try_parse(real())))
    }

    fn deserialize_vector4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_vector4(tri!(self.try_parse(vector4())))
    }

    fn deserialize_quaternion<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_quaternion(tri!(self.try_parse(quaternion())))
    }

    fn deserialize_matrix3<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_matrix3(tri!(self.try_parse(matrix3())))
    }

    fn deserialize_rotation<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_rotation(tri!(self.try_parse(rotation())))
    }

    fn deserialize_qstransform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_qstransform(tri!(self.try_parse(qstransform())))
    }

    fn deserialize_matrix4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_matrix4(tri!(self.try_parse(matrix4())))
    }

    fn deserialize_transform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_transform(tri!(self.try_parse(transform())))
    }

    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_pointer(tri!(self.try_parse(pointer())))
    }

    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (name, len) = tri!(self.try_parse(array_start_tag())); // Parse `<hkparam name="key" numelements="3">`
        #[cfg(feature = "tracing")]
        tracing::debug!(name, len);

        self.in_array = true;
        let value = tri!(visitor.visit_array(Separated::new(self))); // Give the visitor access to each element of the sequence.
        self.in_array = false;

        tri!(self.try_parse(end_tag("hkparam"))); // Parse the closing tag of the sequence.
        Ok(value)
    }

    fn deserialize_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    #[inline]
    fn deserialize_cstring<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_cstring(CString::from_str(tri!(self.try_parse(string()))))
    }

    #[inline]
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_uint64(visitor)
    }

    fn deserialize_enum_flags<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_half<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let float = tri!(self.try_parse(real()));
        visitor.visit_half(f16::from_f32(float))
    }

    #[inline]
    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = match self.in_array {
            true => tri!(self.try_parse(string_in_array())), // </hkcstring>
            false => tri!(self.try_parse(string())),         // until  </hkparam>
        };
        visitor.visit_stringptr(StringPtr::from_str(s))
    }
}

// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct Separated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> Separated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Separated { de, first: true }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for Separated<'a, 'de> {
    type Error = Error;

    fn next_primitive_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Space is required before every element except the first.
        if !self.first {
            tri!(self.de.try_parse(multispace1));
        }

        // Check if there are no more elements.
        if self.de.try_parse_peek(end_tag("hkparam")).is_ok() {
            return Ok(None);
        }
        #[cfg(feature = "tracing")]
        tracing::trace!(self.de.input);

        self.first = false;
        let value = seed.deserialize(&mut *self.de).map(Some)?; // Deserialize an array element.
        Ok(value)
    }

    fn next_class_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.input.starts_with("</hkparam>") {
            return Ok(None);
        }

        // Space is required before every element except the first.
        if !self.first && self.de.next_char()? != ' ' {
            return Err(Error::ExpectedArraySpace);
        }
        self.first = false;

        #[cfg(feature = "tracing")]
        tracing::trace!(self.de.input);

        // Deserialize an array element.
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_math_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.input.starts_with("</hkparam>") {
            return Ok(None);
        }

        // Space is required before every element except the first.
        tri!(self
            .de
            .try_parse(multispace0.context(StrContext::Label("Array math seed multispace"))));
        self.first = false;

        #[cfg(feature = "tracing")]
        tracing::trace!(self.de.input);
        seed.deserialize(&mut *self.de).map(Some) // Deserialize an array element.
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
        // Space is required before every element except the first.
        if !self.first {
            tri!(self.de.try_parse(multispace1));
        }

        // Check if there are no more elements.
        if self.de.try_parse_peek(end_tag("hkparam")).is_ok() {
            return Ok(None);
        }
        #[cfg(feature = "tracing")]
        tracing::trace!(self.de.input);

        tri!(self.de.try_parse(start_tag("hkcstring")));
        let ret = seed.deserialize(&mut *self.de).map(Some)?;
        tri!(self.de.try_parse(end_tag("hkcstring")));
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    // use crate::common::mocks::{classes::*, enums::EventMode};
    // use havok_types::*;

    fn from_str_assert<'a, T>(s: &'a str, expected: T)
    where
        T: Deserialize<'a> + PartialEq + fmt::Debug,
    {
        match from_str::<T>(s) {
            Ok(res) => assert_eq!(res, expected),
            Err(err) => match err {
                Error::ReadableError { source } => panic!("{source}"),
                _ => panic!("{err}"),
            },
        }
    }

    #[test]
    #[quick_tracing::init(test = "xml_deserialize")]
    fn test_deserialize() {
        // let xml = &include_str!("../../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml");

        from_str_assert(
            r#"
        <hkparam name="bool" numelements="2">
          true false
        </hkparam>"#,
            vec![true, false],
        );

        from_str_assert(
            r#"
<hkparam name="i32" numelements="2">
  1 2 3 4
</hkparam>
"#,
            vec![1, 2, 3, 4],
        );

        from_str_assert(
            r#"
<hkparam name="string" numelements="1">
  (0.000000 0.000000 0.000000 0.000000  )
  (   0.000000 0.000000 0.000000 0.000000  )
  (   0.000000 0.000000 0.000000 0.000000 )
</hkparam>"#,
            vec![QsTransform::default()],
        );

        from_str_assert(
            r#"
<hkparam name="string" numelements="3">
  (-0.000000 0.000000 -0.000000 1.000000  )
  (   0.000000 0.000000 -0.000000 1.000000  )
  (   -0.000000 0.000000 -0.000000 1.000000 )
</hkparam>"#,
            vec![
                Vector4 {
                    x: -0.0,
                    y: 0.0,
                    z: -0.0,
                    w: 1.0,
                },
                Vector4 {
                    x: 0.0,
                    y: 0.0,
                    z: -0.0,
                    w: 1.0,
                },
                Vector4 {
                    x: -0.0,
                    y: 0.0,
                    z: -0.0,
                    w: 1.0,
                },
            ],
        );

        from_str_assert::<Vec<StringPtr>>(
            r#"
<hkparam name="string" numelements="2">
  <hkcstring>Hello</hkcstring>
  <hkcstring>World</hkcstring>
</hkparam>"#,
            vec!["Hello".into(), "World".into()],
        );
    }
}
