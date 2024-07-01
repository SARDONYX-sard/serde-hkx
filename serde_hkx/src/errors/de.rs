//! Deserialize error
use crate::lib::*;

/// Deserialize error
#[derive(Debug, PartialEq, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// User custom error.
    #[snafu(display("{msg}"))]
    Message {
        /// Error message
        msg: String,
    },

    /// Incomplete parsing binary.
    #[snafu(display("Index to hold processing deserialization status of struct is not found."))]
    NotFoundIndex,

    /// Incomplete parsing binary.
    #[snafu(display("Incomplete parsing binary. Remain: {remain:?}"))]
    TrailingBytes { remain: Vec<u8> },

    /// Still need to parse the syntax but the string provided is not enough.
    #[snafu(display("Still need to parse the syntax but the string provided is not enough."))]
    Eof,

    /// Incomplete parsing of XML.
    #[snafu(display("Incomplete parsing XML. Remain: {remain}"))]
    TrailingCharacters { remain: String },

    /// Expected class {expected}, but got {actual}.
    #[snafu(display("Expected class {expected}, but got {actual}."))]
    MismatchClassName {
        actual: &'static str,
        expected: String,
    },

    /// Human readable XML parsing error
    #[snafu(transparent)]
    ReadableError {
        source: super::readable::ReadableError,
    },

    /// Contain null bytes in a string error
    #[snafu(transparent)]
    FromBytesWithNulError {
        /// Contain null bytes in a string error
        source: std::ffi::FromBytesWithNulError,
    },
}

impl havok_serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Message {
            msg: msg.to_string(),
        }
    }
}

/// Wrapper on [`core::result::Result`] for Deserializer.
pub type Result<T, E = Error> = core::result::Result<T, E>;
