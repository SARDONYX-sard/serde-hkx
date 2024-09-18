pub mod convert;
pub mod diff;
pub mod dump;
pub mod error;
pub mod read_ext;
pub mod tree;
mod toml_err_avoider;

pub(crate) type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;
