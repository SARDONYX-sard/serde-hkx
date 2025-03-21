//! # HKX Header Format Specification
//!
//! The HKX header format is used for storing metadata information in HKX files.
//! HKX files are binary files commonly used in video game development for storing animation and physics data.
//! The header contains essential information about the structure and properties of the HKX file.
//!
//! Size: 64bytes
//!
//! | Field Name                     | Description                                                    | Size (bytes) | Offset (bytes) |
//! | ------------------------------ | -------------------------------------------------------------- | ------------ | -------------- |
//! | Magic0                         | First magic number (`0x57E0E057`)                              | 4            | 0              |
//! | Magic1                         | Second magic number (`0x10C0C010`)                             | 4            | 4              |
//! | UserTag                        | User-defined tag                                               | 4            | 8              |
//! | FileVersion                    | Version of the file (LittleEndian e.g. 0x08 0x00 0x00 0x00)    | 4            | 12             |
//! | PointerSize                    | Size of pointers in bytes (4 or 8)                             | 1            | 16             |
//! | Endian                         | Endianness of the file (0 for big-endian, 1 for little-endian) | 1            | 17             |
//! | PaddingOption                  | Padding option used in the file                                | 1            | 18             |
//! | BaseClass                      | Base class                                                     | 1            | 19             |
//! | SectionCount                   | Number of sections in the HKX file                             | 4            | 20             |
//! | ContentsSectionIndex           | Index of the contents section within the file                  | 4            | 24             |
//! | ContentsSectionOffset          | Offset of the contents section within the file                 | 4            | 28             |
//! | ContentsClassNameSectionIndex  | Index of the contents class name section within the file       | 4            | 32             |
//! | ContentsClassNameSectionOffset | Offset of the contents class name section within the file      | 4            | 36             |
//! | ContentsVersionString          | Version string of the contents (fixed-size string, 16 bytes)   | 16           | 40             |
//! | Flags                          | Various flags used in the file                                 | 4            | 56             |
//! | MaxPredicate                   | Maximum predicate value. None if -1.                           | 2            | 60             |
//! | SectionOffset                  | Section offset within the file. None if -1.                    | 2            | 62             |
//!
//! ## Paddings
//! If SectionOffset number is 16, read 64bytes header + an extra 16bytes as padding.
//!
//! | Field Name                     | Description                                                    | Size (bytes) | Offset (bytes) |
//! | ------------------------------ | -------------------------------------------------------------- | ------------ | -------------- |
//! | Unk40                          | Unknown field (Hex offset: 40)                                 | 2            | 64             |
//! | Unk42                          | Unknown field (Hex offset: 42)                                 | 2            | 66             |
//! | Unk44                          | Unknown field (Hex offset: 44)                                 | 4            | 68             |
//! | Unk48                          | Unknown field (Hex offset: 48)                                 | 4            | 72             |
//! | Unk4C                          | Unknown field (Hex offset: 4C)                                 | 4            | 76             |

use crate::{
    bytes::hexdump,
    errors::{de::Error, readable::ReadableError},
};
use winnow::{
    binary::{self, Endianness},
    combinator::{dispatch, empty, fail},
    error::{ContextError, StrContext, StrContextValue::*},
    seq,
    token::{take, take_until},
    Parser,
};

/// The 64bytes HKX header contains metadata information about the HKX file.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct HkxHeader {
    /// First magic number (`0x57E0E057`)
    pub magic0: i32,
    /// Second magic number (`0x10C0C010`)
    pub magic1: i32,
    /// User-defined tag.
    pub user_tag: i32,
    /// Version of the file.
    pub file_version: i32,
    /// Size of pointers in bytes (4 or 8)
    pub pointer_size: u8,
    /// Endianness of the file (0 for big-endian, 1 for little-endian).
    pub endian: u8,
    /// Padding option used in the file.
    pub padding_option: u8,
    /// Base class.
    pub base_class: u8,
    /// Number of sections in the HKX file.
    ///
    /// # Examples
    /// For SkyrimSE, the bytes are arranged in the following order.
    /// - `__classnames__`
    /// - `__types__`
    /// - `__data__`
    pub section_count: i32,
    /// Index of the contents section.
    pub contents_section_index: i32,
    /// Offset of the contents section.
    pub contents_section_offset: i32,
    /// Index of the contents class name section.
    pub contents_class_name_section_index: i32,
    /// Offset of the contents class name section.
    pub contents_class_name_section_offset: i32,
    /// Version string of the contents.
    ///
    /// # Bytes Example
    /// - SkyrimSE
    /// ```rust
    /// assert_eq!(
    ///   *b"hk_2010.2.0-r1\0",
    ///   [0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00]
    /// );
    /// ```
    pub contents_version_string: [u8; 15],
    /// Version string of the contents separator. Always 0xff
    pub contents_version_string_separator: u8,
    /// Various flags.
    pub flags: i32,
    /// Maximum predicate. None is -1 (== `0xFF 0xFF`)
    pub max_predicate: i16,
    /// Section offset. None is -1 (== `0xFF 0xFF`)
    ///
    /// If this number is 16, read 64bytes header plus an extra 16bytes as padding.
    pub section_offset: i16,
}

impl HkxHeader {
    /// Return Big-endian or little-endian
    ///
    /// # Note
    /// Endian must be `0(big)` or `1(little)`.
    /// - If you used the `from_bytes` constructor, it is not a problem because the endian check is already done.
    pub const fn endian(&self) -> Endianness {
        match self.endian {
            0 => Endianness::Big,
            _ => Endianness::Little,
        }
    }

    /// Create a header by parsing 64bytes from bytes.
    ///
    /// # Errors
    /// If invalid header format
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut input = bytes;
        let header = Self::parser().parse_next(&mut input).map_err(|err| {
            let hex = hexdump::to_string(bytes);
            let hex_pos = hexdump::to_hexdump_pos(input.len());
            ReadableError::from_context(err, &hex, hex_pos)
        })?;
        Ok(header)
    }

    /// Check valid endian & Parse as hkx root header.
    pub fn parser<'a>() -> impl Parser<&'a [u8], Self, ContextError> {
        move |bytes: &mut &[u8]| {
            let endianness = {
                let (mut bytes, _) = take(17_usize).parse_peek(*bytes)?;
                dispatch!(binary::u8; // 18th of bytes
                    0 => empty.value(Endianness::Big),
                    1 => empty.value(Endianness::Little),
                    _ => fail.context(StrContext::Expected(Description("Big-Endian: 0")))
                            .context(StrContext::Expected(Description("Little-Endian: 1")))
                )
                .context(StrContext::Label("Root header endianness"))
                .parse_next(&mut bytes)?
            };

            let mut verify_magic0 = binary::i32(endianness)
                .verify(|magic0| *magic0 == 0x57E0E057)
                .context(StrContext::Label("magic0"))
                .context(StrContext::Expected(StringLiteral("0x57E0E057")));
            let mut verify_magic1 = binary::i32(endianness)
                .verify(|magic0| *magic0 == 0x10C0C010)
                .context(StrContext::Label("magic1"))
                .context(StrContext::Expected(StringLiteral("0x10C0C010")));

            seq! {
                Self {
                    magic0: verify_magic0,
                    magic1: verify_magic1,
                    user_tag: binary::i32(endianness).context(StrContext::Label("user_tag"))
                        .context(StrContext::Expected(Description("i32"))),
                    file_version: binary::i32(endianness).context(StrContext::Label("file_version"))
                        .context(StrContext::Expected(Description("i32"))),
                    pointer_size: binary::u8.context(StrContext::Label("pointer_size"))
                        .context(StrContext::Expected(StringLiteral("4u8")))
                        .context(StrContext::Expected(StringLiteral("8u8"))),
                    endian: binary::u8.context(StrContext::Label("endian: u8"))
                        .context(StrContext::Expected(Description("(0u8: big)")))
                        .context(StrContext::Expected(Description("(1u8: little)"))),
                    padding_option: binary::u8.context(StrContext::Label("padding_option"))
                        .context(StrContext::Expected(Description("u8"))),
                    base_class: binary::u8.context(StrContext::Label("base_class"))
                        .context(StrContext::Expected(Description("u8"))),
                    section_count: binary::i32(endianness).context(StrContext::Label("section_count"))
                        .context(StrContext::Expected(Description("i32"))),
                    contents_section_index: binary::i32(endianness).context(StrContext::Label("contents_section_index"))
                        .context(StrContext::Expected(Description("i32"))),
                    contents_section_offset: binary::i32(endianness).context(StrContext::Label("contents_section_offset"))
                        .context(StrContext::Expected(Description("i32"))),
                    contents_class_name_section_index: binary::i32(endianness).context(StrContext::Label("contents_class_name_section_index"))
                        .context(StrContext::Expected(Description("i32"))),
                    contents_class_name_section_offset: binary::i32(endianness).context(StrContext::Label("contents_class_name_section_offset"))
                        .context(StrContext::Expected(Description("i32"))),
                    contents_version_string: take(15_usize).try_map(TryFrom::try_from).context(StrContext::Label("contents_version_string"))
                        .context(StrContext::Expected(Description("[u8; 15]"))),
                    contents_version_string_separator: 0xff.context(StrContext::Label("contents_version_string_separator"))
                        .context(StrContext::Expected(StringLiteral("0xFF"))),
                    flags: binary::i32(endianness).context(StrContext::Label("flags"))
                        .context(StrContext::Expected(Description("i32"))),
                    max_predicate: binary::i16(endianness).context(StrContext::Label("max_predicate"))
                        .context(StrContext::Expected(Description("i16"))),
                    section_offset: binary::i16(endianness).context(StrContext::Label("section_offset"))
                        .context(StrContext::Expected(Description("i16"))),
                }
            }.context(StrContext::Label("Hkx Root Header"))
            .parse_next(bytes)
        }
    }

    /// Convert to bytes.
    ///
    /// # Note
    /// If `self.endian` is 0, the data is converted to binary data as little endian, otherwise as big endian.
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut buffer = [0; 64];

        if self.endian == 0 {
            buffer[..4].copy_from_slice(&self.magic0.to_be_bytes());
            buffer[4..8].copy_from_slice(&self.magic1.to_be_bytes());
            buffer[8..12].copy_from_slice(&self.user_tag.to_be_bytes());
            buffer[12..16].copy_from_slice(&self.file_version.to_be_bytes());
            buffer[16] = self.pointer_size;
            buffer[17] = self.endian;
            buffer[18] = self.padding_option;
            buffer[19] = self.base_class;
            buffer[20..24].copy_from_slice(&self.section_count.to_be_bytes());
            buffer[24..28].copy_from_slice(&self.contents_section_index.to_be_bytes());
            buffer[28..32].copy_from_slice(&self.contents_section_offset.to_be_bytes());
            buffer[32..36].copy_from_slice(&self.contents_class_name_section_index.to_be_bytes());
            buffer[36..40].copy_from_slice(&self.contents_class_name_section_offset.to_be_bytes());
            buffer[40..55].copy_from_slice(self.contents_version_string.as_slice());
            buffer[55] = self.contents_version_string_separator;
            buffer[56..60].copy_from_slice(&self.flags.to_be_bytes());
            buffer[60..62].copy_from_slice(&self.max_predicate.to_be_bytes());
            buffer[62..64].copy_from_slice(&self.section_offset.to_be_bytes());
        } else {
            buffer[..4].copy_from_slice(&self.magic0.to_le_bytes());
            buffer[4..8].copy_from_slice(&self.magic1.to_le_bytes());
            buffer[8..12].copy_from_slice(&self.user_tag.to_le_bytes());
            buffer[12..16].copy_from_slice(&self.file_version.to_le_bytes());
            buffer[16] = self.pointer_size;
            buffer[17] = self.endian;
            buffer[18] = self.padding_option;
            buffer[19] = self.base_class;
            buffer[20..24].copy_from_slice(&self.section_count.to_le_bytes());
            buffer[24..28].copy_from_slice(&self.contents_section_index.to_le_bytes());
            buffer[28..32].copy_from_slice(&self.contents_section_offset.to_le_bytes());
            buffer[32..36].copy_from_slice(&self.contents_class_name_section_index.to_le_bytes());
            buffer[36..40].copy_from_slice(&self.contents_class_name_section_offset.to_le_bytes());
            buffer[40..55].copy_from_slice(self.contents_version_string.as_slice());
            buffer[55] = self.contents_version_string_separator;
            buffer[56..60].copy_from_slice(&self.flags.to_le_bytes());
            buffer[60..62].copy_from_slice(&self.max_predicate.to_le_bytes());
            buffer[62..64].copy_from_slice(&self.section_offset.to_le_bytes());
        }
        buffer
    }

    /// Get padding size.
    ///
    /// # Note
    /// If `Self.section_offset` is negative, 0 is returned.
    #[inline]
    pub const fn padding_size(&self) -> u32 {
        match self.section_offset {
            i16::MIN..=0 => 0,
            pad => pad as u32,
        }
    }

    /// Get `contents_version_string` as [`str`]
    ///
    /// # Errors
    /// Returns `Err` if the slice is not UTF-8.
    ///
    /// # Expected bytes examples
    /// - SkyrimSE
    /// ```rust:no_run
    /// assert_eq!(
    ///     b"hk_2010.2.0-r1\0",
    ///     [0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00].as_slice()
    /// ); // To "hk_2010.2.0-r1"
    /// ```
    pub fn contents_version_string(&self) -> winnow::ModalResult<&str> {
        let mut bytes = self.contents_version_string.as_slice();
        take_until(0.., b'\0')
            .try_map(|bytes| core::str::from_utf8(bytes))
            .parse_next(&mut bytes)
    }

    /// Create a new `HkXHeader` instance with default values for Skyrim Special Edition.
    ///
    /// # Features
    /// - file version: 8
    /// - pointer size: 8 bytes(64bit)
    /// - endian: 1(little endian)
    /// - base class: 1
    /// - section count: 3(`__classnames__`, `__type__`, `__data__`)
    /// - content section index: 2. In zero-based index, `data` section means the third section.
    /// - content class name section offset: 0x4B
    /// - max predicate: -1 (Always `0xff 0xff` in ver. hk2010)
    /// - section offset: -1 (Always `0xff 0xff` in ver. hk2010)
    pub const fn new_skyrim_se() -> Self {
        Self {
            magic0: i32::from_le_bytes([0x57, 0xE0, 0xE0, 0x57]),
            magic1: i32::from_le_bytes([0x10, 0xC0, 0xC0, 0x10]),
            user_tag: 0,
            file_version: i32::from_le_bytes([0x08, 0x00, 0x00, 0x00]),
            pointer_size: 8,
            endian: 1,
            padding_option: 0,
            base_class: 1,
            section_count: i32::from_le_bytes([0x03, 0x00, 0x00, 0x00]),
            contents_section_index: i32::from_le_bytes([0x02, 0x00, 0x00, 0x00]),
            contents_section_offset: 0,
            contents_class_name_section_index: 0,
            contents_class_name_section_offset: i32::from_le_bytes([0x4B, 0x00, 0x00, 0x00]),
            contents_version_string: *b"hk_2010.2.0-r1\0",
            contents_version_string_separator: 0xff,
            flags: 0,
            max_predicate: -1,
            section_offset: -1,
        }
    }

    /// Create a new `HkXHeader` instance with default values for Skyrim Legendary Edition.
    ///
    /// # Features
    /// Almost the same as SkyrimSE, only the `pointer_size` is different, 4 instead of 8.
    /// This means that the `pointer_size` is 32 bits(4 bytes), for a 32-bit application.
    ///
    /// - file version: 8
    /// - pointer size: 4 bytes(32bit)
    /// - endian: 1(little endian)
    /// - base class: 1
    /// - section count: 3(`__classnames__`, `__type__`, `__data__`)
    /// - content section index: 2. In zero-based index, `data` section means the third section.
    /// - content class name section offset: 0x4B
    /// - max predicate: -1 (Always `0xff 0xff` in ver. hk2010)
    /// - section offset: -1 (Always `0xff 0xff` in ver. hk2010)
    pub const fn new_skyrim_le() -> Self {
        let mut le_header = Self::new_skyrim_se();
        le_header.pointer_size = 4;
        le_header
    }
}

/// Skyrim SpecialEdition(64bit) header binary
/// ```txt
/// 0x57, 0xE0, 0xE0, 0x57, // magic0(Always 0x57, 0xE0, 0xE0, 0x57)
/// 0x10, 0xC0, 0xC0, 0x10, // magic1(Always 0x10, 0xC0, 0xC0, 0x10) 0x00,
/// 0x00, 0x00, 0x00, // user tag
/// 0x08, 0x00, 0x00, 0x00, // file version
/// 0x08, // pointer size
/// 0x01, // endian(1 is little)
/// 0x00, // padding option
/// 0x01, // base class
/// 0x03, 0x00, 0x00, 0x00, // section count
/// 0x02, 0x00, 0x00, 0x00, // contents section index
/// 0x00, 0x00, 0x00, 0x00, // content section offset
/// 0x00, 0x00, 0x00, 0x00, // contents class name section index
/// 0x4b, 0x00, 0x00, 0x00, // contents class name section offset
/// 0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00, // contents version: b"hk_2010.2.0-r1\0\0" =  ([u8;15])
/// 0xFF, // separator always 0xFF
/// 0x00, 0x00, 0x00, 0x00, // flags
/// 0xFF, 0xFF, //  max predicate: -1 as i16. This means is none.
/// 0xFF, 0xFF, // section offset: -1 as i16. This means is none.
/// ```
#[rustfmt::skip]
pub const SKYRIM_SE_ROW_HEADER: [u8; 64] = [
    0x57, 0xE0, 0xE0, 0x57, // magic0
    0x10, 0xC0, 0xC0, 0x10, // magic1
    0x00, 0x00, 0x00, 0x00, // user tag
    0x08, 0x00, 0x00, 0x00, // file version
    0x08, // pointer size
    0x01, // endian
    0x00, // padding option
    0x01, // base class
    0x03, 0x00, 0x00, 0x00, // section count
    0x02, 0x00, 0x00, 0x00, // contents section index
    0x00, 0x00, 0x00, 0x00, // content section offset
    0x00, 0x00, 0x00, 0x00, // contents class name section index
    0x4b, 0x00, 0x00, 0x00, // contents class name section offset
    // contents version: b"hk_2010.2.0-r1\0\0" + separator 0xFF =  ([u8;16])
    0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00, 0xFF,
    0x00, 0x00, 0x00, 0x00, // flags
    0xFF, 0xFF, //  max predicate: -1 as i16. This means is none.
    0xFF, 0xFF, // section offset: -1 as i16. This means is none.
];

/// Skyrim LegendaryEdition(32bit) header binary
/// ```txt
/// 0x57, 0xE0, 0xE0, 0x57, // magic0(Always 0x57, 0xE0, 0xE0, 0x57)
/// 0x10, 0xC0, 0xC0, 0x10, // magic1(Always 0x10, 0xC0, 0xC0, 0x10) 0x00,
/// 0x00, 0x00, 0x00, // user tag
/// 0x08, 0x00, 0x00, 0x00, // file version
/// 0x04, // pointer size
/// 0x01, // endian
/// 0x00, // padding option
/// 0x01, // base class
/// 0x03, 0x00, 0x00, 0x00, // section count
/// 0x02, 0x00, 0x00, 0x00, // contents section index
/// 0x00, 0x00, 0x00, 0x00, // content section offset
/// 0x00, 0x00, 0x00, 0x00, // contents class name section index
/// 0x4b, 0x00, 0x00, 0x00, // contents class name section offset
/// 0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00, // contents version: b"hk_2010.2.0-r1\0\0" =  ([u8;15])
/// 0xFF, // separator always 0xFF
/// 0x00, 0x00, 0x00, 0x00, // flags
/// 0xFF, 0xFF, //  max predicate: -1 as i16. This means is none.
/// 0xFF, 0xFF, // section offset: -1 as i16. This means is none.
/// ```
#[rustfmt::skip]
pub const SKYRIM_LE_ROW_HEADER: [u8; 64] = [
    0x57, 0xE0, 0xE0, 0x57, // magic0(Always 0x57, 0xE0, 0xE0, 0x57)
    0x10, 0xC0, 0xC0, 0x10, // magic1(Always 0x10, 0xC0, 0xC0, 0x10)
    0x00, 0x00, 0x00, 0x00, // user tag
    0x08, 0x00, 0x00, 0x00, // file version
    0x04, // pointer size
    0x01, // endian
    0x00, // padding option
    0x01, // base class
    0x03, 0x00, 0x00, 0x00, // section count
    0x02, 0x00, 0x00, 0x00, // contents section index
    0x00, 0x00, 0x00, 0x00, // content section offset
    0x00, 0x00, 0x00, 0x00, // contents class name section index
    0x4b, 0x00, 0x00, 0x00, // contents class name section offset
    // contents version string: b"hk_2010.2.0-r1\0\0" =  ([u8;15])
    0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00,
    0xFF, //  separator (always 0xFF)
    0x00, 0x00, 0x00, 0x00, // flags
    0xFF, 0xFF, //  max predicate: -1 as i16. This means is none.
    0xFF, 0xFF, // section offset: -1 as i16. This means is none.
];

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_endian_bytes() {
        assert_eq!(SKYRIM_SE_ROW_HEADER[16], 0x08); // pointer size
        assert_eq!(SKYRIM_SE_ROW_HEADER[17], 0x01); // endian
        assert_eq!(HkxHeader::new_skyrim_se().endian(), Endianness::Little);
    }

    #[test]
    fn should_read_hkx_bytes() {
        let header = HkxHeader::parser().parse(&SKYRIM_SE_ROW_HEADER).unwrap();

        assert_eq!(header, HkxHeader::new_skyrim_se());
        assert_eq!(header.padding_size(), 0); // SkyrimSE, no padding.
        assert_eq!(header.contents_version_string(), Ok("hk_2010.2.0-r1"));
    }

    #[test]
    fn should_write_hkx_bytes() {
        assert_eq!(HkxHeader::new_skyrim_se().to_bytes(), SKYRIM_SE_ROW_HEADER);
    }
}
