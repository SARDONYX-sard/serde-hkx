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
/// assert_eq!(Pointer::new("#0050").to_string(), "#0050");
/// assert_eq!(Pointer::new("#0100").to_string(), "#0100");
/// ```
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(transparent))]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display("{0}")]
pub struct Pointer<'a>(
    #[cfg_attr(feature = "json_schema", schemars(with = "String"))] Cow<'a, str>,
);

impl<'a> Pointer<'a> {
    /// Creates a new `Pointer`
    #[inline]
    pub const fn new(ptr: Cow<'a, str>) -> Self {
        Self(ptr)
    }

    /// Pointer(Class index) is null(`#0000`)?
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == "#0000"
    }

    /// Get inner value.
    #[inline]
    pub const fn get(&self) -> &Cow<'a, str> {
        &self.0
    }
}

/// #0000
/// #9999
/// $sample$ <- new for Nemesis
impl<'a> From<Cow<'a, str>> for Pointer<'a> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        Self(value)
    }
}

// #[cfg(feature = "serde")]
// impl<'de: 'a, 'a> serde::Deserialize<'de> for Pointer<'a> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct PointerVisitor;

//         impl<'de> serde::de::Visitor<'de> for PointerVisitor {
//             type Value = Pointer<'de>;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("Pointer with a string starting with '#' or '$'")
//             }

//             fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
//             where
//                 E: serde::de::Error,
//             {
//                 if value.starts_with('#') || value.starts_with('$') {
//                     Ok(Pointer::new(Cow::Borrowed(value)))
//                 } else {
//                     Err(E::custom(format!("invalid pointer format: {}", value)))
//                 }
//             }
//         }
//         deserializer.deserialize_str(PointerVisitor)
//     }
// }
