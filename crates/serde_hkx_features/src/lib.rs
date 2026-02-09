pub mod alt_map;
pub mod convert;
pub mod diff;
pub mod dump;
pub mod error;
pub mod fs;
#[cfg(feature = "json_schema")]
pub mod json_schema_gen;
pub mod progress;
pub mod serde;
#[cfg(feature = "extra_fmt")]
pub mod serde_extra;
pub mod tree;
#[cfg(any(feature = "extra_fmt", feature = "json_schema"))]
pub mod types_wrapper;
pub mod verify;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Module to allow import of the most used and useful APIs in one place.
// NOTE: Keep it to top-level public because it is not easy to use if prelude as `Result`.
pub use crate::convert::{Format, tokio::convert};
pub use crate::error::Result;
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use havok_classes::Classes;
use indexmap::IndexMap;

/// - key: class index(e.g `1`)
/// - value: C++ Class
pub type ClassMap<'a> = IndexMap<usize, Classes<'a>>;

// https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#include-items-only-when-collecting-doctests

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
