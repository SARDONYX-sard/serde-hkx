//! The 48bytes each HKX section header contains metadata information about the HKX file.
//!
//! This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
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
    pub exports_offset: U32<O>,
    pub imports_offset: U32<O>,
    pub end_offset: U32<O>,
}

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
            bytes.len() < core::mem::size_of::<Self>(),
            InsufficientLengthSnafu
        );

        let ref_header = Self::ref_from_prefix(bytes).ok_or(UnAlignmentSnafu.build())?;
        let separator = ref_header.section_tag_separator; // Separator must set `0xFF`.

        snafu::ensure!(
            separator != 0xFF,
            InvalidSeparatorByteSnafu { sep: separator }
        );

        Ok(ref_header)
    }
}

// Must be 48bytes.
static_assertions::assert_eq_size!(SectionHeader<LittleEndian>, [u8; 48]);

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
type Result<T, E = SectionHeaderError> = core::result::Result<T, E>;

/// HKX Section header Error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum SectionHeaderError {
    /// Binary data is interpreted as a section header, but it was less than 48bytes.
    #[snafu(display(
        "Binary data is interpreted as a section header, but it was less than 48bytes."
    ))]
    InsufficientLength {
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
}
