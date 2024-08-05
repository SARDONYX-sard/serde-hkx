/// Cli error
#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// The only supported extension is `.hkx` or `.xml`. But this path is neither: {path}.
    #[snafu(display(
        "The only supported extension is `.hkx` or `.xml`. But this path is neither: {path}."
    ))]
    UnknownExtension { path: String },

    /// Invalid format: {unknown_fmt}
    #[snafu(display("Invalid format: {unknown_fmt}"))]
    InvalidOutputFormat { unknown_fmt: String },

    /// Deserialize error
    #[snafu(transparent)]
    DeError {
        source: serde_hkx::errors::de::Error,
    },

    /// Serialize error
    #[snafu(transparent)]
    SerError {
        source: serde_hkx::errors::ser::Error,
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

pub type Result<T> = core::result::Result<T, Error>;
