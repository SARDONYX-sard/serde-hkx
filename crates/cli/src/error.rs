#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
pub enum ConvertError {
    /// Unknown extension of file: {path}
    #[snafu(display("Unknown extension of file: {path}"))]
    UnknownExtension { path: String },

    #[snafu(transparent)]
    DeError {
        source: serde_hkx::errors::de::Error,
    },
    #[snafu(transparent)]
    SerError {
        source: serde_hkx::errors::ser::Error,
    },

    #[snafu(transparent)]
    IoError { source: std::io::Error },

    #[snafu(transparent)]
    StripPrefixError { source: std::path::StripPrefixError },

    #[snafu(transparent)]
    JwalkError { source: jwalk::Error },

    #[snafu(transparent)]
    TracingError {
        source: tracing::subscriber::SetGlobalDefaultError,
    },
}

pub type Result<T> = core::result::Result<T, ConvertError>;
