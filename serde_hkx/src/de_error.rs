//! crate Root error
use crate::lib::*;

/// Crate root error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum DeError {
    /// User custom error.
    Message {
        /// Error message
        msg: String,
    },

    Eof,
    ExpectedArray,
    ExpectedArrayEnd,
    ExpectedArraySpace,
    ExpectedArrayStringEndTag,
    ExpectedArrayStringStartTag,

    ExpectedBoolean,
    ExpectedInteger,
    ExpectedString,
    TrailingCharacters,

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

/// Wrapper on [`core::result::Result`] for Havok Serde.
pub type Result<T, E = DeError> = core::result::Result<T, E>;
