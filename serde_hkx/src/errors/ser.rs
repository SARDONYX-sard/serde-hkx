//! Serialize error
use crate::lib::*;
use havok_types::Pointer;

/// Serialize error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// {msg}
    Message {
        /// Error message
        msg: String,
    },

    /// Only 0 (big) or 1 (little) can be specified for the header endian. But got {invalid}
    InvalidEndian {
        invalid: u8,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// The only supported pointer sizes are 4 and 8. But got {invalid}
    UnsupportedPtrSize {
        invalid: u8,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Relative position cannot be obtained because abs is larger than {position}.
    /// This indicates that the value of `absolute_data_offset` in the header is wrong.
    OverflowSubtractAbs { position: u32, abs_data_offset: u32 },

    /// Missing global fixup class: {ptr}
    MissingGlobalFixupClass {
        /// missing global fixup class ptr(e.g. #0050)
        ptr: Pointer<'static>,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// The constructor class for virtual_fixup did not exist in the class
    /// The constructor class for virtual_fixup did not exist in the class
    /// in the `__classnames__` section written.: {class_name}
    MissingClassInClassnamesSection {
        class_name: &'static str,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Not found where to write ptr. This could be an incorrect value inside hkx or a mistake by the library implementor.
    NotFoundPointedPosition,

    /// Variable `{event_id}` for Nemesis is used, but its name could not be found in `hkbBehaviorGraphStringData`.`eventNames` in hkx, so the variable could not be replaced
    NotFoundEventId {
        event_id: String,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Variable `{variable_id}` for Nemesis is used, but its name could not be found in `hkbBehaviorGraphStringData`.`variableNames` in hkx, so the variable could not be replaced
    NotFoundVariableId {
        variable_id: String,
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

    /// std io error.
    #[snafu(transparent)]
    IoError {
        /// I/O Error
        source: std::io::Error,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// The state machine in behavior is topologically sorted circularly referenced.
    #[snafu(display(
        "The state machine in behavior is topologically sorted circularly referenced."
    ))]
    UnexpectedCyclicSort {
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

/// Wrapper on [`core::result::Result`] for Serializer.
pub type Result<T, E = Error> = core::result::Result<T, E>;
