/// A module that collects frequently used molds. It is intended to be easily imported in one place.
pub use crate::bytes::{
    de::{
        from_bytes, from_bytes_with_opt, from_partial_bytes, from_partial_bytes_with_opt,
        BytesDeserializer,
    },
    ser::{to_bytes, to_bytes_with_opt, ByteSerializer},
};
pub use crate::errors::SerdeHkxError;
pub use crate::sort::HavokSort;
pub use crate::xml::{
    de::{
        from_partial_str, from_partial_str_with_opt, from_str, from_str_peek, from_str_with_opt,
        XmlDeserializer,
    },
    ser::{to_string, to_string_with_opt, XmlSerializer},
};
pub use havok_serde::{
    de::{self, Deserialize, Deserializer, Error as _, MapAccess, Visitor},
    ser::{self, Error as _, Serialize, SerializeFlags, SerializeStruct, Serializer},
    HavokClass,
};
pub use havok_types::*;

/// - key: class index(e.g. `#0001`)
/// - value: class
pub type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;
