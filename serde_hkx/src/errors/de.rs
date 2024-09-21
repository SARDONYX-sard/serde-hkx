//! Deserialize error
use crate::lib::*;

/// Deserialize error
#[derive(Debug, PartialEq, snafu::Snafu)]
pub enum Error {
    /// {msg}
    Message {
        /// Error message
        msg: String,
    },

    /// The number of required constructors in C++ is insufficient. actual: {actual}, expected: {expected}
    LackOfConstructors { actual: usize, expected: usize },

    /// The data position pointed to by the pointer of the read position ({key}) is not found in local_fixups.
    NotFoundDataLocalFixupsValue { key: u32 },

    /// Could not find the {index}th corresponding class: {start_offset}
    NotFoundClass { index: usize, start_offset: u32 },

    /// Class Ptr is None.
    NotFoundClassPtr,

    /// Incomplete parsing binary.
    TrailingBytes,

    /// Still need to parse the syntax but the string provided is not enough.
    Eof,

    /// Incomplete parsing XML. Remain: {remain}
    TrailingCharacters { remain: String },

    /// Expected class {expected}, but got {actual}.
    MismatchClassName {
        actual: &'static str,
        expected: String,
    },

    /// Parser combinator Error
    #[snafu(display("{err}"))]
    ContextError {
        err: winnow::error::ErrMode<winnow::error::ContextError>,
    },

    /// Human readable XML parsing error
    #[snafu(transparent)]
    ReadableError {
        source: super::readable::ReadableError,
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
