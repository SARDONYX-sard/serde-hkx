//! Serialize/Deserialize ClassMap with extra formats.
use crate::types_wrapper::ClassPtrMap;
use crate::{
    convert::OutFormat,
    error::{JsonSnafu, Result, TomlSerSnafu, YamlSnafu},
};
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
/// If the `OutFormat` is not `json`, `toml` or `yaml`.
/// That means the API is being used incorrectly.
pub fn to_bytes<I>(input: I, format: OutFormat, classes: &mut ClassPtrMap<'_>) -> Result<Vec<u8>>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();

    let contents = match format {
        OutFormat::Json => simd_json::to_string_pretty(&classes).context(JsonSnafu {
            input: input.to_path_buf(),
        })?,
        OutFormat::Toml => toml::to_string_pretty(&classes).context(TomlSerSnafu {
            input: input.to_path_buf(),
        })?,
        OutFormat::Yaml => serde_yml::to_string(&classes).context(YamlSnafu {
            input: input.to_path_buf(),
        })?,
        _ => unreachable!(),
    };
    Ok(contents.into_bytes())
}
