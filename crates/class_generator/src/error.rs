//! crate Root error

/// Crate root error
#[derive(Debug, snafu::Snafu, snafu_location_derive::Location)]
#[snafu(visibility(pub))]
pub enum ClassGeneratorError {
    /// File meta-information not found error
    #[snafu(display("Not found file stem: {path}"))]
    NotFoundFileStem {
        path: String,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Directory walker error
    #[snafu(transparent)]
    JWalkError {
        source: jwalk::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Standard library io error
    #[snafu(transparent)]
    IoError {
        source: std::io::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Json (De)Serialization error
    #[snafu(transparent)]
    SerdeJsonError {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },
}

/// Wrapper on [`core::result::Result`] for code generator.
pub type Result<T, E = ClassGeneratorError> = core::result::Result<T, E>;
