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
use zerocopy::{AsBytes, ByteOrder, FromBytes, FromZeroes, LittleEndian, I16, I32};

/// The 64bytes HKX header contains metadata information about the HKX file.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, FromBytes, AsBytes, FromZeroes)]
#[repr(C, packed)]
pub struct HkxHeader<O: ByteOrder> {
    /// First magic number (`0x57E0E057`)
    pub magic0: I32<O>,
    /// Second magic number (`0x10C0C010`)
    pub magic1: I32<O>,
    /// User-defined tag.
    pub user_tag: I32<O>,
    /// Version of the file.
    pub file_version: I32<O>,
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
    pub section_count: I32<O>,
    /// Index of the contents section.
    pub contents_section_index: I32<O>,
    /// Offset of the contents section.
    pub contents_section_offset: I32<O>,
    /// Index of the contents class name section.
    pub contents_class_name_section_index: I32<O>,
    /// Offset of the contents class name section.
    pub contents_class_name_section_offset: I32<O>,
    /// Version string of the contents. + separator(0xFF)
    ///
    /// # Bytes Example
    /// - SkyrimSE
    /// ```rust
    /// assert_eq!(
    ///   *b"hk_2010.2.0-r1\0\xFF",
    ///   [0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00, 0xFF]
    /// );
    /// ```
    pub contents_version_string: [u8; 16],
    /// Various flags.
    pub flags: I32<O>,
    /// Maximum predicate. None is -1 (== `0xFF 0xFF`)
    pub max_predicate: I16<O>,
    /// Section offset. None is -1 (== `0xFF 0xFF`)
    ///
    /// If this number is 16, read 64bytes header plus an extra 16bytes as padding.
    pub section_offset: I16<O>,
}

static_assertions::assert_eq_size!(HkxHeader<LittleEndian>, [u8; 64]); // Must be 64 bytes.

impl HkxHeader<LittleEndian> {
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
    /// - max predicate: -1 (This mean is None)
    /// - section offset: -1 (This mean is None)
    pub const fn new_skyrim_se() -> Self {
        Self {
            magic0: I32::from_bytes([0x57, 0xE0, 0xE0, 0x57]),
            magic1: I32::from_bytes([0x10, 0xC0, 0xC0, 0x10]),
            user_tag: I32::ZERO,
            file_version: I32::from_bytes([0x08, 0x00, 0x00, 0x00]),
            pointer_size: 8,
            endian: 1,
            padding_option: 0,
            base_class: 1,
            section_count: I32::from_bytes([0x03, 0x00, 0x00, 0x00]),
            contents_section_index: I32::from_bytes([0x02, 0x00, 0x00, 0x00]),
            contents_section_offset: I32::ZERO,
            contents_class_name_section_index: I32::ZERO,
            contents_class_name_section_offset: I32::from_bytes([0x4B, 0x00, 0x00, 0x00]),
            contents_version_string: *b"hk_2010.2.0-r1\0\xFF",
            flags: I32::ZERO,
            max_predicate: I16::from_bytes([0xFF, 0xFF]),
            section_offset: I16::from_bytes([0xFF, 0xFF]),
        }
    }

    /// Almost the same as SkyrimSE, only the ptr size is different, 4 instead of 8.
    ///
    /// This means that the ptr size is 32 bits, or 4 bytes, for a 32-bit application.
    pub const fn new_skyrim_le() -> Self {
        let mut le_header = Self::new_skyrim_se();
        le_header.pointer_size = 4;
        le_header
    }

    /// Get pointer size of this hkx file from header information.
    ///
    /// # Assumptions
    /// Passed argument bytes are first hkx header bytes.
    ///
    /// # Panics
    /// - If `bytes` < 17(bytes)
    pub const fn ptr_size(bytes: &[u8]) -> u8 {
        bytes[16]
    }

    /// Is the binary in the Hkx file big-endian?
    ///
    /// # Assumptions
    /// Passed argument bytes are first hkx header bytes.
    ///
    /// # Panics
    /// - If `bytes` < 18(bytes)
    pub const fn is_big_endian(bytes: &[u8]) -> bool {
        bytes[17] == 0
    }

    /// Get header length. 64(bytes)
    pub const fn len() -> usize {
        core::mem::size_of::<Self>()
    }
}

impl<O: ByteOrder> HkxHeader<O> {
    /// Interprets the given `bytes` as a `&Self` without copying.
    ///
    /// If `bytes.len() != size_of::<Self>()` or `bytes` is not aligned to
    /// `align_of::<Self>()`, this returns `Result::Err`.
    #[inline]
    pub fn ref_from_bytes(bytes: &[u8]) -> Result<&Self> {
        if bytes.len() < core::mem::size_of::<Self>() {
            return InsufficientLengthSnafu.fail();
        }
        Self::ref_from_prefix(bytes).ok_or(UnAlignmentSnafu.build())
    }

    /// Get padding size.
    ///
    /// # Note
    /// If `Self.section_offset` is negative, 0 is returned.
    pub fn padding_size(&self) -> usize {
        let padding = self.section_offset.get();
        if padding < 0 {
            0
        } else {
            padding as usize
        }
    }

    /// Get version string of the contents that trimmed null str and separator(0xFF).
    ///
    /// # Errors
    ///
    /// Returns `Err` if the slice is not UTF-8.
    ///
    /// # Value Examples
    /// - SkyrimSE
    /// ```rust:no_run
    /// "hk_2010.2.0-r1";
    /// [0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31];
    /// ```
    pub fn contents_version_string_as_str(&self) -> Result<&str> {
        let end_position = self
            .contents_version_string
            .iter()
            .position(|c| *c == 0 || *c == 0xFF) // Search null str or separator byte.
            .unwrap_or(self.contents_version_string.len() - 1); // If not present, all.

        Ok(core::str::from_utf8(
            &self.contents_version_string[..end_position],
        )?)
    }
}

/// Hkx header Error Result
type Result<T, E = Error> = core::result::Result<T, E>;
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// Binary data is interpreted as a header, but it was less than 64bytes.
    #[snafu(display("Binary data is interpreted as a header, but it was less than 64bytes."))]
    InsufficientLength {
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Binary data is interpreted as a header, but has an alignment violation.
    #[snafu(display("Binary data is interpreted as a header, but has an alignment violation."))]
    UnAlignment {
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(transparent)]
    Utf8Error {
        source: core::str::Utf8Error,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[rustfmt::skip]
    const SKYRIM_SE_ROW_HEADER: [u8; 64] = [
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

    #[test]
    fn should_parse_endian_bytes() {
        assert_eq!(SKYRIM_SE_ROW_HEADER[16], 0x08); // pointer size
        assert_eq!(SKYRIM_SE_ROW_HEADER[17], 0x01); // endian
        assert_eq!(HkxHeader::is_big_endian(&SKYRIM_SE_ROW_HEADER), false);
    }

    #[test]
    fn should_read_hkx_bytes() {
        let header = HkxHeader::ref_from_bytes(&SKYRIM_SE_ROW_HEADER).unwrap();
        assert_eq!(header, &HkxHeader::new_skyrim_se());

        assert_eq!(header.padding_size(), 0); // SkyrimSE, no padding.

        let content_ver_str = header.contents_version_string_as_str().unwrap();
        assert_eq!(content_ver_str, "hk_2010.2.0-r1");
    }

    #[test]
    fn should_write_hkx_bytes() {
        assert_eq!(HkxHeader::new_skyrim_se().as_bytes(), &SKYRIM_SE_ROW_HEADER);
    }
}
