//! Deserialize ClassMap

/// Serialize Error
#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum DeError {
    /// {location}: hkx Serialize Error: {source}
    Hkx {
        source: serde_hkx::errors::de::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },
    /// {location}: XML Deserialize Error: {source}
    Xml {
        source: serde_hkx::errors::de::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// {location}: Json Deserialize Error: {source}
    #[cfg(any(feature = "extra_fmt", feature = "json_schema"))]
    Json {
        source: sonic_rs::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// {location}: Toml Serialize Error: {source}
    #[cfg(feature = "extra_fmt")]
    Toml {
        source: basic_toml::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// {location}: Yaml Serialize Error: {source}
    #[cfg(feature = "extra_fmt")]
    Yaml {
        source: serde_norway::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },
}
