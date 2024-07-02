mod enum_access;
mod map;
pub mod parser;
mod seq;

use crate::{lib::*, tri};

use self::enum_access::EnumDeserializer;
use self::map::MapDeserializer;
use self::parser::type_kind::{
    boolean, c_str, matrix3, matrix4, qstransform, quaternion, real, rotation, transform, vector4,
};
use self::seq::SeqDeserializer;
use crate::errors::{
    de::{Error, Result},
    readable::ReadableError,
};
use havok_serde::de::{self, Deserialize, ReadEnumSize, Visitor};
use havok_types::*;
use parser::BytesStream;
use rhexdump::hexdump;
use winnow::binary::Endianness;
use winnow::error::{StrContext, StrContextValue};
use winnow::{binary, Parser};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct BytesDeserializer<'de> {
    /// This string starts with the input data and bytes are truncated off the beginning as data is parsed.
    input: &'de [u8],

    /// This is readonly for error report. Not move position.
    original: &'de [u8],

    endian: Endianness,
    is_x86: bool,

    /// Unique Class index & XML name attribute(e.g. `#0050`).
    ///
    /// Incremented each time deserialize_struct is called.
    class_index: usize,
    /// Field index currently being processed
    field_index: Option<usize>,
    /// Field length currently being processed
    field_length: Option<usize>,
}

impl<'de> BytesDeserializer<'de> {
    /// from xml string
    pub fn from_bytes(input: &'de [u8]) -> Self {
        BytesDeserializer {
            input,
            original: input,
            endian: Endianness::Little,
            is_x86: false,
            class_index: 0,
            field_index: None,
            field_length: None,
        }
    }
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = BytesDeserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingBytes {
            remain: deserializer.input.to_owned(),
        })
    }
}

/// 00000000: 48 65
const HEXDUMP_OFFSET: usize = 10;

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> BytesDeserializer<'de> {
    /// Parse by argument parser.
    ///
    /// If an error occurs, it is converted to [`ReadableError`] and returned.
    fn parse<O>(
        &mut self,
        mut parser: impl Parser<BytesStream<'de>, O, winnow::error::ContextError>,
    ) -> Result<O> {
        let res = parser
            .parse_next(&mut self.input)
            .map_err(|err| Error::ReadableError {
                source: ReadableError::from_context(
                    err,
                    &hexdump::RhexdumpString::new().hexdump_bytes(self.original),
                    self.original.len() - self.input.len() + HEXDUMP_OFFSET,
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
                _ => Err(Error::ReadableError {
                    source: ReadableError::from_display(
                        err,
                        &hexdump::RhexdumpString::new().hexdump_bytes(self.original),
                        self.original.len() - self.input.len() + HEXDUMP_OFFSET,
                    ),
                }),
            },
        }
    }
}

// INFO:
// Where did the visit method come from?
// It creates a visit when implementing each Deserialize and reads it. The default is to return an error.
impl<'de, 'a> de::Deserializer<'de> for &'a mut BytesDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_identifier<V>(
        self,
        size: ReadEnumSize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_flags(size, visitor)
    }

    /// When deserializing structs in order, this is called by the `Deserialize` implementation on the each struct side,
    /// so this calls `visit_u64` on the each struct side.
    #[inline]
    fn deserialize_key<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let (Some(index), Some(length)) = (self.field_index, self.field_index) {
            let res = visitor.visit_uint64(index as u64);
            tracing::debug!(index);
            self.field_index = Some(index + 1);
            if length < index {
                Err(Error::OverFlowIndex {
                    expected: length,
                    actual: index,
                })
            } else {
                res
            }
        } else {
            Err(Error::NotFoundIndex)
        }
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

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(tri!(self.parse(
            binary::le_u8.context(StrContext::Expected(StrContextValue::Description("char")))
        )) as char)
    }

    #[inline]
    fn deserialize_int8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int8(tri!(self.parse(
            binary::le_i8.context(StrContext::Expected(StrContextValue::Description("i8")))
        )))
    }

    #[inline]
    fn deserialize_uint8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint8(tri!(self.parse(
            binary::le_u8.context(StrContext::Expected(StrContextValue::Description("u8")))
        )))
    }

    #[inline]
    fn deserialize_int16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int16(tri!(self.parse(
            binary::i16(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("i16")))
        )))
    }

    #[inline]
    fn deserialize_uint16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint16(tri!(self.parse(
            binary::u16(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("u16")))
        )))
    }

    #[inline]
    fn deserialize_int32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int32(tri!(self.parse(
            binary::i32(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("i32")))
        )))
    }

    #[inline]
    fn deserialize_uint32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint32(tri!(self.parse(
            binary::u32(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("u32")))
        )))
    }

    #[inline]
    fn deserialize_int64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_int64(tri!(self.parse(
            binary::i64(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("i64")))
        )))
    }

    #[inline]
    fn deserialize_uint64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_uint64(tri!(self.parse(
            binary::u64(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("u64")))
        )))
    }

    #[inline]
    fn deserialize_real<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_real(tri!(self.parse(
            real(self.endian).context(StrContext::Expected(StrContextValue::Description("f32")))
        )))
    }

    fn deserialize_vector4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_vector4(tri!(self.parse(vector4(self.endian))))
    }

    fn deserialize_quaternion<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_quaternion(tri!(self.parse(quaternion(self.endian))))
    }

    fn deserialize_matrix3<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_matrix3(tri!(self.parse(matrix3(self.endian))))
    }

    fn deserialize_rotation<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_rotation(tri!(self.parse(rotation(self.endian))))
    }

    fn deserialize_qstransform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_qstransform(tri!(self.parse(qstransform(self.endian))))
    }

    fn deserialize_matrix4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_matrix4(tri!(self.parse(matrix4(self.endian))))
    }

    fn deserialize_transform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_transform(tri!(self.parse(transform(self.endian))))
    }

    #[inline]
    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_pointer(Pointer::new(0)) // TODO: get from global fixups
    }

    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_array(SeqDeserializer::new(self))
        // NOTE: If to_readable_err is used here, for some reason the stack overflows and falls into an infinite loop.
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

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.class_index += 1;
        self.field_index = Some(0);
        self.field_length = Some(fields.len());

        let value = tri!(visitor.visit_struct(MapDeserializer::new(
            self,
            Some(Pointer::new(self.class_index)),
        )));
        self.field_index = None;
        self.field_length = None;

        Ok(value)
    }

    /// TODO: binary representation of Variant is unknown.
    #[inline]
    fn deserialize_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // TODO: get from global fixups
        visitor.visit_variant(Variant::new(Pointer::new(0), Pointer::new(0)))
    }

    #[inline]
    fn deserialize_cstring<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let c_str = tri!(c_str(&mut self.input)).to_string_lossy();
        visitor.visit_cstring(CString::from_option(Some(c_str)))
    }

    #[inline]
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_uint64(visitor)
    }

    fn deserialize_flags<V>(self, size: ReadEnumSize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let result = match size {
            ReadEnumSize::Int8 => self.deserialize_int8(visitor),
            ReadEnumSize::Int16 => self.deserialize_int16(visitor),
            ReadEnumSize::Int32 => self.deserialize_int32(visitor),
            ReadEnumSize::Int64 => self.deserialize_int64(visitor),
            ReadEnumSize::Uint8 => self.deserialize_uint8(visitor),
            ReadEnumSize::Uint16 => self.deserialize_uint16(visitor),
            ReadEnumSize::Uint32 => self.deserialize_uint32(visitor),
            ReadEnumSize::Uint64 => self.deserialize_uint64(visitor),
        };
        self.to_readable_err(result)
    }

    fn deserialize_half<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let float = tri!(self.parse(real(self.endian)));
        visitor.visit_half(f16::from_f32(float))
    }

    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let c_str = tri!(c_str(&mut self.input)).to_string_lossy();
        visitor.visit_stringptr(StringPtr::from_option(Some(c_str)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::mocks::{enums::EventMode, flags::FlagValues};
    use pretty_assertions::assert_eq;

    fn parse_assert<'a, T>(s: BytesStream<'a>, expected: T)
    where
        T: Deserialize<'a> + PartialEq + fmt::Debug,
    {
        match from_bytes::<T>(s) {
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
        parse_assert(&[128, 0], FlagValues::ALIGN_8);
        parse_assert(&[0, 0], EventMode::EventModeDefault);
    }

    #[test]
    fn test_deserialize_string() {
        parse_assert::<Vec<StringPtr>>(
            b"Hello\0World\0\0",
            vec!["Hello".into(), "World".into(), "".into()],
        );
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_primitive_vec() {
        parse_assert(&[1, 0], vec![true, false]);

        parse_assert(
            zerocopy::AsBytes::as_bytes(&[
                0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ]),
            (0..21).collect::<Vec<i32>>(),
        );
    }

    #[test]
    fn test_deserialize_math_vec() {
        parse_assert(
            zerocopy::AsBytes::as_bytes(&[
                -0.0f32, 0.0, -0.0, 1.0, // 1 vec4
                0.0, 0.0, -0.0, 1.0, // 2 vec4
                -0.0, 0.0, -0.0, 1.0, // 3 vec4
            ]),
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
        parse_assert::<[char; 0]>(b"", []);
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_class() {
        parse_assert(
            &[
                0, 0, 0, 0, 0, 0, 0, 0, // hkBaseObject
                2, 0, // mem_size_and_flags
                0, 0, // reference_count
                0, 0, 0, 0, 0, 0, 0, 0, // 8bytes align for struct
            ],
            crate::common::mocks::classes::HkReferencedObject {
                __ptr_name_attr: Some(Pointer::new(1)),
                parent: crate::common::mocks::classes::HkBaseObject { _name: None },
                mem_size_and_flags: 2,
                reference_count: 0,
            },
        );
    }
}
