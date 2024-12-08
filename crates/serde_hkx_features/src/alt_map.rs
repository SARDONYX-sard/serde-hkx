use crate::error::{KeyParseSnafu, Result};
use crate::ClassMap;
use havok_classes::Classes;
use indexmap::IndexMap;
use rayon::prelude::*;
use snafu::ResultExt as _;
use std::borrow::Cow;

/// - key: class index(e.g `"1"`)
/// - value: C++ Class
pub type ClassMapAlt<'a> = IndexMap<Cow<'a, str>, Classes<'a>>;

/// Converts a `ClassMapAlt` to a `ClassMap` if all keys can be parsed into [`usize`].
///
/// # Errors
/// - If the first key that failed to parse.
pub fn convert_to_usize_keys(input: ClassMapAlt<'_>) -> Result<ClassMap<'_>> {
    input
        .into_par_iter()
        .map(|(key, value)| {
            key.parse::<usize>()
                .map(|parsed_key| (parsed_key, value))
                .with_context(|_| KeyParseSnafu {
                    key: key.into_owned(),
                })
        })
        .collect()
}

/// Converts a `ClassMap` to a `ClassMapAlt` by converting integer keys to strings.
pub fn convert_to_string_keys(input: ClassMap<'_>) -> ClassMapAlt<'_> {
    input
        .into_par_iter()
        .map(|(key, value)| (Cow::Owned(key.to_string()), value))
        .collect()
}
