use crate::lib::*;

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn cautious<Element>(hint: Option<usize>) -> usize {
    const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

    if mem::size_of::<Element>() == 0 {
        0
    } else {
        cmp::min(
            hint.unwrap_or(0),
            MAX_PREALLOC_BYTES / mem::size_of::<Element>(),
        )
    }
}
