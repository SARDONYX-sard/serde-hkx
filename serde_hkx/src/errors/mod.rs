//! (De)Serialize errors
pub mod de;
pub mod readable;
pub mod ser;

/// This crate errors.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
pub enum SerdeHkxError {
    #[snafu(transparent)]
    DeError { source: crate::errors::de::Error },
    #[snafu(transparent)]
    SerError { source: crate::errors::ser::Error },

    #[snafu(transparent)]
    IoError { source: std::io::Error },
}
