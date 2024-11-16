//! `serde_hkx_features`(`extra_fmt`) errors
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
    },
    /// (De)Serialize yaml error
    #[cfg(feature = "extra_fmt")]
    #[snafu(display("{}:\n {source}", input.display()))]
    TomlSerError {
        input: PathBuf,
        source: toml::ser::Error,
    },
    /// (De)Serialize yaml error
    #[cfg(feature = "extra_fmt")]
    #[snafu(display("{}:\n {source}", input.display()))]
    TomlDeError {
        input: PathBuf,
        source: Box<toml::de::Error>,
    },
    /// (De)Serialize yaml error
    #[cfg(feature = "extra_fmt")]
    #[snafu(display("{}:\n {source}", input.display()))]
    YamlError {
        input: PathBuf,
        source: serde_yml::Error,
    },
}

/// `Result` for `serde_hkx` wrapper crate.
pub type Result<T, E = ExtraSerdeError> = core::result::Result<T, E>;
