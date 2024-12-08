pub mod alt_map;
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

use havok_classes::Classes;
use indexmap::IndexMap;

/// - key: class index(e.g `1`)
/// - value: C++ Class
pub type ClassMap<'a> = IndexMap<usize, Classes<'a>>;

// https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#include-items-only-when-collecting-doctests

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
