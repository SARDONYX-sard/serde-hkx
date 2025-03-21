/// Fixups reader
use crate::bytes::serde::section_header::SectionHeader;
use crate::tri;
use indexmap::IndexMap;
use winnow::{
    binary::{self, Endianness},
    error::{ContextError, StrContext, StrContextValue::*},
    token::take_while,
    Parser,
};

pub type LocalFixups = IndexMap<u32, u32>;
pub type GLobalFixups = IndexMap<u32, (u32, u32)>;
/// NOTE: The order in which C++ constructors are called is fixed. Therefore, we need a Map that holds the Index.
pub type VirtualFixups = IndexMap<u32, (u32, u32)>;

/// Has fixup maps & section content bytes ref data.
///
/// Normally, the 0th `classNameOffset` of this map will contain the starting position
/// of the string `hkRootLevelContainer` in the `__classnames__` section.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Fixups {
    /// A pointer about one struct is tied to the location of its data.
    /// - Pair of the start position of `Array<T>` and the location where `T` is located.
    /// - A pair of the start position of a pointer to a `StringPtr` or `CString` and the location of the data pointed to by the pointer.
    ///
    /// # INFO
    /// - key: current reader seek position
    /// - value: dst_offset(Pointer pointed data position)
    pub local_fixups: LocalFixups,

    /// Location information needed when referencing class pointer, etc.
    ///
    /// # INFO
    /// - key: current reader seek position
    /// - value: (dst_index, dst_offset(from absolute_offset))
    pub global_fixups: GLobalFixups,

    /// Location information for the name of the C++ class that must call the constructor.
    ///
    /// # INFO
    /// - key: current reader seek position
    /// - value: (dst_index, dst_offset(from absolute_offset))
    ///
    /// # What is the `dst_index`?
    /// Index Section ID
    ///
    /// ## Examples(SkyrimSE, LE)
    /// - `__classnames__`: 1
    /// - `__types__`: 2
    /// - `__data__`: 3
    pub virtual_fixups: VirtualFixups,
}

/// src(u32), dst(u32)
pub const LOCAL_FIXUP_ONE_SIZE: u32 = 8;
/// src(u32), section_index(u32), dst(u32)
pub const GLOBAL_FIXUP_ONE_SIZE: u32 = 12;
/// src(u32), section_index(u32), classnames_start(u32)
pub const VIRTUAL_FIXUP_ONE_SIZE: u32 = 12;

/// After writing fixup, since it is 0xff up to align16, read fixup relying on this.
pub const FIXUP_VALUE_FOR_ALIGN: u32 = u32::MAX;

impl Fixups {
    pub const fn new(
        local_fixups: LocalFixups,
        global_fixups: GLobalFixups,
        virtual_fixups: VirtualFixups,
    ) -> Self {
        Self {
            local_fixups,
            global_fixups,
            virtual_fixups,
        }
    }

    /// Create a new fixups from section header & bytes.
    ///
    /// # Errors
    /// If the analysis of fixups fails.
    pub fn from_section_header<'a>(
        header: &SectionHeader,
        endian: Endianness,
    ) -> impl Parser<&'a [u8], Self, ContextError> {
        let SectionHeader {
            local_fixups_offset,
            global_fixups_offset,
            virtual_fixups_offset,
            exports_offset,
            ..
        } = *header;

        let local_range = global_fixups_offset - local_fixups_offset;
        let global_range = virtual_fixups_offset - global_fixups_offset;
        let virtual_range = exports_offset - virtual_fixups_offset;
        let _needs_bytes_len = local_range + global_range + virtual_range;

        #[cfg(feature = "tracing")]
        tracing::trace!(local_range, global_range, virtual_range);

        move |bytes: &mut &'a [u8]| {
            #[cfg(feature = "tracing")]
            if _needs_bytes_len as usize > bytes.len() {
                tracing::error!("need {_needs_bytes_len}. but got {}", bytes.len());
            }

            let local_max_len = local_range / LOCAL_FIXUP_ONE_SIZE;
            let global_max_len = global_range / GLOBAL_FIXUP_ONE_SIZE;
            let virtual_max_len = virtual_range / VIRTUAL_FIXUP_ONE_SIZE;
            Ok(Self {
                local_fixups: tri!(read_local_fixups(bytes, endian, local_max_len)),
                global_fixups: tri!(read_fixups(bytes, endian, global_max_len)),
                virtual_fixups: tri!(read_fixups(bytes, endian, virtual_max_len)),
            })
        }
    }
}

/// Create `local_fixups` from bytes.
///
/// # Note
/// And take align mark(0xff) bytes.
fn read_local_fixups(
    bytes: &mut &[u8],
    endian: Endianness,
    len: u32,
) -> winnow::ModalResult<LocalFixups> {
    let mut local_map = LocalFixups::new();
    for _ in 0..len {
        if let Ok(local_src) = binary::u32::<&[u8], ContextError>(endian)
            .verify(|&src| src != FIXUP_VALUE_FOR_ALIGN)
            .context(StrContext::Expected(Description("local_fixup.src(u32)")))
            .parse_next(bytes)
        {
            let local_dst = tri!(binary::u32(endian)
                .context(StrContext::Expected(Description("local_fixup.dst(u32)")))
                .parse_next(bytes));

            #[cfg(feature = "tracing")]
            tracing::trace!(local_src, local_dst);

            local_map.insert(local_src, local_dst);
        } else {
            break;
        };
    }
    tri!(take_while(0.., 0xff).parse_next(bytes)); // take align mark bytes.

    Ok(local_map)
}

/// Create `global_fixups` or `virtual_fixups` from bytes.
///
/// # Note
/// And take align mark(0xff) bytes.
fn read_fixups(
    bytes: &mut &[u8],
    endian: Endianness,
    len: u32,
) -> winnow::ModalResult<VirtualFixups> {
    let mut fixups = VirtualFixups::new();
    for _ in 0..len {
        if let Ok(src) = binary::u32::<&[u8], ContextError>(endian)
            .verify(|src| *src != FIXUP_VALUE_FOR_ALIGN)
            .parse_next(bytes)
        {
            #[cfg(feature = "tracing")]
            tracing::trace!(src);

            let index = tri!(binary::u32(endian)
                .context(StrContext::Expected(Description("fixup.index(u32)")))
                .parse_next(bytes));
            #[cfg(feature = "tracing")]
            tracing::trace!(index);

            let dst = tri!(binary::u32(endian)
                .context(StrContext::Expected(Description("fixup.dst(u32)")))
                .parse_next(bytes));
            #[cfg(feature = "tracing")]
            tracing::trace!(dst);

            fixups.insert(src, (index, dst));
        } else {
            break;
        }
    }
    tri!(take_while(0.., 0xff).parse_next(bytes)); // take align mark bytes.

    Ok(fixups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_fixups() {
        #[rustfmt::skip]
        const FIXUPS: &[u8] = &[
            // Local fixups
            0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, // 000002d0: ............(...
            0x18, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, // 000002e0: ....@...........
            0x00, 0x01, 0x00, 0x00, 0x08, 0x01, 0x00, 0x00, 0xd0, 0x00, 0x00, 0x00, 0x30, 0x01, 0x00, 0x00, // 000002f0: ............0...
            0xd8, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0xe0, 0x00, 0x00, 0x00, 0x50, 0x01, 0x00, 0x00, // 00000300: ....@.......P...
            0xe8, 0x00, 0x00, 0x00, 0x60, 0x01, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 00000310: ....`...........
            // Global fixups
            0x20, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, // 00000320: .......P...p...
            0x02, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 00000330: ................
            // Virtual fixups
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, // 00000340: ........K...P...
            0x00, 0x00, 0x00, 0x00, 0x65, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 00000350: ....e...........
            0x79, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 00000360: y...............
        ];

        let mut bytes = FIXUPS;

        let local = read_local_fixups(&mut bytes, Endianness::Little, 5 * 16).unwrap();
        let global = read_fixups(&mut bytes, Endianness::Little, 2 * 16).unwrap();
        let virtual_fixups = read_fixups(&mut bytes, Endianness::Little, 3 * 16).unwrap();

        assert_eq!(
            local,
            [
                (216, 320),
                (232, 352),
                (0, 16),
                (24, 64),
                (256, 264),
                (208, 304),
                (176, 256),
                (16, 40),
                (224, 336)
            ]
            .into()
        );

        assert_eq!(global, [(32, (2, 80)), (112, (2, 128))].into());
        assert_eq!(
            virtual_fixups,
            [(0, (0, 75,)), (80, (0, 101,)), (128, (0, 121))].into()
        );
    }
}
