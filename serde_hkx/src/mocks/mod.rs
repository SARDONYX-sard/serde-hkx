pub mod classes;
pub mod constructors;
pub mod enums;
pub mod flags;

/// Reduce the burden of individual imports by importing a set of types needed to create a havok class structure here.
mod class_requires {
    pub use crate::lib::*;
    pub use havok_serde::{
        de::{self, Deserialize, Deserializer, MapAccess, Visitor},
        ser::{Error as _, Serialize, SerializeFlags, SerializeStruct, Serializer},
        HavokClass,
    };
    pub use havok_types::*;
}
