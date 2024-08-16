use std::{io, path::PathBuf};

/// Cli error
#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// The only supported extension is `.hkx` or `.xml`. But this path is neither: {path}.
    #[snafu(display("The only supported extension is `.hkx` or `.xml`. But this path is neither: {}.", path.display()))]
    UnsupportedExtension { path: PathBuf },

    /// This path has a missing extension.: {path}.
    #[snafu(display("This path has a missing extension.: {}.", path.display()))]
    MissingExtension { path: PathBuf },

    /// Failed to read file from
    #[snafu(display("{source}: {}", path.display()))]
    FailedReadFile { source: io::Error, path: PathBuf },

    #[snafu(display("Use `-o [FILE]` option. (Unable to write bytes to stdout.)"))]
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
    #[snafu(transparent)]
    TracingError {
        source: tracing::subscriber::SetGlobalDefaultError,
    },

    #[snafu(transparent)]
    FailedThreadJoin { source: tokio::task::JoinError },
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
