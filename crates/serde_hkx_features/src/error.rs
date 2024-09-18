use std::{io, path::PathBuf};

/// Cli error
#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// The only supported extension is `.hkx` or `.xml`. But this path is neither: {path}.
    #[cfg(not(feature = "extra_fmt"))]
    #[snafu(display("The only supported extension is `.hkx` or `.xml`. But this path is neither: {}.", path.display()))]
    UnsupportedExtensionPath { path: PathBuf },
    #[cfg(feature = "extra_fmt")]
    /// The only supported extension is  `.hkx`, `.xml`, `.json`, `yaml`. But this path is neither: {path}.
    #[snafu(display("The only supported extension is `.hkx`, `.xml`, `.json`, `yaml`. But this path is neither: {}.", path.display()))]
    UnsupportedExtensionPath { path: PathBuf },

    /// The only supported extension is `.hkx` or `.xml`. But this is neither: {ext}.
    #[cfg(not(feature = "extra_fmt"))]
    UnsupportedExtension { ext: String },
    #[cfg(feature = "extra_fmt")]
    /// The only supported extension is  `.hkx`, `.xml`, `.json`, `yaml`. But this is neither: {ext}.
    UnsupportedExtension { ext: String },

    /// This path has a missing extension.
    #[snafu(display("This path has a missing extension.: {}.", path.display()))]
    MissingExtension { path: PathBuf },

    /// Failed to read file from
    #[snafu(display("{source}: {}", path.display()))]
    FailedReadFile { source: io::Error, path: PathBuf },

    /// Please specify the output path.(OS cannot output bytes as stdout.)
    InvalidStdout,

    /// Serialize error
    #[snafu(display("{}:\n {source}", input.display()))]
    SerError {
        input: PathBuf,
        source: serde_hkx::errors::ser::Error,
    },

    /// Deserialize error
    #[snafu(display("{}:\n {source}", input.display()))]
    DeError {
        input: PathBuf,
        source: serde_hkx::errors::de::Error,
    },

    /// Standard io error
    #[snafu(transparent)]
    IoError { source: std::io::Error },

    /// dir strip error
    #[snafu(transparent)]
    StripPrefixError { source: std::path::StripPrefixError },

    /// jwalk path error
    #[snafu(transparent)]
    JwalkError { source: jwalk::Error },

    /// Tracing log error
    #[cfg(feature = "tracing")]
    #[snafu(transparent)]
    TracingError {
        source: tracing::subscriber::SetGlobalDefaultError,
    },

    #[snafu(transparent)]
    FailedThreadJoin { source: tokio::task::JoinError },

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Extra formats
    /// (De)Serialize json error
    #[cfg(feature = "extra_fmt")]
    #[snafu(display("{}:\n {source}", input.display()))]
    JsonError {
        input: PathBuf,
        source: simd_json::Error,
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
pub type Result<T, E = Error> = core::result::Result<T, E>;
