//! Serialize/Deserialize ClassMap
use crate::ClassMap;
use crate::{
    convert::OutFormat,
    error::{Result, SerSnafu},
};
use serde_hkx::{HavokSort, bytes::serde::hkx_header::HkxHeader, to_string};
use snafu::ResultExt as _;
use std::path::Path;

/// Serialize bytes(file contents) to a file.
///
/// # Errors
/// If the information required for serialization is missing.
///
/// See `serde_hkx::errors::ser::Error` for possible errors that may occur.
///
/// # Panics
/// If `format` is not `XML`, `Amd64`, or `WIn32`.
/// That means the API is being used incorrectly.
pub fn to_bytes<I>(input: I, format: OutFormat, classes: &mut ClassMap<'_>) -> Result<Vec<u8>>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();

    // Serialize
    if format == OutFormat::Xml {
        let top_ptr = classes.sort_for_xml().with_context(|_| SerSnafu {
            input: input.to_path_buf(),
        })?;
        let xml = to_string(classes, &top_ptr).with_context(|_| SerSnafu {
            input: input.to_path_buf(),
        })?;
        Ok(xml.into_bytes())
    } else {
        classes.sort_for_bytes();
        let binary_data = match format {
            OutFormat::Win32 => serde_hkx::to_bytes(classes, &HkxHeader::new_skyrim_le()),
            OutFormat::Amd64 => serde_hkx::to_bytes(classes, &HkxHeader::new_skyrim_se()),
            _ => unreachable!(),
        }
        .with_context(|_| SerSnafu {
            input: input.to_path_buf(),
        })?;
        Ok(binary_data)
    }
}
