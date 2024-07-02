//! Trait that defines a set of dedicated methods for writing Havok Class data.
use byteorder::WriteBytesExt as _;
use havok_serde::HavokClass;
use std::{
    collections::HashMap,
    io::{self, Cursor, Write as _},
};
use zerocopy::ByteOrder;

use crate::common::trait_impls::Align as _;

/// Trait that defines a set of dedicated methods for writing Havok Class data.
pub trait ClassNamesWriter {
    /// Write classnames section
    fn write_classnames_section<O, K, V>(
        &mut self,
        classes: &indexmap::IndexMap<K, V>,
    ) -> io::Result<HashMap<&'static str, u32>>
    where
        O: ByteOrder,
        K: core::fmt::Debug,
        V: HavokClass;
}

impl ClassNamesWriter for Cursor<Vec<u8>> {
    fn write_classnames_section<O, K, V>(
        &mut self,
        classes: &indexmap::IndexMap<K, V>,
    ) -> io::Result<HashMap<&'static str, u32>>
    where
        O: ByteOrder,
        K: core::fmt::Debug,
        V: HavokClass,
    {
        let classnames_section_start = self.position() as u32;

        let mut class_map = HashMap::new();

        // Four class information that for some reason is always present in the `__classnames__` section.
        // Not sure where this is used.
        self.write_u32::<O>(0x75585ef6)?;
        self.write_u8(0x09)?;
        self.write_all(b"hkClass\0")?;

        self.write_u32::<O>(0x5C7EA4C2)?;
        self.write_u8(0x09)?;
        self.write_all(b"hkClassMember\0")?;

        self.write_u32::<O>(0x8A3609CF)?;
        self.write_u8(0x09)?;
        self.write_all(b"hkClassEnum\0")?;

        self.write_u32::<O>(0xCE6F8A6C)?;
        self.write_u8(0x09)?;
        self.write_all(b"hkClassEnumItem\0")?;

        for (_, class) in classes.iter() {
            self.write_u32::<O>(class.signature().into())?;
            self.write_u8(0x09)?;

            let name_start = self.position() as u32;
            let class_name = class.name();
            self.write_all(class_name.to_bytes_with_nul())?;

            if classnames_section_start >= name_start {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "abs position is before the name_start position",
                ));
            }
            let class_name = class_name.to_str().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Failed to cast CStr to utf8 str",
                )
            })?;

            class_map.insert(class_name, name_start - classnames_section_start);
        }
        self.align(16, 0xff)?;

        Ok(class_map)
    }
}
