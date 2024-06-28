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
    pub value: i32,
}
