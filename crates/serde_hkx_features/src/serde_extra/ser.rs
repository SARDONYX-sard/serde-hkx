//! Serialize/Deserialize ClassMap with extra formats.
use super::error::{JsonSnafu, TomlSnafu, YamlSnafu};
use crate::convert::Format;
use crate::error::Result;
use crate::types_wrapper::ClassPtrMap;
use snafu::ResultExt as _;
use std::path::Path;

/// Serialize bytes(file contents) to a file.
///
/// # Errors
/// If the information required for serialization is missing.
///
/// See `serde_hkx::errors::ser::Error` for possible errors that may occur.
///
/// # Panics
/// If the `Format` is not `json`, `toml` or `yaml`.
/// That means the API is being used incorrectly.
pub fn to_bytes<I>(input: I, format: Format, classes: &mut ClassPtrMap<'_>) -> Result<Vec<u8>>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();

    let contents = match format {
        Format::Json => simd_json::to_string_pretty(&classes).with_context(|_| JsonSnafu {
            input: input.to_path_buf(),
        })?,
        Format::Toml => basic_toml::to_string(&classes).with_context(|_| TomlSnafu {
            input: input.to_path_buf(),
        })?,
        Format::Yaml => serde_norway::to_string(&classes).with_context(|_| YamlSnafu {
            input: input.to_path_buf(),
        })?,
        _ => unreachable!(),
    };
    Ok(contents.into_bytes())
}
