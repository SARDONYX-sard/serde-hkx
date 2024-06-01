//! Trait that defines a set of dedicated methods for writing Havok Class data.
use crate::cursor_ext::align::Align as _;
use byteorder::WriteBytesExt as _;
use havok_serde::HavokClass;
use std::io::{self, Cursor, Write as _};
use zerocopy::ByteOrder;

/// Trait that defines a set of dedicated methods for writing Havok Class data.
pub trait HavokWrite {
    /// Write classnames section
    fn write_classnames_section<T, O>(&mut self, classes: &[T]) -> io::Result<()>
    where
        T: HavokClass,
        O: ByteOrder;
}

impl HavokWrite for Cursor<Vec<u8>> {
    fn write_classnames_section<T, O>(&mut self, classes: &[T]) -> io::Result<()>
    where
        T: HavokClass,
        O: ByteOrder,
    {
        // Four class information that for some reason is always present in the `__classnames__` section.
        // Not sure where this is used.
        self.write_u32::<O>(0x75585ef6)?;
        self.write_u8(0x09)?;
        self.write(c"hkClass".to_bytes_with_nul())?;

        self.write_u32::<O>(0x5C7EA4C2)?;
        self.write_u8(0x09)?;
        self.write(c"hkClassMember".to_bytes_with_nul())?;

        self.write_u32::<O>(0x8A3609CF)?;
        self.write_u8(0x09)?;
        self.write(c"hkClassEnum".to_bytes_with_nul())?;

        self.write_u32::<O>(0xCE6F8A6C)?;
        self.write_u8(0x09)?;
        self.write(c"hkClassEnumItem".to_bytes_with_nul())?;

        for class in classes.iter() {
            self.write_u32::<O>(class.signature().into())?;
            self.write_u8(0x09)?;
            self.write(class.name().to_bytes_with_nul())?;
        }
        self.align(16, 0xff)
    }
}
