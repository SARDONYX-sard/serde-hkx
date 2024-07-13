//! Deserialize error
use crate::lib::*;

/// Deserialize error
#[derive(Debug, PartialEq, snafu::Snafu)]
pub enum Error {
    /// User custom error.
    #[snafu(display("{msg}"))]
    Message {
        /// Error message
        msg: String,
    },

    /// Unique index of class for this `global_fix.dst`{virtual_src} is missing.
    #[snafu(display(
        "Unique index of class for this `global_fix.dst`(virtual_src): {global_dst} is missing."
    ))]
    NotFoundClassIndex { global_dst: u32 },

    /// The data position pointed to by the pointer of the read position ({key}) is not found in local_fixups.
    #[snafu(display("The data position pointed to by the pointer of the read position ({key}) is not found in local_fixups."))]
    NotFoundDataLocalFixupsValue { key: u32 },

    /// The data position pointed to by the pointer of the read position ({key}) is not found in global_fixups.
    #[snafu(display("The data position pointed to by the pointer of the read position ({key}) is not found in global_fixups."))]
    NotFoundDataGlobalFixupsValue { key: u32 },

    /// The data position pointed to by the pointer of the read position ({key}) is not found in virtual_fixups.
    #[snafu(display("Couldn't find class by this name_offset: {start_offset}."))]
    NotFoundClass { start_offset: u32 },

    /// Incomplete parsing binary.
    #[snafu(display("Incomplete parsing binary."))]
    TrailingBytes,

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
