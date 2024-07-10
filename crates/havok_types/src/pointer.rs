//! Havok C++ Class unique number.
//!
//! - In XML, it takes the place of a pointer.
//! - Not present in binary data.
//!
//! # Example
//! - `#0457`
//! - `#0007`use core::str::FromStr;
use core::str::FromStr;
use parse_display::Display;

/// Havok C++ Class unique number.
///
/// It is automatically assigned as an index during XML deserialization.
///
/// - XML: Class pointer. (e.g. `#0050`)
/// - hkx binary data: Not exist.
///
/// # Examples
/// ```
/// use havok_types::pointer::Pointer;
///
/// assert_eq!(Pointer::new(50).to_string(), "#0050");
/// assert_eq!(Pointer::new(100).to_string(), "#0100");
/// assert_eq!(Pointer::new(1000).to_string(), "#1000");
/// assert_eq!(Pointer::new(10000).to_string(), "#10000");
///
/// assert_eq!("#0050".parse(), Ok(Pointer::new(50)));
/// assert_eq!("#100".parse(),  Ok(Pointer::new(100)));
/// assert_eq!("#1000".parse(), Ok(Pointer::new(1000)));
/// assert_eq!("#10000".parse(),Ok(Pointer::new(10000)));
/// ```
///
/// # Note
/// The [`Copy`] is derive for [`usize`] wrapper type.
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display("#{0:04}")]
pub struct Pointer(usize);

impl Pointer {
    /// Creates a new `Pointer`
    pub const fn new(ptr: usize) -> Self {
        Self(ptr)
    }

    /// Get inner value.
    #[inline]
    pub const fn get(&self) -> usize {
        self.0
    }
}

impl FromStr for Pointer {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            parse_int::parse(s.trim_start_matches('#'))
                .map_err(|_err| "Invalid integer for Name")?,
        ))
    }
}

impl From<usize> for Pointer {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Pointer> for usize {
    #[inline]
    fn from(value: Pointer) -> Self {
        value.0
    }
}
