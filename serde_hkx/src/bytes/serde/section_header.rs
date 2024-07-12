//! The 48bytes each HKX section header contains metadata information about the HKX file.
//!
//! This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
use crate::{lib::*, tri};

use byteorder::{ByteOrder, WriteBytesExt};
use std::io::{self, Cursor, Write as _};
use winnow::{
    binary::{self, Endianness},
    error::{ContextError, StrContext, StrContextValue::*},
    seq,
    token::take,
    Parser,
};

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
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct SectionHeader {
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
    pub absolute_data_start: u32,
    /// Offset from absolute offset to local fixup map.
    pub local_fixups_offset: u32,
    /// Offset from absolute offset to global fixup map.
    pub global_fixups_offset: u32,
    /// Offset from absolute offset to virtual class fixup map.
    pub virtual_fixups_offset: u32,

    /// Unknown offset information.
    ///
    /// Known information.
    /// - This value is the end position of the virtual_fixups_offset.
    /// - The `exports`, `imports` and `end` offsets are all the same value.
    pub exports_offset: u32,
    /// Unknown offset information.
    ///
    /// Known information.
    /// - This value is the end position of the virtual_fixups_offset.
    /// - The `exports`, `imports` and `end` offsets are all the same value.
    pub imports_offset: u32,
    /// Unknown offset information.
    ///
    /// Known information.
    /// - This value is the end position of the virtual_fixups_offset.
    /// - The `exports`, `imports` and `end` offsets are all the same value.
    pub end_offset: u32,
}
static_assertions::assert_eq_size!(SectionHeader, [u8; 48]); // Must be 48bytes.

impl SectionHeader {
    pub fn from_bytes<'a>(endian: Endianness) -> impl Parser<&'a [u8], Self, ContextError> {
        move |bytes: &mut &[u8]| {
            {
                seq! {
                    Self {
                        section_tag: take(19usize).try_map(TryFrom::try_from).context(StrContext::Label("section_tag"))
                            .context(StrContext::Expected(StringLiteral("[u8; 19]"))),
                        section_tag_separator: 0xff.context(StrContext::Label("section_tag_separator"))
                            .context(StrContext::Expected(StringLiteral("0xFF"))),
                        absolute_data_start: binary::u32(endian).context(StrContext::Label("absolute_data_start"))
                            .context(StrContext::Expected(StringLiteral("u32"))),
                        local_fixups_offset: binary::u32(endian).context(StrContext::Label("local_fixups_offset"))
                            .context(StrContext::Expected(StringLiteral("u32"))),
                        global_fixups_offset: binary::u32(endian).context(StrContext::Label("global_fixups_offset"))
                            .context(StrContext::Expected(StringLiteral("u32"))),
                        virtual_fixups_offset: binary::u32(endian).context(StrContext::Label("virtual_fixups_offset"))
                            .context(StrContext::Expected(StringLiteral("u32"))),
                        exports_offset: binary::u32(endian).context(StrContext::Label("exports_offset"))
                            .context(StrContext::Expected(StringLiteral("u32"))),
                        imports_offset: binary::u32(endian).context(StrContext::Label("imports_offset"))
                            .context(StrContext::Expected(StringLiteral("u32"))),
                        end_offset: binary::u32(endian).context(StrContext::Label("end_offset"))
                            .context(StrContext::Expected(StringLiteral("u32"))),
                    }
                }
            }
            .context(StrContext::Label("Hkx Section Header"))
            .parse_next(bytes)
        }
    }

    /// Create new `__classnames__` section header
    ///
    /// - `section_offset`: usually 0xff(ver. hk2010), this case padding is none.
    ///
    /// # Tips
    /// Here, some data values are determined by taking advantage of the fact that the section header is of fixed length size for the sake of speed.
    pub fn write_classnames<O>(
        mut writer: impl WriteBytesExt,
        section_offset: i16,
    ) -> io::Result<()>
    where
        O: ByteOrder,
    {
        writer.write_all(b"__classnames__\0\0\0\0\0\xff")?; // with separator(0xff)
        let section_offset = match section_offset {
            i16::MIN..=0_i16 => 0,
            1.. => section_offset as u32,
        };
        const ABSOLUTE_CLASSNAMES_OFFSET: u32 = 0xd0;
        writer.write_u32::<O>(ABSOLUTE_CLASSNAMES_OFFSET + section_offset)?; // write absolute_data_start

        // Fixup does not exist in `classnames` section, the same data is written.
        let fixups_offset = 0x90 + section_offset;
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
    ///
    /// # Tips
    /// Here, some data values are determined by taking advantage of the fact that the section header is of fixed length size for the sake of speed.
    pub fn write_types<O>(mut writer: impl WriteBytesExt, section_offset: i16) -> io::Result<()>
    where
        O: ByteOrder,
    {
        writer.write_all(b"__types__\0\0\0\0\0\0\0\0\0\0\xff")?; // with separator(0xff)

        ///? INFO:
        /// The fact that the header size is fixed indicates that abs is 0x160 or greater.
        /// But this is shifted in the increasing direction when section_offset is present.
        const ABSOLUTE_DATA_OFFSET: u32 = 0x160;
        let section_offset = match section_offset {
            i16::MIN..=0_i16 => 0,
            1.. => section_offset as u32,
        };
        writer.write_u32::<O>(ABSOLUTE_DATA_OFFSET + section_offset)?; // write absolute_data_start
        tri!(writer.write_all([0u8; 24].as_slice())); // Fixup does not exist in `types` section, always 0.
        Ok(())
    }

    /// Create new `__data__` section header
    ///
    /// - `section_offset`: usually 0xff(ver. hk2010), this case padding is none.
    ///
    /// # Return
    /// (Starting point where fixup_offsets, `absolute_data_start`)
    ///
    /// At this point the fixups have not yet been written correctly. (They are occupied by 0).
    ///
    /// The reason for this is that the fixups offsets is unknown at the time the `section_header` is written.
    /// so when you finish writing `__data__` section and the offset is known, use that start position to write it.
    ///
    /// # Tips
    /// Here, some data values are determined by taking advantage of the fact that the section header is of fixed length size for the sake of speed.
    pub fn write_data(writer: &mut Cursor<Vec<u8>>) -> io::Result<u64> {
        writer.write_all(b"__data__\0\0\0\0\0\0\0\0\0\0\0\xff")?; // with separator(0xff)
        let fixup_offset_start = writer.position();
        // The fixups offset in the data section is temporarily set to 0 because it will be known after the data section is written.
        writer.write_all([0u8; 28].as_slice())?;
        Ok(fixup_offset_start) // Return index for later writing.
    }
}

// To improve visualization of hex dump.
impl Display for SectionHeader {
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

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::LittleEndian;
    use std::io::Cursor;

    #[test]
    fn test_write_classnames() {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::write_classnames::<LittleEndian>(&mut buffer, 0).unwrap();
        let written = buffer.into_inner();

        #[rustfmt::skip]
        const CLASSNAMES_SECTION_HEADER: [u8; 48] = [
            // __classnames__\0\0\0\0\0: [u8; 19]
            0x5F, 0x5F, 0x63, 0x6C, 0x61, 0x73, 0x73, 0x6E, 0x61, 0x6D, 0x65, 0x73, 0x5F, 0x5F, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xff, // separator
            0xd0, 0x00, 0x00, 0x00, // absolute_data_start
            0x90, 0x00, 0x00, 0x00, // local_fixups_offset
            0x90, 0x00, 0x00, 0x00, // global_fixups_offset
            0x90, 0x00, 0x00, 0x00, // virtual_fixups_offset
            0x90, 0x00, 0x00, 0x00, // exports_offset
            0x90, 0x00, 0x00, 0x00, // imports_offset
            0x90, 0x00, 0x00, 0x00, // end_offset
        ];
        assert_eq!(&written, CLASSNAMES_SECTION_HEADER.as_slice());
    }

    #[test]
    fn test_write_types() {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::write_types::<LittleEndian>(&mut buffer, 0).unwrap();
        let written = buffer.into_inner();
        #[rustfmt::skip]
        const TYPES_SECTION_HEADER: [u8; 48] = [
            // __types__\0\0\0\0\0\0\0\0\0\0: [u8; 19]
            0x5f, 0x5f,  0x74, 0x79, 0x70, 0x65, 0x73, 0x5f, 0x5f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0xff, // separator
            // # Fixups
            // These are pending because the location cannot be determined without actually writing the data.
            0x60, 0x01, 0x00, 0x00, // absolute_data_start(0x160)
            0x00, 0x00, 0x00, 0x00, // local_fixups_offset
            0x00, 0x00, 0x00, 0x00, // global_fixups_offset
            0x00, 0x00, 0x00, 0x00, // virtual_fixups_offset
            0x00, 0x00, 0x00, 0x00, // exports_offset
            0x00, 0x00, 0x00, 0x00, // imports_offset
            0x00, 0x00, 0x00, 0x00, // end_offset
        ];
        assert_eq!(&written, TYPES_SECTION_HEADER.as_slice());
    }

    #[test]
    fn test_write_data() {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::write_data(&mut buffer).unwrap();
        let written = buffer.into_inner();

        #[rustfmt::skip]
        const DATA_SECTION_HEADER: [u8; 48] = [
            // __data__\0\0\0\0\0\0\0\0\0\0\0: [u8; 19]
            0x5f, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x5f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0xff, // separator
            // # Fixups
            // These are pending because the location cannot be determined without actually writing the data.
            0x00, 0x00, 0x00, 0x00, // absolute_data_start
            0x00, 0x00, 0x00, 0x00, // local_fixups_offset
            0x00, 0x00, 0x00, 0x00, // global_fixups_offset
            0x00, 0x00, 0x00, 0x00, // virtual_fixups_offset
            0x00, 0x00, 0x00, 0x00, // exports_offset
            0x00, 0x00, 0x00, 0x00, // imports_offset
            0x00, 0x00, 0x00, 0x00, // end_offset
        ];
        assert_eq!(&written, DATA_SECTION_HEADER.as_slice());
    }

    #[test]
    fn test_ref_from_bytes_invalid_length() {
        let written = vec![0; 40];
        let result =
            SectionHeader::from_bytes(Endianness::Little).parse_next(&mut written.as_slice());
        assert!(result.is_err());
    }

    #[test]
    fn test_ref_from_bytes_invalid_separator() {
        let mut written = vec![0; 48];
        written[19] = 0x00; // Invalid separator
        let result =
            SectionHeader::from_bytes(Endianness::Little).parse_next(&mut written.as_slice());
        assert!(result.is_err());
    }
}
