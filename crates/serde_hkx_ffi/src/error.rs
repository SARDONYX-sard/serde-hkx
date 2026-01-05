/// Error codes returned by serde_hkx FFI APIs.
///
/// This enum is a *lossy projection* of `serde_hkx_features::Error`.
/// Detailed information such as file paths or diff contents is not preserved.
/// Those details are expected to be handled on the caller (C/C++) side.
///
/// The numeric values are ABI-stable.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerdeHkxError {
    /// Success.
    Ok = 0,

    /// Invalid argument was provided.
    ///
    /// - Missing extension
    /// - Unsupported extension
    /// - Invalid stdout usage
    /// - Invalid key format
    InvalidArgument,

    /// Unsupported input or output format.
    UnsupportedFormat,

    /// I/O error occurred while reading or writing files.
    IoError,

    /// Failed to walk directories or strip paths.
    WalkError,

    /// Serialization (hkx → other formats) failed.
    SerializeError,

    /// Deserialization (other formats → hkx) failed.
    DeserializeError,

    /// Conversion failed for one or more files.
    ConvertError,

    /// Reproduction failed (diff mismatch).
    ReproduceError,

    /// Threading or async task failed.
    ThreadError,

    /// Error from optional or external components.
    ExternalError,

    /// Internal error or panic.
    InternalError,
}

impl From<serde_hkx_features::error::Error> for SerdeHkxError {
    fn from(e: serde_hkx_features::error::Error) -> Self {
        use serde_hkx_features::error::Error::*;

        match e {
            // ----------------------------
            // Argument / format
            // ----------------------------
            UnsupportedExtensionPath { .. }
            | UnsupportedExtension { .. }
            | MissingExtension { .. }
            | InvalidStdout
            | KeyParse { .. } => Self::InvalidArgument,

            // ----------------------------
            // IO / filesystem
            // ----------------------------
            FailedReadFile { .. } | IoError { .. } | StripPrefixError { .. } => Self::IoError,

            JwalkError { .. } => Self::WalkError,

            // ----------------------------
            // Convert / reproduce
            // ----------------------------
            FailedConvertFiles { .. } => Self::ConvertError,
            FailedReproduceFile { .. } | FailedReproduceFiles { .. } => Self::ReproduceError,

            // ----------------------------
            // Serde
            // ----------------------------
            SerError { .. } => Self::SerializeError,
            DeError { .. } => Self::DeserializeError,

            // ----------------------------
            // Thread / runtime
            // ----------------------------
            FailedThreadJoin { .. } => Self::ThreadError,
            // // ----------------------------
            // // Optional / external
            // // ----------------------------
            // #[cfg(feature = "extra_fmt")]
            // ExtraSerdeError { .. } => Self::ExternalError,

            // #[cfg(feature = "json_schema")]
            // JsonError { .. } => Self::ExternalError,

            // #[cfg(feature = "tracing")]
            // TracingError { .. } => Self::InternalError,
        }
    }
}
