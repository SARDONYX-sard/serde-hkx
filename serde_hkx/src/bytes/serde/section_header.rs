//! The 48bytes each HKX section header contains metadata information about the HKX file.
//!
//! This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
use crate::{lib::*, tri};

use byteorder::{ByteOrder, WriteBytesExt};
use std::io;
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
    /// ```
    /// assert_eq!(
    ///   [0x5F, 0x5F, 0x63, 0x6C, 0x61, 0x73, 0x73, 0x6E, 0x61, 0x6D, 0x65, 0x73, 0x5F, 0x5F, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   *b"__classnames__\0\0\0\0\0"
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
    ///   Hkx header 64bytes + 48bytes * 3 sections = 208bytes == `0xD0`(`__classnames__` section abs)
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
    /// `*b"__data__\0\0\0\0\0\0\0\0\0\0\0"`
    pub const DATA_SECTION_HEADER_TAG: [u8; 19] = *b"__data__\0\0\0\0\0\0\0\0\0\0\0";

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

    /// Write section header to writer.
    pub fn write_bytes<O>(&self, mut writer: impl WriteBytesExt) -> io::Result<()>
    where
        O: ByteOrder,
    {
        writer.write_all(&self.section_tag)?;
        writer.write_u8(self.section_tag_separator)?;
        writer.write_u32::<O>(self.absolute_data_start)?;
        writer.write_u32::<O>(self.local_fixups_offset)?;
        writer.write_u32::<O>(self.global_fixups_offset)?;
        writer.write_u32::<O>(self.virtual_fixups_offset)?;
        writer.write_u32::<O>(self.exports_offset)?;
        writer.write_u32::<O>(self.imports_offset)?;
        writer.write_u32::<O>(self.end_offset)?;
        Ok(())
    }

    /// Create new `__classnames__` section header
    ///
    /// - `section_offset`: usually 0xff(ver. hk2010), this case padding is none.
    pub fn write_classnames<O>(
        mut writer: impl WriteBytesExt,
        section_offset: i16,
        section_end_abs: u32,
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

        let fixups_offset = section_end_abs - 0xd0;
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
    /// # Return
    /// `absolute_data_start`
    ///
    /// # Why do you return the write position of `absolute_data_offset`?
    /// The `absolute_data_offset` is found (basically, depending on the header settings) just before the `__data__` section begins.
    ///
    /// It is not yet known at this point when the types section header is written, so the position is returned so that it can be written later.
    pub fn write_types<O>(mut writer: impl WriteBytesExt, abs_offset: u32) -> io::Result<()>
    where
        O: ByteOrder,
    {
        writer.write_all(b"__types__\0\0\0\0\0\0\0\0\0\0\xff")?; // with separator(0xff)
        writer.write_u32::<O>(abs_offset)?; // same as `__data__` section's `absolute_data_offset`
        tri!(writer.write_all([0u8; 24].as_slice())); // Fixup does not exist in `types` section, always 0.
        Ok(())
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
section tag separator: {section_tag_separator:#02x}

Offsets:
  absolute data start: {absolute_data_start:#02x}
         local fixups: {local_fixups_offset:#02x}
        global fixups: {global_fixups_offset:#02x}
       virtual fixups: {virtual_fixups_offset:#02x}
              exports: {exports_offset:#02x}
              imports; {imports_offset:#02x}
                  end: {end_offset:#02x}
        abs +   local: {l_offset:#02x}
        abs +  global: {g_offset:#02x}
        abs + virtual: {v_offset:#02x}
        abs + exports: {e_offset:#02x}
        abs + imports: {i_offset:#02x}
        abs +     end: {end_off:#02x}
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
    fn test_write_classnames() -> io::Result<()> {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::write_classnames::<LittleEndian>(&mut buffer, 0, 0x160)?;
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
        Ok(())
    }

    #[test]
    fn test_write_types() -> io::Result<()> {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader::write_types::<LittleEndian>(&mut buffer, 0x160)?;
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
        Ok(())
    }

    #[test]
    fn test_write_data() -> io::Result<()> {
        let mut buffer = Cursor::new(Vec::new());
        SectionHeader {
            section_tag: SectionHeader::DATA_SECTION_HEADER_TAG,
            section_tag_separator: 0xff,
            absolute_data_start: 0x160,
            local_fixups_offset: 0x170,
            global_fixups_offset: 0x1C0,
            virtual_fixups_offset: 0x1E0,
            exports_offset: 0x210,
            imports_offset: 0x210,
            end_offset: 0x210,
        }
        .write_bytes::<LittleEndian>(&mut buffer)?;
        let written = buffer.into_inner();

        #[rustfmt::skip]
        const DATA_SECTION_HEADER: [u8; 48] = [
            // __data__\0\0\0\0\0\0\0\0\0\0\0: [u8; 19]
            0x5f, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x5f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0xff, // separator
            // # Fixups
            // These are pending because the location cannot be determined without actually writing the data.
            0x60, 0x01, 0x00, 0x00, // absolute_data_start
            0x70, 0x01, 0x00, 0x00, // local_fixups_offset
            0xC0, 0x01, 0x00, 0x00, // global_fixups_offset
            0xE0, 0x01, 0x00, 0x00, // virtual_fixups_offset
            0x10, 0x02, 0x00, 0x00, // exports_offset
            0x10, 0x02, 0x00, 0x00, // imports_offset
            0x10, 0x02, 0x00, 0x00, // end_offset
        ];
        assert_eq!(&written, DATA_SECTION_HEADER.as_slice());
        Ok(())
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
