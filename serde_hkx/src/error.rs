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

    /// Relative position cannot be obtained because abs is larger than {position}.
    /// This indicates that the value of `absolute_data_offset` in the header is wrong.
    SubAbsOverflowError {
        position: u64,
        abs_data_offset: u32,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(display("Missing global fixup class: {ptr}"))]
    MissingGlobalFixupClass {
        /// missing global fixup class ptr(e.g. #0050)
        ptr: String,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// The constructor class for virtual_fixup did not exist in the class
    /// in the `__classnames__` section written.
    #[snafu(display("The constructor class for virtual_fixup did not exist in the class in the `__classnames__` section written.: {class}"))]
    MissingClassInClassnamesSection {
        class: &'static str,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
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

    #[snafu(transparent)]
    IoError {
        /// I/O Error
        source: std::io::Error,
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
