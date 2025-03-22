//! Havok C++ Class unique number.
//!
//! - In XML, it takes the place of a pointer.
//! - Not present in binary data.
//!
//! # Example
//! - `#0457`
//! - `#0007`
use crate::lib::*;
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
/// # use havok_types::pointer::Pointer;
/// assert_eq!(Pointer::new("#0050".into()).to_string(), "#0050");
/// assert_eq!(Pointer::new("#0100".into()).to_string(), "#0100");
/// ```
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(transparent))]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display("{0}")]
pub struct Pointer<'a>(
    #[cfg_attr(feature = "json_schema", schemars(with = "String"))] Cow<'a, str>,
);

impl Default for Pointer<'_> {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl<'a> Pointer<'a> {
    /// Creates a new `Pointer`
    #[inline]
    pub const fn new(ptr: Cow<'a, str>) -> Self {
        Self(ptr)
    }

    /// Creates a new null `Pointer`
    ///
    /// # Example
    /// ```
    /// # use havok_types::pointer::Pointer;
    /// assert_eq!(Pointer::null().to_string(), "null");
    /// ```
    #[inline]
    pub const fn null() -> Self {
        Self::new(Cow::Borrowed("null"))
    }

    /// Creates a new `Pointer` from a `usize`
    ///
    /// # Example
    /// ```
    /// # use havok_types::pointer::Pointer;
    /// assert_eq!(Pointer::from_usize(50).to_string(), "#0050");
    /// ```
    #[inline]
    pub fn from_usize(value: usize) -> Self {
        Self::new(format!("#{:04}", value).into())
    }

    /// Pointer(Class index) is null?
    ///
    /// # Example
    /// ```
    /// # use havok_types::pointer::Pointer;
    /// assert!(Pointer::new(std::borrow::Cow::Borrowed("#0000")).is_null());
    /// assert!(Pointer::new(std::borrow::Cow::Borrowed("null")).is_null());
    /// assert!(Pointer::new(std::borrow::Cow::Borrowed("")).is_null());
    /// ```
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self.0.as_ref(), "null" | "#0000" | "")
    }

    /// Get inner value.
    #[inline]
    pub const fn get(&self) -> &Cow<'a, str> {
        &self.0
    }

    /// Into inner value.
    #[inline]
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// To owned 'static pointer
    #[inline]
    pub fn to_static(&self) -> Pointer<'static> {
        Pointer(Cow::Owned(self.0.to_string()))
    }
}

// #0000
// #9999
impl From<usize> for Pointer<'_> {
    #[inline]
    fn from(value: usize) -> Self {
        Self::from_usize(value)
    }
}

// #0000
// #9999
// $sample10 <- new for Nemesis
impl<'a> From<Cow<'a, str>> for Pointer<'a> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        Self(value)
    }
}
