//! Serialize/Deserialize ClassMap

use super::ClassMap;
#[cfg(feature = "extra_fmt")]
use crate::error::{JsonSnafu, YamlSnafu};
use crate::{
    convert::Format,
    error::{DeSnafu, Error, FailedReadFileSnafu, Result, SerSnafu},
};
use serde_hkx::{
    bytes::serde::hkx_header::HkxHeader, from_bytes, from_str, to_bytes, to_string, HavokSort,
};
use snafu::ResultExt as _;
use std::{io::Read, path::Path};

/// Serialize bytes(file contents) to a file.
pub async fn serialize_to_bytes<I>(
    input: I,
    format: Format,
    classes: &mut ClassMap<'_>,
) -> Result<Vec<u8>>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();

    // Serialize
    match format {
        Format::Xml => {
            let top_ptr = classes.sort_for_xml().context(SerSnafu {
                input: input.to_path_buf(),
            })?;
            let xml = to_string(classes, top_ptr).context(SerSnafu {
                input: input.to_path_buf(),
            })?;
            Ok(xml.into_bytes())
        }
        _ => {
            classes.sort_for_bytes();
            let binary_data = match format {
                Format::Win32 => to_bytes(classes, &HkxHeader::new_skyrim_le()),
                Format::Amd64 => to_bytes(classes, &HkxHeader::new_skyrim_se()),

                #[cfg(feature = "extra_fmt")]
                Format::Json | Format::Yaml => {
                    let contents = match format {
                        Format::Json => {
                            simd_json::to_string_pretty(&classes).context(JsonSnafu {
                                input: input.to_path_buf(),
                            })?
                        }
                        Format::Yaml => serde_yml::to_string(&classes).context(YamlSnafu {
                            input: input.to_path_buf(),
                        })?,
                        _ => unreachable!(),
                    };
                    return Ok(contents.into_bytes());
                }
                _ => unreachable!(),
            }
            .context(SerSnafu {
                input: input.to_path_buf(),
            })?;
            Ok(binary_data)
        }
    }
}

/// Deserialize bytes(file contents) to ClassMap.
/// - `string`: new String(To avoid XML ownership error.)
pub fn deserialize<'a, I>(
    bytes: &'a Vec<u8>,
    string: &'a mut String,
    input: I,
) -> Result<ClassMap<'a>>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();
    let input_ext = input.extension();

    // Deserialize
    let classes: ClassMap = if let Some(input_ext) = input_ext {
        let fmt =
            Format::from_extension(input_ext).map_err(|_| Error::UnsupportedExtensionPath {
                path: input.to_path_buf(),
            })?;

        match fmt {
            Format::Amd64 | Format::Win32 => from_bytes(bytes).context(DeSnafu {
                input: input.to_path_buf(),
            })?,

            _ => {
                let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
                decoder
                    .read_to_string(&mut *string)
                    .context(FailedReadFileSnafu {
                        path: input.to_path_buf(),
                    })?;

                match fmt {
                    Format::Xml => from_str(&*string).context(DeSnafu {
                        input: input.to_path_buf(),
                    })?,

                    #[cfg(feature = "extra_fmt")]
                    Format::Json => simd_json::from_slice(unsafe { string.as_bytes_mut() })
                        .context(JsonSnafu {
                            input: input.to_path_buf(),
                        })?,
                    #[cfg(feature = "extra_fmt")]
                    Format::Yaml => serde_yml::from_str(&string).context(YamlSnafu {
                        input: input.to_path_buf(),
                    })?,
                    _ => unreachable!(),
                }
            }
        }
    } else {
        return Err(Error::MissingExtension {
            path: input.to_path_buf(),
        });
    };

    Ok(classes)
}
