//! Havok C++ Class signature hex number.
//!
//! # Example
//! - `0x13a39ba7`
use core::str::FromStr;
use derive_new::new;
use parse_display::Display;

/// Havok C++ Class signature hex number.
///
/// - XML: hex string(At least 8 digits)
/// - hkx binary data: Endianness bytes
///
/// # Examples
/// ```
/// use havok_types::signature::Signature;
///
/// assert_eq!(Signature::new(0x13a39ba7).to_string(), "0x13a39ba7");
/// assert_eq!(Signature::new(0x00000001).to_string(), "0x1");
/// assert_eq!(Signature::new(0x000000001).to_string(), "0x1");
/// assert_eq!(Signature::new(0x076ad60a).to_string(), "0x76ad60a");
///
/// assert_eq!("0x13a39ba7".parse(), Ok(Signature::new(0x13a39ba7)));
/// ```
///
/// # NOte
/// The [`Copy`] is derive for [`usize`] wrapper type.
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, new)]
#[display("{0:#x}")]
pub struct Signature(u32);

impl Signature {
    /// Get inner value.
    #[inline]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl FromStr for Signature {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            parse_int::parse(s).map_err(|_err| "Invalid integer for Signature")?,
        ))
    }
}

impl From<u32> for Signature {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Signature> for u32 {
    fn from(value: Signature) -> Self {
        value.0
    }
}
