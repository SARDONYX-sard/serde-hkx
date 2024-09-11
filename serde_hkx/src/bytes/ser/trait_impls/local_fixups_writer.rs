use byteorder::{BigEndian, LittleEndian, WriteBytesExt as _};
use std::io;

pub trait LocalFixupsWriter {
    /// Write local fixups data
    fn write_local_fixups(&mut self, src: u32, dst: u32, is_little_endian: bool) -> io::Result<()>;
}

impl LocalFixupsWriter for Vec<u8> {
    #[inline]
    fn write_local_fixups(&mut self, src: u32, dst: u32, is_little_endian: bool) -> io::Result<()> {
        match is_little_endian {
            true => {
                self.write_u32::<LittleEndian>(src)?;
                self.write_u32::<LittleEndian>(dst)
            }
            false => {
                self.write_u32::<BigEndian>(src)?;
                self.write_u32::<BigEndian>(dst)
            }
        }
    }
}
