//! Trait for n-byte alignment
use std::io::{self, Cursor, Write as _};

/// Trait for n-byte alignment.
pub trait Align {
    /// Fill the binary data in Cursor with zeros until it reaches the desired alignment.
    fn zero_fill_align(&mut self, align: usize) -> io::Result<()>;

    /// Alignment of binary data in Cursor with arbitrary byte data.
    fn align(&mut self, align: usize, byte: u8) -> io::Result<()>;

    fn fill_until(&mut self, target_pos: u64, byte: u8) -> io::Result<()>;
}

impl Align for Cursor<Vec<u8>> {
    fn zero_fill_align(&mut self, align: usize) -> io::Result<()> {
        let position = self.position() as usize;
        let offset = position % align;

        if offset != 0 {
            debug_assert!(align >= offset);
            let padding = align - offset;

            // Length cannot be determined at compile time, so vec must be used.
            let padding_bytes = vec![0u8; padding];
            self.write_all(&padding_bytes)?;
        }
        Ok(())
    }

    fn align(&mut self, align: usize, byte: u8) -> io::Result<()> {
        let position = self.position() as usize;
        let offset = position % align;

        if offset != 0 {
            debug_assert!(align >= offset);
            let padding = align - offset;

            // Length cannot be determined at compile time, so vec must be used.
            let padding_bytes = vec![byte; padding];
            self.write_all(&padding_bytes)?;
        }
        Ok(())
    }

    fn fill_until(&mut self, target_pos: u64, byte: u8) -> io::Result<()> {
        let current_pos = self.position();
        if target_pos < current_pos {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Target position is before the current position",
            ));
        }

        let num_bytes_to_write = (target_pos - current_pos) as usize;
        let buffer = vec![byte; num_bytes_to_write];
        self.write_all(&buffer)?;
        self.set_position(target_pos);

        Ok(())
    }
}
