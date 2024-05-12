//! C++ Havok enum & it's name & value information.
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// C++ Havok enum information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Enum<'a> {
    /// enum Identifier
    #[serde(bound(deserialize = "Cow<'a, str>: Deserialize<'de>"))]
    pub name: Cow<'a, str>,

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
