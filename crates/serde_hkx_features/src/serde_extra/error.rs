//! `serde_hkx_features`(`extra_fmt`) errors
use snafu::Location;
use std::path::PathBuf;

/// Cli error
#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum ExtraSerdeError {
    // Extra formats
    /// (De)Serialize json error
    #[cfg(any(feature = "extra_fmt", feature = "json_schema"))]
    #[snafu(display("{}:\n {source}", input.display()))]
    JsonError {
        input: PathBuf,
        source: simd_json::Error,
        #[snafu(implicit)]
        location: Location,
    },

    /// (De)Serialize toml error
    #[cfg(feature = "extra_fmt")]
    #[snafu(display("{}:\n {source}", input.display()))]
    TomlError {
        input: PathBuf,
        source: basic_toml::Error,
        #[snafu(implicit)]
        location: Location,
    },

    /// (De)Serialize yaml error
    #[cfg(feature = "extra_fmt")]
    #[snafu(display("{}:\n {source}", input.display()))]
    YamlError {
        input: PathBuf,
        source: serde_norway::Error,
        #[snafu(implicit)]
        location: Location,
    },
}
