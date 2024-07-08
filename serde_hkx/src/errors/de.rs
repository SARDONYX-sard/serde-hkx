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

    /// This field should have a default value since serialization is skipped.
    #[snafu(display("This field should have a default value since serialization is skipped."))]
    SkipField,

    #[snafu(display("Unique index of class for this `global_fix.dst`{virtual_src} is missing"))]
    NotFoundClassIndex { virtual_src: u32 },

    /// Classnames section fixups were not found in the binary data.
    #[snafu(display("Classnames section fixups were not found in the binary data."))]
    NotFoundClassNamesFixups,

    /// Data section fixups were not found in the binary data.
    #[snafu(display("Data section fixups were not found in the binary data."))]
    NotFoundDataFixups,

    /// The data position pointed to by the pointer of the read position ({key}) is not found in local_fixups.
    #[snafu(display("The data position pointed to by the pointer of the read position ({key}) is not found in local_fixups."))]
    NotFoundDataLocalFixupsValue { key: u32 },

    /// The data position pointed to by the pointer of the read position ({key}) is not found in global_fixups.
    #[snafu(display("The data position pointed to by the pointer of the read position ({key}) is not found in global_fixups."))]
    NotFoundDataGlobalFixupsValue { key: u32 },

    /// The data position pointed to by the pointer of the read position ({key}) is not found in virtual_fixups.
    #[snafu(display("The data position pointed to by the pointer of the read position ({key}) is not found in virtual_fixups."))]
    NotFoundDataVirtualFixupsValue { key: u32 },

    /// The number of key calls ({actual}) must not be more than the length of the field ({expected}).
    #[snafu(display("The number of key calls ({actual}) must not be more than the length of the field ({expected})."))]
    OverFlowIndex { actual: usize, expected: usize },

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
