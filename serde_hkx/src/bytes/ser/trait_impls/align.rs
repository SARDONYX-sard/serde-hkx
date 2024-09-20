//! Trait for n-byte alignment
use crate::align;
use std::io::{self, Cursor, Write as _};

/// Trait for n-byte alignment.
pub trait Align {
    /// Fill the binary data in Cursor with zeros until it reaches the desired alignment.
    fn zero_fill_align(&mut self, align: u64) -> io::Result<()>;

    /// Alignment of binary data in Cursor with arbitrary byte data.
    fn align(&mut self, align: u64, byte: u8) -> io::Result<()>;
}

impl Align for Cursor<Vec<u8>> {
    #[inline]
    fn zero_fill_align(&mut self, align: u64) -> io::Result<()> {
        Align::align(self, align, 0)
    }

    #[inline]
    fn align(&mut self, align: u64, byte: u8) -> io::Result<()> {
        let position = self.position();
        let aligned_position = align!(position, align);

        if aligned_position != position {
            let padding = aligned_position - position;
            for _ in 0..padding {
                self.write_all(&[byte])?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_fill_align() {
        let mut cursor = Cursor::new(vec![1, 2, 3, 4, 5]);

        // Initially at position 0, move to 5 and zero-fill align to 8-byte boundary
        cursor.set_position(5);
        cursor.zero_fill_align(8).unwrap();

        assert_eq!(cursor.position(), 8);
        assert_eq!(cursor.get_ref(), &[1, 2, 3, 4, 5, 0, 0, 0]);
    }

    #[test]
    fn test_zero_fill_align_no_padding_needed() {
        let mut cursor = Cursor::new(vec![1, 2, 3, 4, 5, 0xAA, 0xAA, 0xAA]);

        // Position is already aligned, so no zero-filling should occur
        cursor.set_position(8);
        cursor.zero_fill_align(8).unwrap();

        assert_eq!(cursor.position(), 8);
        assert_eq!(cursor.get_ref(), &[1, 2, 3, 4, 5, 0xAA, 0xAA, 0xAA]); // No changes to the content
    }

    #[test]
    fn test_align_with_byte_padding() {
        let mut cursor = Cursor::new(vec![1, 2, 3, 4, 5]);

        // Initially at position 5, align to 8-byte boundary with custom padding (e.g., 0xAA)
        cursor.set_position(5);
        cursor.align(8, 0xAA).unwrap();

        assert_eq!(cursor.position(), 8);
        assert_eq!(cursor.get_ref(), &[1, 2, 3, 4, 5, 0xAA, 0xAA, 0xAA]);
    }

    #[test]
    fn test_align_no_padding_needed() {
        let mut cursor = Cursor::new(vec![1, 2, 3, 4, 5, 0xAA, 0xAA, 0xAA]);

        // Position is already at 8 (aligned), so no padding should occur
        cursor.set_position(8);
        cursor.align(8, 0xAA).unwrap();

        assert_eq!(cursor.position(), 8);
        assert_eq!(cursor.get_ref(), &[1, 2, 3, 4, 5, 0xAA, 0xAA, 0xAA]); // No changes to the content
    }
}
