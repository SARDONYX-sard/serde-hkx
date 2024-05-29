//! Trait for n-byte alignment
use std::io::{self, Cursor, Write as _};

/// Trait for n-byte alignment.
pub trait Align {
    /// Returns the bytes to align the current position with the specified alignment.
    fn align(&mut self, align: usize) -> io::Result<()>;
}

impl Align for Cursor<Vec<u8>> {
    fn align(&mut self, align: usize) -> io::Result<()> {
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
}
