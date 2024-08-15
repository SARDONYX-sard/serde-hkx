use crate::lib::*;

/// Not an IEEE expression `f16`
/// # Note
/// - This `f16` is made by truncating the last 16 bits of [`f32`] to 7-bit precision.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct f16(u16);

impl f16 {
    // NOTE: The bits in f64 and f32 are different, so if we get bits from f64, it will be an invalid value.
    //
    /// Creates a new [`f16`] from [`f32`].
    #[inline]
    pub fn from_f32(f: f32) -> Self {
        let bits = f.to_bits();
        Self((bits >> 16) as u16) // Only the most significant 16bits are taken out.
    }

    /// Converts the [`f16`] to a 32-bit float.
    #[inline]
    pub fn to_f32(self) -> f32 {
        f32::from_bits((self.0 as u32) << 16)
    }

    /// Return as [`u16`].
    #[inline]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    /// Creates a new [`f16`] from a little-endian byte array.
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }

    /// Creates a new `f16` from a big-endian byte array.
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_be_bytes(bytes))
    }

    /// Return the memory representation of this integer as a byte array in little-endian byte order.
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.to_bits().to_le_bytes()
    }

    /// Return the memory representation of this integer as a byte array in little-endian byte order.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.to_bits().to_be_bytes()
    }
}

impl fmt::Display for f16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.06}", self.to_f32())
    }
}

impl fmt::Debug for f16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.06}", self.to_f32())
    }
}

#[test]
fn test_half() {
    let half = f16::from_f32(1.0);
    assert_eq!(half.to_le_bytes(), [0x80, 0x3f]);
    assert_eq!(half.to_f32(), 1.0);
    assert_eq!(half.to_string(), "1.000000");

    let half = f16::from_f32(0.049805);
    assert_eq!(half.to_le_bytes(), [0x4c, 0x3d]);
    assert_eq!(half.to_f32(), 0.049805);
    assert_eq!(half.to_string(), "0.049805");
}
