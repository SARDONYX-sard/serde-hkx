//! Alignment functions

///  Get the aligned value.
/// - [Original code (Rust lang)](https://github.com/rust-lang/rust/blob/1.30.0/src/libcore/alloc.rs#L199-L219)
///
/// # Examples
///
/// ```rust
/// use serde_hkx::align;
///
/// assert_eq!(align!(10_u32, 4_u32), 12);
/// assert_eq!(align!(10_usize, 8_usize), 16);
/// ```
#[macro_export]
macro_rules! align {
    ($offset:expr, $align_num:expr) => {
        $offset.wrapping_add($align_num).wrapping_sub(1) & !$align_num.wrapping_sub(1)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_align() {
        assert_eq!(align!(88_u64, 16_u64), 96);
    }
}
