/// Crate root error
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{reason}"))]
    ParseError {
        /// Parse error reason
        reason: String,
        /// error location
        #[snafu(implicit)]
        location: snafu::Location,
    },
}

/// Wrapper on [`core::result::Result`] for havok_types.
pub type Result<T, E = Error> = core::result::Result<T, E>;
