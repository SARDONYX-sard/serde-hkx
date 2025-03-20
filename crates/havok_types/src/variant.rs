//! Variant: object pointer + class meta pointer
use crate::Pointer;

/// C++ info
/// - type size: `   8`(x86)/` 16`(x86_64)
///
/// Only used for `value` of `hkCustomAttributesAttribute`.
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variant<'a> {
    /// # C++ Info
    /// - name: `object`(ctype: `void*`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "Pointer<'a>: serde::Deserialize<'de>"))
    )]
    pub object: Pointer<'a>,
    /// # C++ Info
    /// - name: `class`(ctype: `hkClass*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/` 8`(x86_64)
    ///
    /// `hkClass*` is a class that holds meta-information (Flags, vtable, etc.) for each C++ Havok class and is stored in its own static field.
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "Pointer<'a>: serde::Deserialize<'de>"))
    )]
    pub class: Pointer<'a>,
}

impl<'a> Variant<'a> {
    /// Creates a new `Variant`
    #[inline]
    pub const fn new(object: Pointer<'a>, class: Pointer<'a>) -> Self {
        Self { object, class }
    }
}
