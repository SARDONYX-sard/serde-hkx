use parse_display::{Display, FromStr};

/// Ptr size is the same as the ptr size.
/// - 32bit for hkx for 32bit
/// - 64bit for hkx for 64bit
///
/// # Why not use [`usize`]?
/// Because if this code is compiled as a 32-bit application, the usize will be 32-bit, and 32-bit will not be able to reserve the size for 64-bit.
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_ulong() {
        let num = 12345_u64;
        let ulong = Ulong::new(num);
        assert_eq!(ulong.get(), num);
    }

    #[test]
    fn test_from_u64() {
        let num = 67890_u64;
        let ulong: Ulong = num.into();
        assert_eq!(ulong.get(), num);
    }

    #[test]
    fn test_into_u64() {
        let num = 54321_u64;
        let ulong = Ulong::new(num);
        let value: u64 = ulong.into();
        assert_eq!(value, num);
    }

    #[test]
    fn test_display() {
        let ulong = Ulong::new(12345_u64);
        assert_eq!(format!("{ulong}"), "12345");
    }

    #[test]
    fn test_from_str() {
        let num_str = "67890";
        let ulong = num_str.parse::<Ulong>();
        assert_eq!(ulong, Ok(Ulong::new(67890)));
    }

    #[test]
    fn test_default() {
        let default_ulong = Ulong::default();
        assert_eq!(default_ulong.get(), 0);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let ulong = Ulong::new(12345_u64);

        let serialized = serde_json::to_string(&ulong).unwrap();
        assert_eq!(serialized, "12345");

        let deserialized: Ulong = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ulong);
    }
}
