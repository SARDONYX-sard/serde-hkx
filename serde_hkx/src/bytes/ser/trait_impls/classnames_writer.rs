//! Trait that defines a set of dedicated methods for writing Havok Class data.
use super::Align as _;
use byteorder::{ByteOrder, WriteBytesExt as _};
use havok_serde::HavokClass;
use indexmap::IndexMap;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Cursor, Write as _},
};

/// This must be retained because the information about which C++ class was written to which location
/// will be in `virtual_fixup.src`.
pub type ClassStartsMap = IndexMap<&'static str, u32>;

/// Trait that defines a set of dedicated methods for writing Havok Class data.
/// # Note
///This is implemented in the type that serializes the classes.
pub trait ClassNamesWriter {
    /// Write `__classnames__` section contents.
    fn write_classnames_section<O>(
        &self,
        writer: &mut Cursor<Vec<u8>>,
    ) -> io::Result<ClassStartsMap>
    where
        O: ByteOrder;
}

macro_rules! impl_writer_for_map {
    ($($ty_name:ident),+ $(,)?) => {
        $(impl<K, V> ClassNamesWriter for $ty_name<K, V>
        where
            V: HavokClass,
        {
            fn write_classnames_section<O>(
                &self,
                writer: &mut Cursor<Vec<u8>>,
            ) -> io::Result<ClassStartsMap>
            where
                O: ByteOrder,
            {
                let classnames_section_start = writer.position() as u32;

                // If a class is used in the __data__ section, an `hkClass` is used to represent that information
                // Similarly, `hkClassEnum` is used depending on whether or not enum is used,
                // but here we assume that it is always used and write it in advance.
                writer.write_u32::<O>(0x75585ef6)?;
                writer.write_all(b"\x09hkClass\0")?;

                writer.write_u32::<O>(0x5C7EA4C2)?;
                writer.write_all(b"\x09hkClassMember\0")?;

                writer.write_u32::<O>(0x8A3609CF)?;
                writer.write_all(b"\x09hkClassEnum\0")?;

                writer.write_u32::<O>(0xCE6F8A6C)?;
                writer.write_all(b"\x09hkClassEnumItem\0")?;

                let mut class_map = ClassStartsMap::new(); // To write `virtual_fixup`
                for (_, class) in self.iter() {
                    let class_name = class.name();
                    if class_map.contains_key(class_name) {
                        continue;
                    }

                    writer.write_u32::<O>(class.signature().into())?;
                    writer.write_u8(0x09)?;

                    let name_start = writer.position() as u32;
                    writer.write_all(class_name.as_bytes())?;
                    writer.write_u8(0)?;

                    if classnames_section_start > name_start {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Must be  name_start position >= __classnames__ abs position",
                        ));
                    }

                    let name_offset = name_start - classnames_section_start;
                    class_map.insert(class_name, name_offset);
                }
                writer.align(16, 0xff)?;

                Ok(class_map)
            }
        })*
    };
}

impl_writer_for_map!(IndexMap, HashMap);

macro_rules! impl_writer_for_slice {
    ($($ty_name:ident),+ $(,)?) => {
        $(impl<V> ClassNamesWriter for $ty_name<V>
        where
            V: HavokClass,
        {
            fn write_classnames_section<O>(
                &self,
                writer: &mut Cursor<Vec<u8>>,
            ) -> io::Result<ClassStartsMap>
            where
                O: ByteOrder,
            {
                let classnames_section_start = writer.position() as u32;

                // If a class is used in the __data__ section, an `hkClass` is used to represent that information
                // Similarly, `hkClassEnum` is used depending on whether or not enum is used,
                // but here we assume that it is always used and write it in advance.
                writer.write_u32::<O>(0x75585ef6)?;
                writer.write_all(b"\x09hkClass\0")?;

                writer.write_u32::<O>(0x5C7EA4C2)?;
                writer.write_all(b"\x09hkClassMember\0")?;

                writer.write_u32::<O>(0x8A3609CF)?;
                writer.write_all(b"\x09hkClassEnum\0")?;

                writer.write_u32::<O>(0xCE6F8A6C)?;
                writer.write_all(b"\x09hkClassEnumItem\0")?;

                let mut class_map = ClassStartsMap::new(); // To write `virtual_fixup`
                for class in self.iter() {
                    let class_name = class.name();
                    if class_map.contains_key(class_name) {
                        continue;
                    }

                    writer.write_u32::<O>(class.signature().into())?;
                    writer.write_u8(0x09)?;

                    let name_start = writer.position() as u32;
                    writer.write_all(class_name.as_bytes())?;
                    writer.write_u8(0)?;

                    if classnames_section_start > name_start {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Must be  name_start position >= __classnames__ abs position",
                        ));
                    }

                    let name_offset = name_start - classnames_section_start;
                    class_map.insert(class_name, name_offset);
                }
                writer.align(16, 0xff)?;

                Ok(class_map)
            }
        })*
    };
}

impl_writer_for_slice!(Vec, HashSet);

impl<V> ClassNamesWriter for &[V]
where
    V: HavokClass,
{
    fn write_classnames_section<O>(
        &self,
        writer: &mut Cursor<Vec<u8>>,
    ) -> io::Result<ClassStartsMap>
    where
        O: ByteOrder,
    {
        let classnames_section_start = writer.position() as u32;

        // These classes are meta-information, such as C++ class signatures, and are always considered
        // to exist because they are already defined in the SDK.
        writer.write_u32::<O>(0x75585ef6)?;
        writer.write_all(b"\x09hkClass\0")?;

        writer.write_u32::<O>(0x5C7EA4C2)?;
        writer.write_all(b"\x09hkClassMember\0")?;

        writer.write_u32::<O>(0x8A3609CF)?;
        writer.write_all(b"\x09hkClassEnum\0")?;

        writer.write_u32::<O>(0xCE6F8A6C)?;
        writer.write_all(b"\x09hkClassEnumItem\0")?;

        let mut class_map = ClassStartsMap::new(); // To write `virtual_fixup`
        for class in self.iter() {
            let class_name = class.name();
            if class_map.contains_key(class_name) {
                continue;
            }

            writer.write_u32::<O>(class.signature().into())?;
            writer.write_u8(0x09)?;

            let name_start = writer.position() as u32;
            writer.write_all(class_name.as_bytes())?;
            writer.write_u8(0)?;

            if classnames_section_start > name_start {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Must be  name_start position >= __classnames__ abs position",
                ));
            }

            let name_offset = name_start - classnames_section_start;
            class_map.insert(class_name, name_offset);
        }
        writer.align(16, 0xff)?;

        Ok(class_map)
    }
}
