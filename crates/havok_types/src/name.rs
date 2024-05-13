//! Havok C++ Class unique number.
//!
//! - In XML, it takes the place of a pointer.
//! - Not present in binary data.
//!
//! # Example
//! - `#0457`
//! - `#0007`use core::str::FromStr;
use core::str::FromStr;
use derive_new::new;
use parse_display::Display;

/// Havok C++ Class unique number.
///
/// - In XML, it takes the place of a pointer.
/// - Not present in binary data.
///
/// # Examples
/// ```
/// use havok_types::name::Name;
///
/// assert_eq!(Name::new(50).to_string(), "#0050");
/// assert_eq!(Name::new(100).to_string(), "#0100");
/// assert_eq!(Name::new(1000).to_string(), "#1000");
/// assert_eq!(Name::new(10000).to_string(), "#10000");
///
/// assert_eq!("#0050".parse(), Ok(Name::new(50)));
/// assert_eq!("#100".parse(), Ok(Name::new(100)));
/// assert_eq!("#1000".parse(), Ok(Name::new(1000)));
/// assert_eq!("#10000".parse(), Ok(Name::new(10000)));
/// ```
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[repr(transparent)]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, new)]
#[display("#{0:04}")]
pub struct Name(usize);

impl FromStr for Name {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            parse_int::parse(s.trim_start_matches('#'))
                .map_err(|_err| "Invalid integer for Name")?,
        ))
    }
}
