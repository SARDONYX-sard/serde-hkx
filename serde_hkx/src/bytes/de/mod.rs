mod enum_access;
mod map;
pub mod parser;
mod seq;

use crate::{lib::*, tri};

use self::enum_access::EnumDeserializer;
use self::map::MapDeserializer;
use self::parser::type_kind::{
    boolean, matrix3, matrix4, qstransform, quaternion, real, rotation, string, transform, vector4,
};
use self::parser::{classnames::classnames_section, fixups::Fixups, BytesStream};
use self::seq::SeqDeserializer;
use super::serde::{hkx_header::HkxHeader, section_header::SectionHeader};
use crate::errors::{
    de::{Error, Result},
    readable::ReadableError,
};
use havok_serde::de::{self, Deserialize, ReadEnumSize, Visitor};
use havok_types::*;
use rhexdump::hexdump;
use std::collections::HashMap;
use winnow::binary::Endianness;
use winnow::error::{StrContext, StrContextValue};
use winnow::{binary, Parser};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, educe::Educe)]
#[educe(Default)]
pub struct BytesDeserializer<'de> {
    /// This is readonly bytes data.
    input: &'de [u8],

    /// Binary data position currently being read
    current_position: usize,

    /// Big or Little Endian
    #[educe(Default = Endianness::Little)]
    endian: Endianness,
    /// Is this binary data for 32-bit?
    ///
    /// # Note
    /// This is related to the read size of the pointer type and the skip size of the padding.
    is_x86: bool,

    /// - key: virtual_src
    /// - value: unique class index(e.g. XML name attribute `#0050`)
    class_index_map: HashMap<u32, usize>,

    /// `__classnames__` header
    classnames_header: SectionHeader,
    /// classnames section fixups
    classnames_fixups: Fixups,

    /// `__data__` header
    data_header: SectionHeader,
    /// data section fixups
    data_fixups: Fixups,

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
        Self {
            input,
            ..Default::default()
        }
    }
}

/// Parse binary data as the type specified in the partial generics.
///
/// e.g. one class, 3 booleans, [u32; 10],
///
/// # Note
/// If pointer types are included, it is impossible to deserialize correctly because fixups information is required.
pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = BytesDeserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input[deserializer.current_position..].is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingBytes {
            remain: deserializer.input[deserializer.current_position..].to_owned(),
        })
    }
}

/// Analyze as binary data of one file in order from hkx header.
pub fn from_bytes_file<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = BytesDeserializer::from_bytes(bytes);

    // 1. Deserialize root file header.
    let header = tri!(deserializer.parse(HkxHeader::from_bytes()));
    deserializer.is_x86 = header.pointer_size == 4;
    deserializer.endian = header.endian();

    // 2. Deserialize the fixups in the classnames and data sections.
    tri!(deserializer.set_fixups(
        header.contents_class_name_section_index,
        header.contents_section_index,
        header.section_count
    ));

    // 3. Parse classnames section.
    deserializer.current_position = deserializer.classnames_header.absolute_data_start as usize; // move to classnames section start
    let classnames = tri!(deserializer.parse(classnames_section(
        deserializer.endian,
        deserializer.current_position
    )));

    // 4. Call constructor by class name
    for (_section_index, name_start_offset) in deserializer.data_fixups.virtual_fixups.values() {
        let class_name = tri!(classnames.get(name_start_offset).ok_or(Error::Message {
            msg: format!("Couldn't find class by this name_offset: {name_start_offset}"),
        }));

        let _ = class_name;
        todo!("deserialize classes")
    }

    // 5. class field parse.
    let t = tri!(T::deserialize(&mut deserializer));
    if deserializer.input[deserializer.current_position..].is_empty() {
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
        let (_, res) = parser
            .parse_peek(&self.input[self.current_position..])
            .map_err(|err| Error::ReadableError {
                source: ReadableError::from_context(
                    err,
                    &hexdump::RhexdumpString::new().hexdump_bytes(self.input),
                    self.current_position + HEXDUMP_OFFSET,
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
                Error::Message { msg } => Err(Error::ReadableError {
                    source: ReadableError::from_display(
                        msg,
                        &hexdump::RhexdumpString::new().hexdump_bytes(self.input),
                        self.current_position + HEXDUMP_OFFSET,
                    ),
                }),
                _ => Err(Error::ReadableError {
                    source: ReadableError::from_display(
                        err,
                        &hexdump::RhexdumpString::new().hexdump_bytes(self.input),
                        self.current_position + HEXDUMP_OFFSET,
                    ),
                }),
            },
        }
    }

    /// Deserialize the fixups in the `classnames` and `data` sections, relying on the information in the root header.
    ///
    /// And, sets fixups to deserializer.
    fn set_fixups(
        &mut self,
        classnames_section_index: i32,
        data_section_index: i32,
        section_len: i32,
    ) -> Result<()> {
        let backup_pos = self.current_position;
        for i in 0..section_len {
            match i {
                i if classnames_section_index == i => {
                    let header = tri!(self.parse(SectionHeader::from_bytes(self.endian)));
                    let fixups_start_pos = header.absolute_data_start + header.local_fixups_offset;

                    self.current_position = fixups_start_pos as usize;
                    self.classnames_fixups =
                        tri!(self.parse(Fixups::from_section_heder(&header, self.endian)));
                    self.classnames_header = header;
                }

                i if data_section_index == i => {
                    let header = tri!(self.parse(SectionHeader::from_bytes(self.endian)));
                    let fixups_start = header.absolute_data_start + header.local_fixups_offset;

                    self.current_position = fixups_start as usize;
                    self.data_fixups =
                        tri!(self.parse(Fixups::from_section_heder(&header, self.endian)));
                    self.data_header = header;
                }
                _ => {} // Skip unused __types__ section
            }
        }
        self.current_position = backup_pos;
        Ok(())
    }

    /// Skip until align N.
    #[allow(unused)]
    fn skip_align(&mut self, n: usize) -> Result<()> {
        let alignment = (n - (self.current_position % n)) % n;
        if alignment > 0 && alignment <= self.input.len() {
            self.input = &self.input[alignment..];
            Ok(())
        } else {
            Err(Error::Eof)
        }
    }

    /// Skip ptr size
    ///
    /// # Errors
    /// Error if the value of ptr to skip is not 0.
    #[inline]
    fn skip_ptr_size(&mut self) -> Result<()> {
        match self.is_x86 {
            true => {
                tri!(
                    self.parse(binary::u32(self.endian).verify(|uint| *uint == 0).context(
                        StrContext::Expected(StrContextValue::Description(
                            "Skip x86 ptr size(0 fill 4bytes)"
                        ))
                    ))
                );
                self.current_position += 4;
            }
            false => {
                tri!(self.parse(
                    binary::u64(self.endian)
                        .verify(|ulong| *ulong == 0)
                        .context(StrContext::Expected(StrContextValue::Description(
                            "Skip x64 ptr size(0 fill 8bytes)"
                        )))
                ));

                self.current_position += 8;
            }
        };
        Ok(())
    }

    /// Get current bytes position.
    ///
    /// # Note
    /// This returns [`u32`] to be used as a key to retrieve the data position from the `fixups` that fixes
    /// the data position pointed to by the pointer type.
    #[inline]
    const fn current_position(&self) -> u32 {
        self.current_position as u32
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

    /// This is used to determine which field in struct to deserialize.
    ///
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
        let res = visitor.visit_bool(tri!(self.parse(boolean())));
        self.current_position += 1;
        res
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_char(tri!(self.parse(
            binary::le_u8.context(StrContext::Expected(StrContextValue::Description("char")))
        )) as char);
        self.current_position += 1;
        res
    }

    #[inline]
    fn deserialize_int8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_int8(tri!(self.parse(
            binary::le_i8.context(StrContext::Expected(StrContextValue::Description("i8")))
        )));
        self.current_position += 1;
        res
    }

    #[inline]
    fn deserialize_uint8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_uint8(tri!(self.parse(
            binary::le_u8.context(StrContext::Expected(StrContextValue::Description("u8")))
        )));
        self.current_position += 1;
        res
    }

    #[inline]
    fn deserialize_int16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_int16(tri!(self.parse(
            binary::i16(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("i16")))
        )));
        self.current_position += 2;
        res
    }

    #[inline]
    fn deserialize_uint16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_uint16(tri!(self.parse(
            binary::u16(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("u16")))
        )));
        self.current_position += 2;
        res
    }

    #[inline]
    fn deserialize_int32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_int32(tri!(self.parse(
            binary::i32(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("i32")))
        )));
        self.current_position += 4;
        res
    }

    #[inline]
    fn deserialize_uint32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_uint32(tri!(self.parse(
            binary::u32(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("u32")))
        )));
        self.current_position += 4;
        res
    }

    #[inline]
    fn deserialize_int64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_int64(tri!(self.parse(
            binary::i64(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("i64")))
        )));
        self.current_position += 8;
        res
    }

    #[inline]
    fn deserialize_uint64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_uint64(tri!(self.parse(
            binary::u64(self.endian)
                .context(StrContext::Expected(StrContextValue::Description("u64")))
        )));
        self.current_position += 8;
        res
    }

    #[inline]
    fn deserialize_real<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res =
            visitor
                .visit_real(tri!(self.parse(real(self.endian).context(
                    StrContext::Expected(StrContextValue::Description("f32"))
                ))));
        self.current_position += 4;
        res
    }

    #[inline]
    fn deserialize_vector4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_vector4(tri!(self.parse(vector4(self.endian))));
        self.current_position += 16;
        res
    }

    #[inline]
    fn deserialize_quaternion<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_quaternion(tri!(self.parse(quaternion(self.endian))));
        self.current_position += 16;
        res
    }

    #[inline]
    fn deserialize_matrix3<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_matrix3(tri!(self.parse(matrix3(self.endian))));
        self.current_position += 48;
        res
    }

    #[inline]
    fn deserialize_rotation<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_rotation(tri!(self.parse(rotation(self.endian))));
        self.current_position += 48;
        res
    }

    #[inline]
    fn deserialize_qstransform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_qstransform(tri!(self.parse(qstransform(self.endian))));
        self.current_position += 48;
        res
    }

    #[inline]
    fn deserialize_matrix4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_matrix4(tri!(self.parse(matrix4(self.endian))));
        self.current_position += 64;
        res
    }

    #[inline]
    fn deserialize_transform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_transform(tri!(self.parse(transform(self.endian))));
        self.current_position += 64;
        res
    }

    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let current_position = self.current_position();
        match self.data_fixups.global_fixups.get(&current_position) {
            Some((_section_index, virtual_src)) => match self.class_index_map.get(virtual_src) {
                Some(index) => visitor.visit_pointer(Pointer::new(*index)),
                None => Err(Error::NotFoundClassIndex {
                    virtual_src: *virtual_src,
                }),
            },
            None => Err(Error::NotFoundDataGlobalFixupsValue {
                key: current_position,
            }),
        }
    }

    #[inline]
    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_array(SeqDeserializer::new(self))
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

        dbg!(self.current_position());
        self.class_index_map
            .insert(self.current_position(), self.class_index);

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
        let res = visitor.visit_variant(Variant::new(Pointer::new(0), Pointer::new(0)));
        if self.is_x86 {
            self.current_position += 8;
        } else {
            self.current_position += 16;
        };
        res
    }

    fn deserialize_cstring<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let local_src = self.current_position();

        self.current_position = *tri!(self
            .data_fixups
            .local_fixups
            .get(&local_src)
            .ok_or(Error::NotFoundDataLocalFixupsValue { key: local_src }))
            as usize;

        let s = tri!(self.parse(string()));

        self.current_position = local_src as usize;
        tri!(self.skip_ptr_size());

        visitor.visit_cstring(CString::from_str(s))
    }

    #[inline]
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.is_x86 {
            true => self.deserialize_uint32(visitor),
            false => self.deserialize_uint64(visitor),
        }
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

    #[inline]
    fn deserialize_half<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_half(tri!(self.parse(parser::type_kind::half(self.endian))));
        self.current_position += 2;
        res
    }

    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let local_src = self.current_position();

        self.current_position = *tri!(self
            .data_fixups
            .local_fixups
            .get(&local_src)
            .ok_or(Error::NotFoundDataLocalFixupsValue { key: local_src }))
            as usize;

        let s = tri!(self.parse(string()));

        self.current_position = local_src as usize;
        tri!(self.skip_ptr_size());

        visitor.visit_stringptr(StringPtr::from_str(s))
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
        parse_assert(&[0], EventMode::EventModeDefault);
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_primitive_array() {
        parse_assert::<[char; 0]>(b"", []);

        parse_assert(&[1, 0], [true, false]);
        parse_assert(
            zerocopy::AsBytes::as_bytes(&[
                0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ]),
            [
                0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ],
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
            [
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
