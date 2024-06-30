//! C++ Havok enum & it's name & value information.
use super::TypeKind;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// C++ Havok enum information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Enum<'a> {
    /// enum Identifier
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    pub name: Cow<'a, str>,

    /// Is this enum an `TYPE_ENUM` or a `TYPE_FLAGS`?
    pub vtype: TypeKind,

    /// The size of this enum when it is written to the binary; if it is `TYPE_VOID`, it means that the enum is not used and is unknown.
    pub vsubtype: TypeKind,

    /// Unknown flags . Always `00000000` in hk2010 version
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    pub flags: Cow<'a, str>,

    /// C++ Havok names & values(in enum) information.
    pub enum_item: Vec<EnumItem<'a>>,
}

/// C++ Havok name & value(in enum) information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EnumItem<'a> {
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    pub name: Cow<'a, str>,
    /// If the value of `enum_item` is set to [`i32`] and it is `u32`, the value is interpreted as [`i128`] because the numbers do not match.
    ///
    /// This allows you to convert the value to [`i64`], [`u64`] with `as` and still get the value correctly.
    pub value: i128,
}
