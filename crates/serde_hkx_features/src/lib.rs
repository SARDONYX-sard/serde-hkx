pub mod convert;
pub mod diff;
pub mod dump;
pub mod error;
pub mod fs;
#[cfg(feature = "json_schema")]
pub mod json_schema_gen;
pub mod serde;
#[cfg(feature = "extra_fmt")]
pub mod serde_extra;
pub mod tree;
#[cfg(any(feature = "extra_fmt", feature = "json_schema"))]
pub mod types_wrapper;

/// - key: class index(e.g `1`)
/// - value: C++ Class
pub type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;
