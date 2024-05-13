//! Havok C++ Class signature hex number.
//!
//! # Example
//! - `0x13a39ba7`
use core::str::FromStr;
use derive_new::new;
use parse_display::Display;

/// Havok C++ Class signature hex number.
///
/// # Examples
///
/// ```
/// use havok_types::signature::Signature;
///
/// assert_eq!(Signature::new(0x13a39ba7).to_string(), "0x13a39ba7");
/// assert_eq!(Signature::new(0x00000001).to_string(), "0x00000001");
/// assert_eq!(Signature::new(0x000000001).to_string(), "0x00000001");
///
/// assert_eq!("0x13a39ba7".parse(), Ok(Signature::new(0x13a39ba7)));
/// ```
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[repr(transparent)]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, new)]
#[display("0x{0:08x}")]
pub struct Signature(u32);

impl FromStr for Signature {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            parse_int::parse(s).map_err(|_err| "Invalid integer for Signature")?,
        ))
    }
}
