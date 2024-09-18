pub mod convert;
pub mod diff;
pub mod dump;
pub mod error;
pub mod read_ext;
mod toml_err_avoider;
pub mod tree;

pub(crate) type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;
