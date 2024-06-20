mod map;
pub mod parser;
mod seq;

use crate::{lib::*, tri};

use self::map::MapDeserializer;
use self::parser::error::ReadableError;
use self::parser::tag::{array_start_tag, class_start_tag, end_tag};
use self::parser::type_kind::{
    boolean, matrix3, matrix4, pointer, qstransform, quaternion, real, rotation, string,
    string_in_array, transform, vector4,
};
use self::seq::SeqDeserializer;
use crate::de_error::{DeError as Error, Result};
use havok_serde::de::{self, Deserialize, Visitor};
use havok_types::*;
use winnow::ascii::{dec_int, dec_uint};
use winnow::Parser;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct XmlDeserializer<'de> {
    /// This string starts with the input data and characters are truncated off
    /// the beginning as data is parsed.
    input: &'de str,

    /// This is readonly for error report. not used.
    original: &'de str,

    ///Flag to deal with cases where the XML notation differs between within an `Array` and without, as in `StringPtr`.
    in_array: bool,
}

impl<'de> XmlDeserializer<'de> {
    /// from xml string
    pub fn from_str(input: &'de str) -> Self {
        XmlDeserializer {
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
    let mut deserializer = XmlDeserializer::from_str(s);
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
impl<'de> XmlDeserializer<'de> {
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
    fn parse<O>(
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
    fn parse_peek<O>(
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

impl<'de, 'a> de::Deserializer<'de> for &'a mut XmlDeserializer<'de> {
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
        visitor.visit_bool(tri!(self.parse(boolean())))
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
        visitor.visit_int8(tri!(self.parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint8(tri!(self.parse(dec_uint)))
    }

    #[inline]
    fn deserialize_int16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int16(tri!(self.parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint16(tri!(self.parse(dec_uint)))
    }

    #[inline]
    fn deserialize_int32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int32(tri!(self.parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint32(tri!(self.parse(dec_uint)))
    }

    #[inline]
    fn deserialize_int64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int64(tri!(self.parse(dec_int)))
    }

    #[inline]
    fn deserialize_uint64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint64(tri!(self.parse(dec_uint)))
    }

    fn deserialize_real<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_real(tri!(self.parse(real())))
    }

    fn deserialize_vector4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_vector4(tri!(self.parse(vector4())))
    }

    fn deserialize_quaternion<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_quaternion(tri!(self.parse(quaternion())))
    }

    fn deserialize_matrix3<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_matrix3(tri!(self.parse(matrix3())))
    }

    fn deserialize_rotation<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_rotation(tri!(self.parse(rotation())))
    }

    fn deserialize_qstransform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_qstransform(tri!(self.parse(qstransform())))
    }

    fn deserialize_matrix4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_matrix4(tri!(self.parse(matrix4())))
    }

    fn deserialize_transform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_transform(tri!(self.parse(transform())))
    }

    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_pointer(tri!(self.parse(pointer())))
    }

    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // TODO: TODO: This should be parsed there with more patterns of keys in MapDeserializer
        let (name, len) = tri!(self.parse(array_start_tag())); // Parse `<hkparam name="key" numelements="3">`
        #[cfg(feature = "tracing")]
        tracing::debug!(name, len);

        self.in_array = true;
        let value = tri!(visitor.visit_array(SeqDeserializer::new(self))); // Give the visitor access to each element of the sequence.
        self.in_array = false;

        tri!(self.parse(end_tag("hkparam"))); // Parse the closing tag of the sequence.
        Ok(value)
    }

    /// # Example of XML to be parsed
    /// ```xml
    /// <hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7">
    ///   <!-- memSizeAndFlags SERIALIZE_IGNORED -->
    ///   <!-- referenceCount SERIALIZE_IGNORED -->
    ///   <hkparam name="worldUpWS">(0.000000 0.000000 1.000000 0.000000)</hkparam>
    ///   <hkparam name="stringData">#0009</hkparam>
    ///   <hkparam name="defaultEventMode">EVENT_MODE_IGNORE_FROM_GENERATOR</hkparam>
    /// </hkobject>
    /// ```
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Parse Sequence start tag.
        let (ptr_name, class_name, signature) = tri!(self.parse(class_start_tag()));
        #[cfg(feature = "tracing")]
        {
            tracing::debug!("ptr_name={ptr_name}, class_name={class_name}, Signature={signature}");
            tracing::debug!(name, ?fields);
        }
        if name != class_name {
            return Err(Error::MismatchClassName {
                actual: name,
                expected: class_name.to_string(),
            });
        };

        let value = tri!(visitor.visit_struct(MapDeserializer::new(self, fields)));

        tri!(self.parse(end_tag("hkobject")));
        Ok(value)
    }

    /// TODO: XML representation of Variant is unknown.
    fn deserialize_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_pointer(tri!(self.parse(pointer())))
    }

    #[inline]
    fn deserialize_cstring<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_cstring(CString::from_str(tri!(self.parse(string()))))
    }

    #[inline]
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_uint64(visitor)
    }

    fn deserialize_enum_flags<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_half<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let float = tri!(self.parse(real()));
        visitor.visit_half(f16::from_f32(float))
    }

    #[inline]
    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = match self.in_array {
            true => tri!(self.parse(string_in_array())), // take until `</hkcstring>`
            false => tri!(self.parse(string())),         // take until `</hkparam>`
        };
        visitor.visit_stringptr(StringPtr::from_str(s))
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
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    #[quick_tracing::init(test = "xml_deserialize")]
    fn test_deserialize() {
        // let xml = &include_str!("../../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml");
        from_str_assert(
            r##"
<hkobject name="#01000" class="hkReferencedObject" signature="0xea7f1d08">
        <hkparam name="memSizeAndFlags">2</hkparam>
        <!-- comment1 -->
        <!-- comment2 -->
        <hkparam name="referenceCount">0</hkparam>
        <!-- comment3 -->
        <!-- comment4 -->
</hkobject>"##,
            crate::common::mocks::classes::HkReferencedObject {
                _name: None,
                parent: crate::common::mocks::classes::HkBaseObject { _name: None },
                mem_size_and_flags: 1,
                reference_count: 2,
            },
        );

        from_str_assert(
            r#"
        <hkparam name="bool" numelements="2">
        <!-- Hi? -->
        <!-- Hi? -->
        <!-- Hi? -->
          true false
        <!-- Hi?2 -->
        </hkparam>"#,
            vec![true, false],
        );

        from_str_assert(
            r#"
<hkparam name="i32" numelements="2">
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
    16 17 18 19 20
</hkparam>
"#,
            (0..21).collect::<Vec<i32>>(),
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
  <hkcstring></hkcstring>
</hkparam>"#,
            vec!["Hello".into(), "World".into(), "".into()],
        );
    }
}
