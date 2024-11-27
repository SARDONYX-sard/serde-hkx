//! crate Root error

/// Crate root error
#[derive(Debug, snafu::Snafu)]
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
    SynError {
        source: syn::Error,
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

/// Create `syn::Error`
macro_rules! syn_error {
    ($span:expr, $($msg:tt)*) => {
        syn::Error::new($span, format!($($msg)*))
    };

    ($($msg:tt)*) => {
        syn::Error::new(proc_macro2::Span::call_site(), format!($($msg)*))
    };
}
pub(crate) use syn_error;

/// early return `syn::Error`
macro_rules! bail_syn_err {
    ($span:expr, $($msg:tt)*) => {
        return Err(syn::Error::new($span, format!($($msg)*)))
    };
    ($($msg:tt)*) => {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), format!($($msg)*)))
    };
}
pub(crate) use bail_syn_err;
