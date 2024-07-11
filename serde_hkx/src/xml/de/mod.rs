mod class_index_map;
mod enum_access;
mod map;
pub mod parser;
mod seq;

use crate::{lib::*, tri};

use self::class_index_map::ClassIndexMapDeserializer;
use self::enum_access::EnumDeserializer;
use self::map::MapDeserializer;
use self::parser::{
    comment_multispace0,
    tag::{attr_string, end_tag},
    type_kind::{
        boolean, matrix3, matrix4, pointer, qstransform, quaternion, real, rotation, string,
        transform, vector4,
    },
};
use self::seq::SeqDeserializer;
use crate::errors::{
    de::{Error, Result},
    readable::ReadableError,
};
use havok_serde::de::{self, Deserialize, ReadEnumSize, Visitor};
use havok_types::*;
use parser::tag::{class_start_tag, start_tag};
use winnow::ascii::{dec_int, dec_uint};
use winnow::Parser;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct XmlDeserializer<'de> {
    /// This string starts with the input data and characters are truncated off
    /// the beginning as data is parsed.
    input: &'de str,

    /// This is readonly for error report. Not move position.
    original: &'de str,

    ///  In `Struct` deserialization?
    ///
    /// # Why need this flag?
    /// This flag is necessary because XML handles deserialization of a field in a struct differently
    /// than it handles deserialization of a struct in a field in a struct.
    ///
    /// - root struct: `<hkobject name="#0050" class="" signature=""></hkobject>`
    /// - struct in field: `<hkobject></hkobject>`
    in_struct: bool,
}

impl<'de> XmlDeserializer<'de> {
    /// from xml string
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &'de str) -> Self {
        XmlDeserializer {
            input,
            original: input,
            in_struct: false,
        }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = XmlDeserializer::from_str(s);
    let t = tri!(T::deserialize(&mut deserializer));

    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters {
            remain: deserializer.input.to_string(),
        })
    }
}

pub fn from_file_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = XmlDeserializer::from_str(s);
    tri!(deserializer.parse(winnow::token::take_until(0.., "<hkobject")));
    let t = tri!(T::deserialize(&mut deserializer));

    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters {
            remain: deserializer.input.to_string(),
        })
    }
}

/// Deserializes any value and returns the rest of the string together.
///
/// # Returns
/// (remain input, deserialized value)
pub fn from_str_peek<'a, T>(s: &'a str) -> Result<(&'a str, T)>
where
    T: Deserialize<'a>,
{
    let mut deserializer = XmlDeserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok((deserializer.input, t))
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> XmlDeserializer<'de> {
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
                    self.original,
                    self.original.len() - self.input.len(),
                ),
            })?;
        Ok(res)
    }

    /// Parse by argument parser no consume.
    ///
    /// If an error occurs, it is converted to [`ReadableError`] and returned.
    fn parse_peek<O>(
        &self,
        mut parser: impl Parser<&'de str, O, winnow::error::ContextError>,
    ) -> Result<O> {
        let (_, res) = parser
            .parse_peek(self.input)
            .map_err(|err| Error::ReadableError {
                source: ReadableError::from_context(
                    err,
                    self.original,
                    self.original.len() - self.input.len(),
                ),
            })?;
        Ok(res)
    }

    /// Convert Visitor errors to position-assigned errors.
    ///
    /// # Why is this necessary?
    /// Because Visitor errors that occur within each `Deserialize` implementation cannot indicate the error location in XML.
    #[cold]
    fn to_readable_err<T>(&self, result: Result<T>) -> Result<T> {
        match result {
            Ok(value) => Ok(value),
            Err(err) => match err {
                Error::ReadableError { .. } => Err(err),
                _ => {
                    tracing::debug!("{err}");
                    Err(Error::ReadableError {
                        source: ReadableError::from_display(
                            err,
                            self.original,
                            self.original.len() - self.input.len(),
                        ),
                    })
                }
            },
        }
    }
}

// INFO:
// Where did the visit method come from?
// It creates a visit when implementing each Deserialize and reads it. The default is to return an error.
impl<'de, 'a> de::Deserializer<'de> for &'a mut XmlDeserializer<'de> {
    type Error = Error;

    /// # Note
    /// The enum implementor must parse the incoming parsed enum (or bitflags) by calling
    /// `visit_stringptr` in `impl Deserialize`.
    ///
    /// 1. Read `ANY_ENUM_VARIANT` in `<hkparam>ANY_ENUM_VARIANT</hkparam>`
    /// 2. Check by calling `visit_stringptr` .
    fn deserialize_identifier<V>(
        self,
        _size: ReadEnumSize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = tri!(self.parse(string())); // Read Until `</hkparam>`
        visitor.visit_stringptr(StringPtr::from_str(s))
    }

    // To parse field in struct (e.g. `<hkparam name="key"></hkparam>`)
    #[inline]
    fn deserialize_key<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let key = tri!(self.parse(attr_string()));

        #[cfg(feature = "tracing")]
        tracing::debug!(key);

        visitor.visit_key(key)
    }

    #[inline]
    fn deserialize_class_index<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_class_index(ClassIndexMapDeserializer::new(self))
    }

    #[inline]
    fn deserialize_void<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_void(())
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(tri!(self.parse(boolean())))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let ch = self.input.chars().next().ok_or(Error::Eof)?;
        self.input = &self.input[ch.len_utf8()..];
        visitor.visit_char(ch)
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

    #[inline]
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
        tri!(self.parse(comment_multispace0()));
        let value = visitor.visit_array(SeqDeserializer::new(self));

        // NOTE: If to_readable_err is used here, for some reason the stack overflows
        // and falls into an infinite loop.
        value
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let result = visitor.visit_enum(EnumDeserializer::new(self));
        self.to_readable_err(result)
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
        let value = if self.in_struct {
            // When a struct is present in the field of struct, the name and signature attributes are not present.
            tri!(self.parse(start_tag("hkobject")));
            tri!(visitor.visit_struct(MapDeserializer::new(self, None, fields)))
        } else {
            let (ptr_name, class_name, signature) = tri!(self.parse(class_start_tag()));
            #[cfg(feature = "tracing")]
            tracing::debug!("ptr_name={ptr_name}, class_name={class_name}, Signature={signature}");

            if name != class_name {
                return Err(Error::MismatchClassName {
                    actual: name,
                    expected: class_name.to_string(),
                });
            };
            self.in_struct = true;
            tri!(visitor.visit_struct(MapDeserializer::new(self, Some(ptr_name), fields)))
        };

        tri!(self.parse(end_tag("hkobject")));
        self.in_struct = false;
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

    fn deserialize_flags<V>(self, _size: ReadEnumSize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = tri!(self.parse(string()));
        let result = visitor.visit_stringptr(StringPtr::from_str(s));
        self.to_readable_err(result)
    }

    fn deserialize_half<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let float = tri!(self.parse(real()));
        visitor.visit_half(f16::from_f32(float))
    }

    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = tri!(self.parse(string())); // take until `</`
        visitor.visit_stringptr(StringPtr::from_str(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::mocks::{classes::*, enums::EventMode, flags::FlagValues};
    use pretty_assertions::assert_eq;

    fn parse_assert<'a, T>(s: &'a str, expected: T)
    where
        T: Deserialize<'a> + PartialEq + fmt::Debug,
    {
        match from_str::<T>(s) {
            Ok(res) => assert_eq!(res, expected),
            Err(err) => {
                tracing::error!(?err);
                panic!("{err}")
            }
        }
    }

    fn parse_peek_assert<'a, T>(s: &'a str, expected: (&str, T))
    where
        T: Deserialize<'a> + PartialEq + fmt::Debug,
    {
        match from_str_peek::<T>(s) {
            Ok(res) => assert_eq!(res, expected),
            Err(err) => {
                tracing::error!(?err);
                panic!("{err}")
            }
        }
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_primitive() {
        use havok_classes::hkClassMember_::FlagValues;
        use havok_classes::EventMode;

        parse_peek_assert(
            "ALIGN_8|ALiGN_16|SERIALIZE_IGNORED</hkparam>",
            (
                "</hkparam>",
                FlagValues::ALIGN_8 | FlagValues::ALIGN_16 | FlagValues::SERIALIZE_IGNORED,
            ),
        );

        parse_peek_assert(
            "EVENT_MODE_DEFAULT</hkparam>",
            ("</hkparam>", EventMode::EVENT_MODE_DEFAULT),
        );
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_string() {
        parse_assert::<Vec<StringPtr>>(
            r#"
    <hkcstring>Hello</hkcstring>
        <hkcstring>World</hkcstring>
    <hkcstring></hkcstring>
        "#,
            vec!["Hello".into(), "World".into(), "".into()],
        );
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_primitive_vec() {
        parse_assert(
            r#"
                <!-- Hi? -->
                <!-- Hi? -->
                true

                <!-- Hi? -->
                   false
                <!-- Hi?2 -->
        "#,
            vec![true, false],
        );

        parse_assert(
            r#"
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
    16 17 18 19 20
"#,
            (0..21).collect::<Vec<i32>>(),
        );
    }

    #[test]
    fn test_deserialize_math_vec() {
        parse_assert(
            r#"   <!-- comment -->

        (-0.000000 0.000000 -0.000000 1.000000  )
<!-- comment -->

            (   0.000000 0.000000 -0.000000 1.000000  )

                <!-- COmment -->

(   -0.000000 0.000000 -0.000000 1.000000 )
                            <!-- comment -->
"#,
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
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_primitive_array() {
        parse_assert::<[char; 0]>("", []);
    }

    #[test]
    #[quick_tracing::init]
    fn should_skip_class() {
        use havok_classes::{hkBaseObject, hkReferencedObject};
        parse_assert(
            r##"
<hkobject name="#01000" class="hkReferencedObject" signature="0xea7f1d08">
    <!-- memSizeAndFlags SERIALIZE_IGNORED -->
    <!-- referenceCount SERIALIZE_IGNORED -->
</hkobject>
            "##,
            hkReferencedObject {
                __ptr: Some(Pointer::new(1000)),
                parent: hkBaseObject { __ptr: None },
                m_memSizeAndFlags: 0,
                m_referenceCount: 0,
            },
        );
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_class() {
        use havok_classes::hkRootLevelContainer;
        // use crate::mocks::Classes;

        let xml = r###"
		<hkobject name="#0008" class="hkRootLevelContainer" signature="0x2772c11e">
			<hkparam name="namedVariants" numelements="1">
				<hkobject>
					<hkparam name="name">hkbProjectData</hkparam>
					<hkparam name="className">hkbProjectData</hkparam>
					<hkparam name="variant">#0010</hkparam>
				</hkobject>
			</hkparam>
		</hkobject>
"###;

        let res = match from_str::<hkRootLevelContainer>(xml) {
            Ok(res) => res,
            Err(err) => panic!("{err}"),
        };
        dbg!(res);
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_class_index() {
        use havok_classes::Classes;
        // use crate::mocks::Classes;

        let xml =
            &include_str!("../../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml");

        let res = match from_file_str::<Vec<Classes>>(xml) {
            Ok(res) => res,
            Err(err) => panic!("{err}"),
        };
        std::fs::write("./res.txt", format!("{res:?}")).unwrap();
    }
}
