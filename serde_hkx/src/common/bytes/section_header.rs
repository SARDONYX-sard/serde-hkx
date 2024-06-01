//! The 48bytes each HKX section header contains metadata information about the HKX file.
//!
//! This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
use std::io::{self, Cursor, Write as _};

use byteorder::WriteBytesExt;
use zerocopy::{AsBytes, ByteOrder, FromBytes, FromZeroes, LittleEndian, U32};

/// The 48bytes each HKX section header contains metadata information about the HKX file.
///
/// For SkyrimSE, the bytes are arranged in the following order.
/// - `__classnames__` 48bytes
/// - `__types__` 48bytes
/// - `__data__` 48bytes
///
/// # Note
/// This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
///
/// Depending on the havok version, there may be padding after the section header group.
/// (at least not in SkyrimSE).
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, FromBytes, AsBytes, FromZeroes)]
#[repr(C, packed)]
pub struct SectionHeader<O: ByteOrder> {
    /// Section name.
    ///
    /// For SkyrimSE, the bytes are arranged in the following order.
    /// - `__classnames__`
    /// - `__types__`
    /// - `__data__`
    ///
    /// # Bytes Example
    /// ```rust
    /// assert_eq!(
    ///   *b"__classnames__\0\0\0\0\0",
    ///   [0x5F, 0x5F, 0x63, 0x6C, 0x61, 0x73, 0x73, 0x6E, 0x61, 0x6D, 0x65, 0x73, 0x5F, 0x5F, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00]
    /// );
    /// ```
    pub section_tag: [u8; 19],
    /// Always must be `0xFF`
    pub section_tag_separator: u8,
    /// Section start & fixup base offset.
    ///
    /// # Example of position
    /// `hkx_header.section_count:3` & `hkx_header.section_offset:0` => `0x000000D0` bytes.
    ///
    /// - Calculation formula
    ///
    ///   Hkx header 64bytes + 48bytes * 3 sections = 208bytes == `0xD0`
    pub absolute_data_start: U32<O>,
    /// Offset from absolute offset to local fixup map.
    pub local_fixups_offset: U32<O>,
    /// Offset from absolute offset to global fixup map.
    pub global_fixups_offset: U32<O>,
    /// Offset from absolute offset to virtual class fixup map.
    pub virtual_fixups_offset: U32<O>,

    /// Unknown offset information.
    ///
    /// Known information.
    /// - This value is the end position of the virtual_fixups_offset.
    /// - The `exports`, `imports` and `end` offsets are all the same value.
    pub exports_offset: U32<O>,
    /// Unknown offset information.
    ///
    /// Known information.
    /// - This value is the end position of the virtual_fixups_offset.
    /// - The `exports`, `imports` and `end` offsets are all the same value.
    pub imports_offset: U32<O>,
    /// Unknown offset information.
    ///
    /// Known information.
    /// - This value is the end position of the virtual_fixups_offset.
    /// - The `exports`, `imports` and `end` offsets are all the same value.
    pub end_offset: U32<O>,
}
static_assertions::assert_eq_size!(SectionHeader<LittleEndian>, [u8; 48]); // Must be 48bytes.

impl SectionHeader<LittleEndian> {
    /// Get header length. 48(bytes)
    pub const fn len() -> usize {
        core::mem::size_of::<Self>()
    }
}

impl<O: ByteOrder> SectionHeader<O> {
    /// Interprets the given `bytes` as a `&Self` without copying.
    ///
    /// If `bytes.len() != size_of::<Self>()` or `bytes` is not aligned to
    /// `align_of::<Self>()`, this returns `Result::Err`.
    #[inline]
    pub fn ref_from_bytes(bytes: &[u8]) -> Result<&Self> {
        snafu::ensure!(
            bytes.len() >= core::mem::size_of::<Self>(),
            InsufficientLengthSnafu {
                actual: bytes.len()
            }
        );

        let ref_header = Self::ref_from_prefix(bytes).ok_or(UnAlignmentSnafu.build())?;
        let separator = ref_header.section_tag_separator; // Separator must set `0xFF`.

        snafu::ensure!(
            separator == 0xFF,
            InvalidSeparatorByteSnafu { sep: separator }
        );

        Ok(ref_header)
    }

    /// Create new `__classnames__` section header
    ///
    /// - `section_offset`: usually 0xff(ver. hk2010), this case padding is none.
    pub fn write_classnames(mut writer: impl WriteBytesExt, section_offset: i16) -> io::Result<()> {
        let section_offset = match section_offset <= 0 {
            true => 0,
            false => section_offset as u32,
        };

        writer.write(b"__classnames__\0\0\0\0\0")?;
        writer.write_u8(0xff)?; // separator
        writer.write_u32::<O>(section_offset + 0xd0)?; // absolute_data_start

        // Fixup does not exist in `classnames` section, the same data is written.
        let fixups_offset = section_offset + 0x90;
        writer.write_u32::<O>(fixups_offset)?; // local_fixups_offset
        writer.write_u32::<O>(fixups_offset)?; // global_fixups_offset
        writer.write_u32::<O>(fixups_offset)?; // virtual_fixups_offset
        writer.write_u32::<O>(fixups_offset)?; // exports_offset
        writer.write_u32::<O>(fixups_offset)?; // imports_offset
        writer.write_u32::<O>(fixups_offset) // end_offset
    }

    /// Create new `__types__` section header
    ///
    /// - `section_offset`: usually 0xff(ver. hk2010), this case padding is none.
    pub fn write_types(mut writer: impl WriteBytesExt, section_offset: i16) -> io::Result<()> {
        let section_offset = match section_offset <= 0 {
            true => 0,
            false => section_offset as u32,
        };

        writer.write(b"__types__\0\0\0\0\0\0\0\0\0\0")?;
        writer.write_u8(0xff)?; // separator
        writer.write_u32::<O>(section_offset + 0x160)?; // absolute_data_start

        // Fixup does not exist in `types` section, always 0.
        writer.write([0u8; 24].as_bytes())?;
        Ok(())
    }

    /// Create new `__types__` section header
    ///
    /// - `section_offset`: usually 0xff(ver. hk2010), this case padding is none.
    ///
    /// # Return
    /// Starting point where fixup_offsets.
    ///
    /// At this point the fixups have not yet been written correctly. (They are occupied by 0).
    ///
    /// The reason for this is that the fixups offsets is unknown at the time the `section_header` is written.
    /// so when you finish writing `__data__` section and the offset is known, use that start position to write it.
    pub fn write_data(writer: &mut Cursor<Vec<u8>>, section_offset: i16) -> io::Result<u64> {
        let section_offset = match section_offset <= 0 {
            true => 0,
            false => section_offset as u32,
        };

        writer.write(b"__data__\0\0\0\0\0\0\0\0\0\0\0")?;
        writer.write_u8(0xff)?; // separator
        writer.write_u32::<O>(section_offset + 0x160)?; // absolute_data_start

        let fixup_offset_start = writer.position();
        // The fixups offset in the data section is temporarily set to 0 because it will be known after the data section is written.
        writer.write([0u8; 24].as_bytes())?;

        Ok(fixup_offset_start) // Return index for later writing.
    }
}

// To improve visualization of hex dump.
impl<O: ByteOrder> core::fmt::Display for SectionHeader<O> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            section_tag,
            section_tag_separator,
            absolute_data_start,
            local_fixups_offset,
            global_fixups_offset,
            virtual_fixups_offset,
            exports_offset,
            imports_offset,
            end_offset,
        } = *self;

        let section_tag = core::str::from_utf8(&section_tag)
            .unwrap_or_default()
            .trim_matches(char::from(0));
        let l_offset = absolute_data_start + local_fixups_offset;
        let g_offset = absolute_data_start + global_fixups_offset;
        let v_offset = absolute_data_start + virtual_fixups_offset;
        let e_offset = absolute_data_start + exports_offset;
        let i_offset = absolute_data_start + imports_offset;
        let end_off = absolute_data_start + end_offset;

        write!(
            f,
            r#"
          section tag: {section_tag}
section tag separator: {section_tag_separator:#02X}

Offsets:
  absolute data start: {absolute_data_start:#02X}
         local fixups: {local_fixups_offset:#02X}
        global fixups: {global_fixups_offset:#02X}
       virtual fixups: {virtual_fixups_offset:#02X}
              exports: {exports_offset:#02X}
              imports; {imports_offset:#02X}
                  end: {end_offset:#02X}
        abs +   local: {l_offset:#02X}
        abs +  global: {g_offset:#02X}
        abs + virtual: {v_offset:#02X}
        abs + exports: {e_offset:#02X}
        abs + imports: {i_offset:#02X}
        abs +     end: {end_off:#02X}
"#
        )
    }
}

/// Result for [`SectionHeader`]
type Result<T, E = Error> = core::result::Result<T, E>;

/// HKX Section header Error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// Binary data is interpreted as a section header, but it was less than 48bytes.
    #[snafu(display(
        "Binary data is interpreted as a section header, but it was less than 48bytes. but got {actual}"
    ))]
    InsufficientLength {
        actual: usize,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// The next byte after section_tag (e.g. `__classnames__`) in section header should be `0xFF`, but got `{sep:#2X}` came.
    InvalidSeparatorByte {
        sep: u8,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Binary data is interpreted as a section header, but has an alignment violation.
    #[snafu(display(
        "Binary data is interpreted as a section header, but has an alignment violation."
    ))]
    UnAlignment {
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Binary data is interpreted as a section header, but has an alignment violation.
    #[snafu(transparent)]
    IoError {
        /// std I/O error.
        source: io::Error,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::LittleEndian;
    use std::io::Cursor;

    #[test]
    fn test_write_classnames() {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::<LittleEndian>::write_classnames(&mut buffer, 0).unwrap();
        let written = buffer.into_inner();
        let header = SectionHeader::<LittleEndian>::read_from_prefix(&written).unwrap();

        assert_eq!(&header.section_tag, b"__classnames__\0\0\0\0\0");
        assert_eq!(header.section_tag_separator, 0xff);
        assert_eq!(header.absolute_data_start.get(), 0xd0);
        assert_eq!(header.local_fixups_offset.get(), 0x90);
        assert_eq!(header.global_fixups_offset.get(), 0x90);
        assert_eq!(header.virtual_fixups_offset.get(), 0x90);
        assert_eq!(header.exports_offset.get(), 0x90);
        assert_eq!(header.imports_offset.get(), 0x90);
        assert_eq!(header.end_offset.get(), 0x90);
    }

    #[test]
    fn test_write_types() {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::<LittleEndian>::write_types(&mut buffer, 0).unwrap();
        let written = buffer.into_inner();
        let header = SectionHeader::<LittleEndian>::read_from_prefix(&written).unwrap();

        assert_eq!(&header.section_tag, b"__types__\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(header.section_tag_separator, 0xff);
        assert_eq!(header.absolute_data_start.get(), 0x160);
        assert_eq!(header.local_fixups_offset.get(), 0);
        assert_eq!(header.global_fixups_offset.get(), 0);
        assert_eq!(header.virtual_fixups_offset.get(), 0);
        assert_eq!(header.exports_offset.get(), 0);
        assert_eq!(header.imports_offset.get(), 0);
        assert_eq!(header.end_offset.get(), 0);
    }

    #[test]
    fn test_write_data() {
        let mut buffer = Cursor::new(Vec::new());
        let offset = SectionHeader::<LittleEndian>::write_data(&mut buffer, 0).unwrap();
        let written = buffer.into_inner();

        assert_eq!(written.len(), 48);
        assert_eq!(&written[0..19], b"__data__\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(written[19], 0xff);
        assert_eq!(offset, 24);
        assert_eq!(&written[24..48], &[0; 24]);
    }

    #[test]
    fn test_ref_from_bytes_valid() {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::<LittleEndian>::write_classnames(&mut buffer, 0).unwrap();
        let written = buffer.into_inner();

        let header = SectionHeader::<LittleEndian>::ref_from_bytes(&written).unwrap();
        assert_eq!(header.section_tag, *b"__classnames__\0\0\0\0\0");
        assert_eq!(header.section_tag_separator, 0xff);
    }

    #[test]
    fn test_ref_from_bytes_invalid_length() {
        let written = vec![0; 40];
        let result = SectionHeader::<LittleEndian>::ref_from_bytes(&written);
        assert!(matches!(result, Err(Error::InsufficientLength { .. })));
    }

    #[test]
    fn test_ref_from_bytes_invalid_separator() {
        let mut written = vec![0; 48];
        written[19] = 0x00; // Invalid separator
        let result = SectionHeader::<LittleEndian>::ref_from_bytes(&written);
        assert!(matches!(result, Err(Error::InvalidSeparatorByte { .. })));
    }
}
