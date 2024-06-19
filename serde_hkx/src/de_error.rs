//! crate Root error
use crate::lib::*;

/// Crate root error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum DeError {
    /// User custom error.
    #[snafu(display(""))]
    Message {
        /// Error message
        msg: String,
    },

    #[snafu(display("Still need to parse the syntax but the string provided is not enough."))]
    Eof,
    #[snafu(display("Incomplete parsing of syntax."))]
    TrailingCharacters,

    #[snafu(display("Expected class {expected}, but got {actual}."))]
    MismatchClassName {
        actual: &'static str,
        expected: String,
    },

    #[snafu(transparent)]
    ReadableError {
        source: crate::xml::de::parser::error::ReadableError,
    },
}

impl havok_serde::de::Error for DeError {
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
pub type Result<T, E = DeError> = core::result::Result<T, E>;
