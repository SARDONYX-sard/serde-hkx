pub mod convert;
pub mod diff;
pub mod dump;
pub mod error;
pub mod fs;
pub mod serde;
pub mod tree;

/// - key: class index(e.g `#0001`)
/// - value: C++ Class
pub type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;
