//! Each section fixup map reader/writer
use super::header::SectionHeader;
use core::mem::size_of;
use indexmap::IndexMap;
use std::io;
use zerocopy::{ByteOrder, U32};

trait ReadFixup {
    /// Reads `core::mem::size_of::<Self>` size fixup data from the argument byte slice.
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> io::Result<Self>
    where
        Self: Sized;
}

trait WriteFixup {
    /// Writes `core::mem::size_of::<Self>` size fixup data to the argument byte slice.
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> io::Result<()>;
}

/// Map linking the byte position of the current reader/writer with the byte position of the data
/// pointed to by `hkArray`, `hkStringPtr`.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalFixup {
    /// Current reader/writer position.
    pub src: u32,
    /// The byte position of the data to which the pointer points. (position relative to `absolute_data_offset`)
    pub dst: u32,
}

impl ReadFixup for LocalFixup {
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> io::Result<Self> {
        let src = B::read_u32(bytes);
        let dst = B::read_u32(&bytes[4..]);

        Ok(LocalFixup { src, dst })
    }
}

impl WriteFixup for LocalFixup {
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> io::Result<()> {
        B::write_u32(bytes, self.src);
        B::write_u32(&mut bytes[4..], self.dst);

        Ok(())
    }
}

/// Location information needed when referencing class pointer, etc.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalFixup {
    /// Current reader seek position
    pub src: u32,
    ///  Index Section ID
    ///
    /// # Examples
    /// - `__class_names__`: 0
    /// - `__type__`: 1
    /// - `__data__`: 2
    pub dst_section_index: u32,
    /// Location information needed when referencing class pointer, etc.
    ///
    /// # Examples
    /// - When global map dst is 128
    ///
    /// `128` =>
    /// If `virtualFixup.src == 128` => `virtualFixup.class_name_offset` =>
    /// `class_names[class_name_offset]` => class name
    pub dst: u32,
}

impl ReadFixup for GlobalFixup {
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> io::Result<Self> {
        let src = B::read_u32(bytes);
        let dst_section_index = B::read_u32(&bytes[4..]);
        let dst = B::read_u32(&bytes[8..]);

        Ok(GlobalFixup {
            src,
            dst_section_index,
            dst,
        })
    }
}

impl WriteFixup for GlobalFixup {
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> io::Result<()> {
        B::write_u32(bytes, self.src);
        B::write_u32(&mut bytes[4..], self.dst_section_index);
        B::write_u32(&mut bytes[8..], self.dst);

        Ok(())
    }
}

/// Location information for the name of the C++ class that must call the constructor
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualFixup {
    /// Current reader/writer seek position.
    ///
    /// It is partially the same as the value in `globalFixup.dst`.
    pub src: u32,
    /// Index Section ID
    ///
    /// # Examples
    /// Note that the following may change with the header section settings.
    /// - `__class_names__`: 0
    /// - `__type__`: 1
    /// - `__data__`: 2
    pub section_index: u32,
    /// Havok Class name start position.
    ///
    /// # How do we use this?
    /// `ClassNames[name_offset]` => class name
    pub name_offset: u32,
}

impl ReadFixup for VirtualFixup {
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> io::Result<Self> {
        let src = B::read_u32(bytes);
        let dst_section_index = B::read_u32(&bytes[4..]);
        let dst = B::read_u32(&bytes[8..]);

        Ok(VirtualFixup {
            src,
            section_index: dst_section_index,
            name_offset: dst,
        })
    }
}

impl WriteFixup for VirtualFixup {
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()> {
        B::write_u32(bytes, self.src);
        B::write_u32(&mut bytes[4..], self.section_index);
        B::write_u32(&mut bytes[8..], self.name_offset);

        Ok(())
    }
}

/// Has fixup maps & section content bytes ref data.
///
/// Normally, the 0th `classNameOffset` of this map will contain the starting position
/// of the string `hkRootLevelContainer` in the `__classnames__` section.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SectionContents<'bytes> {
    /// Location information needed when referencing class pointer, etc.
    pub global_map: IndexMap<u32, GlobalFixup>,
    /// This information indicates where the data of the field in the Class will be placed.
    pub local_map: IndexMap<u32, LocalFixup>,
    /// Location information for the name of the C++ class that must call the constructor.
    pub virtual_map: IndexMap<u32, VirtualFixup>,
    /// Each section bytes data.
    ///
    /// `== &bytes[absolute_data_start..absolute_data_start + local_fixups_offset]`
    pub section_data: &'bytes [u8],
    ///  Index Section ID
    ///
    /// # Examples
    /// - `__class_names__`: 1
    /// - `__type__`: 2
    /// - `__data__`: 3
    pub section_index: u32,
}

impl<'a> SectionContents<'a> {
    const OFFSET_MAP_IS_NONE: u32 = 0xFFFFFFFF;

    /// Read the fixup map from bytes by relying on the section header.
    ///
    /// # Assumption
    /// - Bytes position is 0 start(== HKX Header start). **Not section header start**.
    pub fn from_bytes<B: ByteOrder>(
        bytes: &'a [u8],
        section_header: &SectionHeader<B>,
        section_id: u32,
    ) -> std::io::Result<Self> {
        let absolute_data_start = section_header.absolute_data_start.get() as usize;
        let local_fixups_offset = section_header.local_fixups_offset.get() as usize;
        let global_fixups_offset = section_header.global_fixups_offset.get() as usize;
        let virtual_fixups_offset = section_header.virtual_fixups_offset.get() as usize;
        let exports_offset = section_header.exports_offset.get() as usize;

        let section_data = &bytes[absolute_data_start..absolute_data_start + local_fixups_offset];

        // Read local fixups
        let mut offset = absolute_data_start + local_fixups_offset;
        let mut local_map = IndexMap::new();

        let max_readable_count =
            (global_fixups_offset - local_fixups_offset) / size_of::<LocalFixup>();
        for _ in 0..max_readable_count {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let local_fixup = LocalFixup::from_bytes::<B>(&bytes[offset..])?;
                offset += size_of::<LocalFixup>();
                local_map.insert(local_fixup.src, local_fixup);
            }
        }

        // Read global fixups
        let mut offset = absolute_data_start + global_fixups_offset;
        let mut global_map = IndexMap::new();

        for _ in 0..(virtual_fixups_offset - global_fixups_offset) / size_of::<GlobalFixup>() {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let global_fixup = GlobalFixup::from_bytes::<B>(&bytes[offset..])?;
                offset += size_of::<GlobalFixup>();
                global_map.insert(global_fixup.src, global_fixup);
            }
        }

        // Read virtual fixups
        let mut offset = absolute_data_start + virtual_fixups_offset;
        let mut virtual_map = IndexMap::new();
        let max_readable_count =
            (exports_offset - virtual_fixups_offset) / size_of::<VirtualFixup>();

        for _ in 0..max_readable_count {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let virtual_fixup = VirtualFixup::from_bytes::<B>(&bytes[offset..])?;
                offset += size_of::<VirtualFixup>();
                virtual_map.insert(virtual_fixup.src, virtual_fixup);
            }
        }

        Ok(SectionContents {
            global_map,
            local_map,
            virtual_map,
            section_data,
            section_index: section_id,
        })
    }

    /// Write All fixup map & Set fixup's offsets to section header.
    ///
    /// And fills with `0xFF` until 16 bytes alignment.
    ///
    /// # Assumption
    /// - The `absolute_data_offset` of `section_header` is a valid value.
    /// - Bytes position is 0 start(== HKX Header start). **Not section header start**.
    ///
    /// # Note
    /// This is for the convenience of updating the `section header` to the correct fixup offset,
    /// so the byte write of the `section header` is called after this method.
    pub fn write_bytes<B: ByteOrder>(
        &self,
        bytes: &mut [u8],
        section_header: &mut SectionHeader<B>,
    ) -> std::io::Result<()> {
        let absolute_data_start = section_header.absolute_data_start.get();

        // Next write position. (At first, set a section start position.)
        let mut offset = absolute_data_start as usize;

        // Write section data
        bytes[offset..offset + self.section_data.len()].copy_from_slice(self.section_data);
        offset += self.section_data.len();

        // pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write local fixups
        let local_offset = offset as u32 - absolute_data_start;
        for (_, local_fixup) in &self.local_map {
            local_fixup.write_bytes::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<LocalFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write global fixups
        let global_offset = offset as u32 - absolute_data_start;
        for (_, global_fixup) in &self.global_map {
            global_fixup.write_bytes::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<GlobalFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write virtual fixups
        let virtual_offset = offset as u32 - absolute_data_start;
        for (_, virtual_fixup) in &self.virtual_map {
            virtual_fixup.write_bytes::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<VirtualFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        let export_offset = offset as u32 - absolute_data_start;

        // Update section header
        section_header.section_tag_separator = 0xFF;
        section_header.local_fixups_offset = U32::<B>::from(local_offset);
        section_header.global_fixups_offset = U32::<B>::from(global_offset);
        section_header.virtual_fixups_offset = U32::<B>::from(virtual_offset);
        section_header.exports_offset = U32::<B>::from(export_offset);
        section_header.imports_offset = U32::<B>::from(export_offset);
        section_header.end_offset = U32::<B>::from(export_offset);

        Ok(())
    }
}
