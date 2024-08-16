use parse_display::{Display, FromStr};

/// Ptr size is the same as the ptr size.
/// - 32bit for hkx for 32bit
/// - 64bit for hkx for 64bit
///
/// # Why not use [`usize`]?
/// Because if this code is compiled as a 32-bit application, the usize will be 32-bit, and 32-bit will not be able to reserve the size for 64-bit.
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr)]
#[display("{0}")]
pub struct Ulong(u64);

impl Ulong {
    /// Creates a new `Ulong`
    #[inline]
    pub const fn new(num: u64) -> Self {
        Self(num)
    }

    /// Get inner value.
    #[inline]
    pub const fn get(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Ulong {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Ulong> for u64 {
    #[inline]
    fn from(value: Ulong) -> Self {
        value.0
    }
}
