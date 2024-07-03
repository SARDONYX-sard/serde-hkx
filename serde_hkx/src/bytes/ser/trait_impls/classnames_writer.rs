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
        V: HavokClass;
}

impl ClassNamesWriter for Cursor<Vec<u8>> {
    fn write_classnames_section<O, K, V>(
        &mut self,
        classes: &indexmap::IndexMap<K, V>,
    ) -> io::Result<HashMap<&'static str, u32>>
    where
        O: ByteOrder,
        V: HavokClass,
    {
        let classnames_section_start = self.position() as u32;

        // These classes are meta-information, such as C++ class signatures, and are always considered
        // to exist because they are already defined in the SDK.
        self.write_u32::<O>(0x75585ef6)?;
        self.write_all(b"\x09hkClass\0")?;

        self.write_u32::<O>(0x5C7EA4C2)?;
        self.write_all(b"\x09hkClassMember\0")?;

        self.write_u32::<O>(0x8A3609CF)?;
        self.write_all(b"\x09hkClassEnum\0")?;

        self.write_u32::<O>(0xCE6F8A6C)?;
        self.write_all(b"\x09hkClassEnumItem\0")?;

        let mut class_map = HashMap::new(); // To write `virtual_fixup`
        for (_, class) in classes.iter() {
            self.write_u32::<O>(class.signature().into())?;
            self.write_u8(0x09)?;

            let name_start = self.position() as u32;
            let class_name = class.name();
            self.write_all(class_name.as_bytes())?;
            self.write_u8(0)?;

            if classnames_section_start >= name_start {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "abs position is before the name_start position",
                ));
            }

            class_map.insert(class_name, name_start - classnames_section_start);
        }
        self.align(16, 0xff)?;

        Ok(class_map)
    }
}
