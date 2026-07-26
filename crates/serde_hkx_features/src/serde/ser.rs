//! Serialize/Deserialize ClassMap
use crate::ClassMap;
use crate::{convert::Format, error::Result};
use serde_hkx::{HavokSort as _, bytes::serde::hkx_header::HkxHeader};
use snafu::ResultExt as _;

/// Serialize bytes(file contents) to a file.
///
/// # Errors
/// If the information required for serialization is missing.
///
/// See `serde_hkx::errors::ser::Error` for possible errors that may occur.
pub fn to_bytes(classes: &mut ClassMap<'_>, output_format: Format) -> Result<Vec<u8>, SerError> {
    match output_format {
        Format::Win32 => to_bytes_inner(classes, &HkxHeader::new_skyrim_le()),
        Format::Amd64 => to_bytes_inner(classes, &HkxHeader::new_skyrim_se()),
        Format::Xml => {
            let top_ptr = classes.sort_for_xml().context(XmlSnafu {})?;
            let xml = serde_hkx::to_string(classes, top_ptr).context(XmlSnafu {})?;
            Ok(xml.into_bytes())
        }

        #[cfg(feature = "extra_fmt")]
        Format::Json => {
            classes.sort_for_bytes().context(HkxSnafu {})?;
            let json = sonic_rs::to_string_pretty(&classes).context(JsonSnafu {})?;
            Ok(json.into_bytes())
        }
        #[cfg(feature = "extra_fmt")]
        Format::Toml => {
            classes.sort_for_bytes().context(HkxSnafu {})?;
            let toml = basic_toml::to_string(&classes).context(TomlSnafu {})?;
            Ok(toml.into_bytes())
        }
        #[cfg(feature = "extra_fmt")]
        Format::Yaml => {
            classes.sort_for_bytes().context(HkxSnafu {})?;
            let toml = serde_norway::to_string(&classes).context(YamlSnafu {})?;
            Ok(toml.into_bytes())
        }
    }
}

fn to_bytes_inner<'a>(classes: &mut ClassMap<'a>, header: &HkxHeader) -> Result<Vec<u8>, SerError> {
    classes.sort_for_bytes().context(HkxSnafu {})?;
    serde_hkx::to_bytes(classes, header).context(HkxSnafu {})
}

/// Serialize Error
#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum SerError {
    /// {location}: hkx Serialize Error: {source}
    Hkx {
        source: serde_hkx::errors::ser::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },
    /// {location}: XML Serialize Error: {source}
    Xml {
        source: serde_hkx::errors::ser::Error,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// {location}: Json Serialize Error: {source}
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
