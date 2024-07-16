mod class_index_map;
mod enum_access;
mod map;
pub mod parser;
mod seq;

use crate::{lib::*, tri};

use self::class_index_map::BytesClassIndexMapDeserializer;
use self::enum_access::EnumDeserializer;
use self::map::MapDeserializer;
use self::parser::{
    classnames::{classnames_section, ClassNames},
    fixups::Fixups,
    type_kind::{
        array_meta, boolean, matrix3, matrix4, qstransform, quaternion, real, rotation, string,
        transform, vector4,
    },
    BytesStream,
};
use self::seq::SeqDeserializer;
use super::serde::{hkx_header::HkxHeader, section_header::SectionHeader};
use crate::errors::{
    de::{Error, Result},
    readable::ReadableError,
};
use havok_serde::de::{self, Deserialize, ReadEnumSize, Visitor};
use havok_types::*;
use rhexdump::hexdump;
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

    /// `__classnames__` section contents
    ///
    /// - key: class name start offset
    /// - value: class name
    classnames: ClassNames<'de>,

    /// `__classnames__` header
    classnames_header: SectionHeader,

    /// `__data__` header
    data_header: SectionHeader,
    /// data section fixups
    data_fixups: Fixups,

    /// Unique Class index & XML name attribute(e.g. `#0050`).
    ///
    /// Incremented each time deserialize_struct is called.
    /// # Note
    /// It exists to enable class_index to be retrieved at any time when seq'd as a key in a HashMap, etc.
    class_index: usize,

    /// takeable index for root class (e.g. `#0050`)
    ///
    /// # Intent that field exists.
    /// Provide an [`Option::take`] able index to prevent accidentally giving an index to a class in the structure
    /// or to the parent class of an inheritance source.
    takable_class_index: Option<Pointer>,
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
/// e.g. one class, 3 booleans, `[u32; 10]`,
///
/// # Note
/// If pointer types are included, it is impossible to deserialize correctly because fixups information is required.
pub fn from_partial_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    from_partial_bytes_with_opt(BytesDeserializer::from_bytes(bytes))
}

/// Parse binary data as the type specified in the partial generics with custom `BytesDeserializer` settings.
///
/// e.g. one class, 3 booleans, `[u32; 10]`,
///
/// # Note
/// If pointer types are included, it is impossible to deserialize correctly because fixups information is required.
pub fn from_partial_bytes_with_opt<'a, T>(de: BytesDeserializer<'a>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = de;
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input[deserializer.current_position..].is_empty() {
        Ok(t)
    } else {
        deserializer.to_readable_err(Err(Error::TrailingBytes))
    }
}

/// Analyze as binary data of one file in order from hkx header.
#[inline]
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    from_bytes_with_opt(BytesDeserializer::from_bytes(bytes))
}

/// Analyze as binary data of one file in order from hkx header(with custom deserializer settings).
pub fn from_bytes_with_opt<'a, T>(de: BytesDeserializer<'a>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = de;

    // 1. Deserialize root file header.
    let header = tri!(deserializer.parse_peek(HkxHeader::from_bytes()));
    deserializer.current_position += 64;

    deserializer.is_x86 = header.pointer_size == 4;
    deserializer.endian = header.endian();

    // 2. Deserialize the fixups in the classnames and data sections.
    tri!(deserializer.set_section_header_and_fixups(
        header.contents_class_name_section_index,
        header.contents_section_index,
        header.section_count
    ));

    // 3. Parse `__classnames__` section.
    let classnames_abs = deserializer.classnames_header.absolute_data_start as usize;
    let data_abs = deserializer.data_header.absolute_data_start as usize;
    let classnames_section_range = classnames_abs..data_abs; // FIXME: assumption that `classnames_abs` < `data_abs`
    deserializer.classnames = tri!(deserializer.parse_range(
        classnames_section(deserializer.endian, 0),
        classnames_section_range,
    ));

    // 4. Parse `__data__` section.
    deserializer.current_position = data_abs; // move to data section start
    T::deserialize(&mut deserializer)
}

/// Calculates the position in the hexdump output where the byte at the given
/// binary error position will appear.
///
/// The hexdump format for reference:
/// ```txt
/// 00000000: 4865 6c6c 6f20 576f 726c 6421 0a                  Hello World!
/// ```
/// In this format:
/// - The first 8 characters are the offset (`00000000`).
/// - The next 2 characters are a colon and a space (`: `).
/// - The next 48 characters are the hexadecimal representation of the 16 bytes of data (`4865 6c6c 6f20 576f 726c 6421 0a`).
/// - The last 16 characters are the ASCII representation of the 16 bytes of data (`Hello World!`).
///
/// Each line represents 16 bytes of the binary data.
const fn to_hexdump_pos(bytes_pos: usize) -> usize {
    const HEXDUMP_OFFSET: usize = 10;
    const BYTES_PER_LINE: usize = 16;
    const HEX_GROUP_SIZE: usize = 3;
    const ASCII_OFFSET: usize = 18;

    let line_number = bytes_pos / BYTES_PER_LINE;
    let line_offset = bytes_pos % BYTES_PER_LINE;

    HEXDUMP_OFFSET
        + (line_offset * HEX_GROUP_SIZE)
        + line_number * (HEXDUMP_OFFSET + (BYTES_PER_LINE * HEX_GROUP_SIZE) + ASCII_OFFSET)
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> BytesDeserializer<'de> {
    /// Parse by argument parser.
    ///
    /// If an error occurs, it is converted to [`ReadableError`] and returned.
    fn parse_peek<O>(
        &mut self,
        mut parser: impl Parser<BytesStream<'de>, O, winnow::error::ContextError>,
    ) -> Result<O> {
        let (_, res) = parser
            .parse_peek(&self.input[self.current_position..])
            .map_err(|err| Error::ReadableError {
                source: ReadableError::from_context(
                    err,
                    &hexdump::RhexdumpString::new().hexdump_bytes(self.input),
                    to_hexdump_pos(self.current_position),
                ),
            })?;
        Ok(res)
    }

    /// Parse by argument parser.
    ///
    /// If an error occurs, it is converted to [`ReadableError`] and returned.
    fn parse_range<O>(
        &mut self,
        mut parser: impl Parser<BytesStream<'de>, O, winnow::error::ContextError>,
        range: Range<usize>,
    ) -> Result<O> {
        let (_, res) =
            parser
                .parse_peek(&self.input[range])
                .map_err(|err| Error::ReadableError {
                    source: ReadableError::from_context(
                        err,
                        &hexdump::RhexdumpString::new().hexdump_bytes(self.input),
                        to_hexdump_pos(self.current_position),
                    ),
                })?;
        Ok(res)
    }

    /// Convert Visitor errors to position-assigned errors.
    ///
    /// # Why is this necessary?
    /// Because Visitor errors that occur within each `Deserialize` implementation cannot indicate the error location in XML.
    fn to_readable_err<T>(&self, result: Result<T>) -> Result<T> {
        match result {
            Ok(value) => Ok(value),
            Err(err) => match err {
                Error::ReadableError { .. } => Err(err),
                _ => {
                    let input = &hexdump::RhexdumpString::new().hexdump_bytes(self.input);
                    let err_pos = to_hexdump_pos(self.current_position);
                    Err(Error::ReadableError {
                        source: ReadableError::from_display(err, input, err_pos),
                    })
                }
            },
        }
    }

    /// Deserialize the fixups in the `classnames` and `data` sections, relying on the information in the root header.
    ///
    /// And, sets fixups to deserializer.
    fn set_section_header_and_fixups(
        &mut self,
        classnames_section_index: i32,
        data_section_index: i32,
        section_len: i32,
    ) -> Result<()> {
        for i in 0..section_len {
            match i {
                i if classnames_section_index == i => {
                    self.classnames_header =
                        tri!(self.parse_peek(SectionHeader::from_bytes(self.endian)));
                    #[cfg(feature = "tracing")]
                    tracing::debug!("classnames_header: {}", self.classnames_header);

                    // NOTE: no fixups
                    // The `classnames` section always has no fixups but its place is filled with abs data.
                }

                i if data_section_index == i => {
                    // 1/4: After parsing the headers, the fixups are parsed, but the position must be returned for the next header read.
                    let backup_pos = self.current_position;

                    // 2/4: read header
                    let header = tri!(self.parse_peek(SectionHeader::from_bytes(self.endian)));

                    // 3/4: read fixups
                    let fixups_start = header.absolute_data_start + header.local_fixups_offset;
                    self.current_position = fixups_start as usize;
                    self.data_fixups =
                        tri!(self.parse_peek(Fixups::from_section_header(&header, self.endian)));

                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "data_header: {header},\ndata_fixups: {:#?}",
                        self.data_fixups
                    );
                    self.data_header = header; // Let them be substituted here to avoid ownership issues.

                    // 4/4: back to header position
                    self.current_position = backup_pos;
                }
                _ => {} // Skip unused __types__ section
            };
            self.current_position += 48; // advance section header size(48bytes)
        }
        Ok(())
    }

    /// Get current position(as `global_fixup.src`) -> `global_fixup.dst` -> class index
    fn get_class_index_ptr(&mut self) -> Result<Pointer> {
        let global_fixup_src = self.relative_position();

        match self.data_fixups.global_fixups.get(&global_fixup_src) {
            Some((_section_index, global_dst)) => {
                if let Some(class_index) = self.data_fixups.virtual_fixups.get_index_of(global_dst)
                {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "global_fixup_src: {global_fixup_src}, class_index(from global_dst): {class_index}"
                    );

                    self.current_position += if self.is_x86 { 4 } else { 8 };
                    Ok(Pointer::new(class_index + 1))
                } else {
                    Err(Error::NotFoundClassIndex {
                        global_dst: *global_dst,
                    })
                }
            }
            None => {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    "Not found `global_fixup.src({global_fixup_src})` -> `global_fixup.dst`. `NullPtr` is entered instead."
                );
                self.current_position += if self.is_x86 { 4 } else { 8 };
                Ok(Pointer::new(0))
            }
        }
    }

    /// Extract the absolute position of the data position pointed to by ptr
    ///
    /// Take the relative position of the `__data__` section at the current position as a key
    /// and extract the corresponding value from the `local_fixups`.
    fn get_local_fixup_dst(&self) -> Result<usize> {
        let local_src = self.relative_position();

        #[cfg(feature = "tracing")]
        {
            let local_src_abs = self.current_position;
            tracing::debug!("local_src: {local_src}/abs({local_src_abs:#x})");
        }

        #[allow(clippy::unnecessary_lazy_evaluations)]
        let local_dst = *tri!({
            self.data_fixups
                .local_fixups
                .get(&local_src)
                .ok_or_else(|| {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("Not found `local_fixup.src({local_src})` -> `local_dst`.");
                    Error::NotFoundDataLocalFixupsValue { key: local_src }
                })
        });

        // Change to abs
        let local_dst_abs = (local_dst + self.data_header.absolute_data_start) as usize;
        #[cfg(feature = "tracing")]
        tracing::debug!("local_dst: {local_dst}/abs({local_dst_abs:#x})");
        Ok(local_dst_abs)
    }

    /// Jump current position(`local_fixup.src`) to dst, then parse, and back to current position.
    fn parse_local_fixup<O>(
        &mut self,
        parser: impl Parser<BytesStream<'de>, O, winnow::error::ContextError>,
    ) -> Result<Option<O>> {
        let backup_position = self.current_position();
        self.current_position = match self.get_local_fixup_dst().ok() {
            Some(dst) => dst,
            None => return Ok(None),
        };

        let res = tri!(self.parse_peek(parser));

        self.current_position = backup_position as usize;
        Ok(Some(res))
    }

    /// Skip ptr size
    ///
    /// # Errors
    /// Error if the value of ptr to skip is not 0.
    #[inline]
    fn skip_ptr_size(&mut self) -> Result<()> {
        if self.is_x86 {
            tri!(
                self.parse_peek(binary::u32(self.endian).verify(|uint| *uint == 0).context(
                    StrContext::Expected(StrContextValue::Description(
                        "Skip x86 ptr size(0 fill 4bytes)"
                    ))
                ))
            );
            self.current_position += 4;
        } else {
            tri!(self.parse_peek(
                binary::u64(self.endian)
                    .verify(|ulong| *ulong == 0)
                    .context(StrContext::Expected(StrContextValue::Description(
                        "Skip x64 ptr size(0 fill 8bytes)"
                    )))
            ));
            self.current_position += 8;
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

    /// Returns the relative position of the start of data_section as 0.

    /// # Intent
    /// Use this API when key of fixups requires relative position.
    #[inline]
    const fn relative_position(&self) -> u32 {
        self.current_position() - self.data_header.absolute_data_start
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

    // NOTE: This method is never used with bytes because the number of times is controlled by the for loop.
    #[cold]
    fn deserialize_key<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_void(())
    }

    // Deserialize one class.
    #[inline]
    fn deserialize_class_index<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_class_index(BytesClassIndexMapDeserializer::new(self))
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
        let res = visitor.visit_bool(tri!(self.parse_peek(boolean())));
        self.current_position += 1;
        res
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_char(tri!(self.parse_peek(
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
        let res = visitor.visit_int8(tri!(self.parse_peek(
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
        let res = visitor.visit_uint8(tri!(self.parse_peek(
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
        let res = visitor.visit_int16(tri!(self.parse_peek(
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
        let res = visitor.visit_uint16(tri!(self.parse_peek(
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
        let res = visitor.visit_int32(tri!(self.parse_peek(
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
        let res = visitor.visit_uint32(tri!(self.parse_peek(
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
        let res = visitor.visit_int64(tri!(self.parse_peek(
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
        let res = visitor.visit_uint64(tri!(self.parse_peek(
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
                .visit_real(tri!(self.parse_peek(real(self.endian).context(
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
        let res = visitor.visit_vector4(tri!(self.parse_peek(vector4(self.endian))));
        self.current_position += 16;
        res
    }

    #[inline]
    fn deserialize_quaternion<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_quaternion(tri!(self.parse_peek(quaternion(self.endian))));
        self.current_position += 16;
        res
    }

    #[inline]
    fn deserialize_matrix3<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_matrix3(tri!(self.parse_peek(matrix3(self.endian))));
        self.current_position += 48;
        res
    }

    #[inline]
    fn deserialize_rotation<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_rotation(tri!(self.parse_peek(rotation(self.endian))));
        self.current_position += 48;
        res
    }

    #[inline]
    fn deserialize_qstransform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_qstransform(tri!(self.parse_peek(qstransform(self.endian))));
        self.current_position += 48;
        res
    }

    #[inline]
    fn deserialize_matrix4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_matrix4(tri!(self.parse_peek(matrix4(self.endian))));
        self.current_position += 64;
        res
    }

    #[inline]
    fn deserialize_transform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_transform(tri!(self.parse_peek(transform(self.endian))));
        self.current_position += 64;
        res
    }

    #[inline]
    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_pointer(tri!(self.get_class_index_ptr()))
    }

    #[inline]
    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "current_position: relative({:#x})/abs({:#x})",
            self.relative_position(),
            self.current_position
        );

        // If size is 0, local_fixups does not exist, so check size first.
        // NOTE: This is a look-ahead, assuming the position does not move with this method.
        let (size, _cap_and_flags) = tri!(self.parse_peek(array_meta(self.is_x86, self.endian)));
        #[cfg(feature = "tracing")]
        tracing::debug!("in_struct array_size: {size}",);

        if size == 0 {
            self.current_position += if self.is_x86 { 12 } else { 16 };
            visitor.visit_array(SeqDeserializer::new(self, 0))
        } else {
            // The specification requires that the ptr data position be extracted before parsing meta information such as `ptr_size`.
            let pointed_data_position = tri!(self.to_readable_err(self.get_local_fixup_dst()));
            self.current_position += if self.is_x86 { 12 } else { 16 }; // NOTE: If we move position before asking for local_fixup, we will not get key correctly.
            let backup_position = self.current_position;

            self.current_position = pointed_data_position;
            let res = visitor.visit_array(SeqDeserializer::new(self, size));
            self.current_position = backup_position;
            res
        }
    }

    // Fixed size array(stack array) is written directly without metadata
    #[inline]
    fn deserialize_fixed_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // The fixed size array is controlled by a for loop, so the number of times is not controlled on the deserializer side.
        // Therefore, a dummy is plugged in.
        visitor.visit_array(SeqDeserializer::new(self, usize::MAX))
    }

    fn deserialize_class_index_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let size = self.data_fixups.virtual_fixups.len();

        #[cfg(feature = "tracing")]
        tracing::debug!("class_map_size: {size}");
        visitor.visit_array(SeqDeserializer::new(self, size))
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

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_struct_for_bytes(MapDeserializer::new(self, fields))
    }

    /// TODO: binary representation of Variant is unknown.
    #[inline]
    fn deserialize_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let res = visitor.visit_variant(Variant::new(Pointer::new(0), Pointer::new(0)));
        self.current_position += if self.is_x86 { 8 } else { 16 };
        res
    }

    fn deserialize_cstring<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = match tri!(self.parse_local_fixup(string())) {
            Some(s) => CString::from_str(s),
            None => {
                #[cfg(feature = "tracing")]
                tracing::debug!("CString is NullPtr");
                CString::from_option(None)
            }
        };
        tri!(self.skip_ptr_size());
        visitor.visit_cstring(s)
    }

    #[inline]
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_x86 {
            self.deserialize_uint32(visitor)
        } else {
            self.deserialize_uint64(visitor)
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
        let res = visitor.visit_half(tri!(self.parse_peek(parser::type_kind::half(self.endian))));
        self.current_position += 2;
        res
    }

    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = match tri!(self.parse_local_fixup(string())) {
            Some(s) => StringPtr::from_str(s),
            None => {
                #[cfg(feature = "tracing")]
                tracing::debug!("StringPtr is NullPtr");
                StringPtr::from_option(None)
            }
        };
        tri!(self.skip_ptr_size());
        visitor.visit_stringptr(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::{classes::*, enums::EventMode, flags::FlagValues};
    use pretty_assertions::assert_eq;

    fn partial_parse_assert<'a, T>(s: BytesStream<'a>, expected: T)
    where
        T: Deserialize<'a> + PartialEq + fmt::Debug,
    {
        match from_partial_bytes::<T>(s) {
            Ok(res) => assert_eq!(res, expected),
            Err(err) => {
                tracing::error!(?err);
                panic!("{err}")
            }
        }
    }

    #[test]
    fn test_deserialize_primitive() {
        partial_parse_assert(&[128, 0], FlagValues::ALIGN_8);
        partial_parse_assert(&[0], EventMode::EVENT_MODE_DEFAULT);
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_primitive_array() {
        partial_parse_assert::<[char; 0]>(b"", []);

        partial_parse_assert(&[1, 0], [true, false]);
        partial_parse_assert(
            zerocopy::AsBytes::as_bytes(&[
                0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ]),
            [
                0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ],
        );
    }

    #[test]
    fn test_deserialize_math_array() {
        let expected = [
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
        ];
        partial_parse_assert(
            zerocopy::AsBytes::as_bytes(&[
                -0.0f32, 0.0, -0.0, 1.0, // 1 vec4
                0.0, 0.0, -0.0, 1.0, // 2 vec4
                -0.0, 0.0, -0.0, 1.0, // 3 vec4
            ]),
            expected,
        );
    }

    #[test]
    #[quick_tracing::init]
    fn test_deserialize_class() {
        partial_parse_assert(
            &[
                0, 0, 0, 0, 0, 0, 0, 0, // hkBaseObject
                2, 0, // mem_size_and_flags
                0, 0, // reference_count
                0, 0, 0, 0, // 8bytes align for struct
            ],
            hkReferencedObject {
                __ptr: Some(Pointer::new(0)), // In single class partial mode, ptr is not allocated.
                parent: hkBaseObject { __ptr: None },
                m_memSizeAndFlags: 2,
                m_referenceCount: 0,
            },
        );
    }

    #[test]
    #[cfg_attr(
        feature = "tracing",
        quick_tracing::init(test = "deserialize_classes_from_bytes")
    )]
    fn test_deserialize_class_index() {
        use havok_classes::Classes;
        // use crate::mocks::Classes;

        fn from_file<'a, T>(bytes: &'a [u8]) -> Result<T>
        where
            T: Deserialize<'a>,
        {
            match from_bytes::<T>(bytes) {
                Ok(res) => Ok(res),
                Err(err) => {
                    tracing::error!("{err}");
                    panic!("{err}")
                }
            }
        }

        let bytes = include_bytes!("../../../../docs/handson_hex_dump/defaultmale/defaultmale.hkx");
        tracing::debug!("{:#?}", from_file::<Vec<Classes>>(bytes).unwrap());
        tracing::debug!(
            "{:#?}",
            from_file::<indexmap::IndexMap<usize, Classes>>(bytes).unwrap()
        );
    }
}
