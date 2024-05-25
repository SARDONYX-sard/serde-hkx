//! crate Root error
use crate::lib::*;

/// Crate root error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// User custom error.
    Message {
        /// Error message
        msg: String,
    },

    /// Invalid utf8 error
    #[snafu(transparent)]
    Utf8Error {
        /// Invalid utf8 error
        source: std::str::Utf8Error,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Contain null bytes in a string error
    #[snafu(transparent)]
    NulError {
        /// Contain null bytes in a string error
        source: std::ffi::NulError,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl havok_serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Message {
            msg: msg.to_string(),
        }
    }
}

/// Wrapper on [`core::result::Result`] for Havok Serde.
pub type Result<T, E = Error> = core::result::Result<T, E>;
