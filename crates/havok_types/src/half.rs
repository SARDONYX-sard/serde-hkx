use crate::lib::*;

/// Not an IEEE expression `f16`
///
/// # Note
/// - It is created by taking the upper 16 bits from f32.
/// - Internally, it is the same as u16.
#[allow(non_camel_case_types)]
#[repr(transparent)]
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

    /// Return the memory representation of this integer as a byte array in big-endian byte order.
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

#[cfg(feature = "serde")]
const _: () = {
    use super::f16;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for f16 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.to_f32().serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for f16 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let f = f32::deserialize(deserializer)?;
            Ok(Self::from_f32(f))
        }
    }
};

#[test]
fn test_half() {
    {
        let half = f16::from_f32(1.0);
        const HALF_LE: [u8; 2] = [0x80, 0x3f];
        const HALF_BE: [u8; 2] = [0x3f, 0x80];

        assert_eq!(half.to_le_bytes(), HALF_LE);
        assert_eq!(half.to_be_bytes(), HALF_BE);
        assert_eq!(f16::from_le_bytes(HALF_LE), half);
        assert_eq!(f16::from_be_bytes(HALF_BE), half);
        assert_eq!(half.to_f32().to_bits(), (1.0_f32).to_bits()); // Check f32 at the bit level because of errors.
        assert_eq!(half.to_string(), "1.000000");
    }

    {
        let half = f16::from_f32(0.049805);
        const HALF_LE: [u8; 2] = [0x4c, 0x3d];
        const HALF_BE: [u8; 2] = [0x3d, 0x4c];
        assert_eq!(half.to_le_bytes(), HALF_LE);
        assert_eq!(half.to_be_bytes(), HALF_BE);
        assert_eq!(f16::from_le_bytes(HALF_LE), half);
        assert_eq!(f16::from_be_bytes(HALF_BE), half);
        assert_eq!(half.to_f32().to_bits(), (0.049804688_f32).to_bits()); // Check f32 at the bit level because of errors.
        assert_eq!(half.to_string(), "0.049805");
    }
}
