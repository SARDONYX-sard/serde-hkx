//! Deserialize ClassMap with extra formats.

use super::error::{JsonSnafu, TomlSnafu, YamlSnafu};
use crate::types_wrapper::ClassPtrMap;
use crate::{
    ClassMap,
    convert::OutFormat,
    error::{DeSnafu, Error, FailedReadFileSnafu, Result},
};
use snafu::ResultExt as _;
use std::{io::Read, path::Path};

/// Deserialize bytes(file contents) to ClassMap.
/// - `string`: new String(To avoid XML ownership error.)
///
/// # Errors
/// - Missing extension.
/// - If the input extension is not `hkx`, `xml` `json`, `toml` or `yaml`.
/// - Incorrect syntax in the input path file.(`DeError`)
///
/// See `serde_hkx::errors::de::Error` for possible errors that may occur.
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
    let classes = if let Some(input_ext) = input_ext {
        let input_fmt =
            OutFormat::from_extension(input_ext).map_err(|_| Error::UnsupportedExtensionPath {
                path: input.to_path_buf(),
            })?;

        match input_fmt {
            OutFormat::Amd64 | OutFormat::Win32 => {
                serde_hkx::from_bytes(bytes).context(DeSnafu {
                    input: input.to_path_buf(),
                })?
            }

            _ => {
                let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
                decoder
                    .read_to_string(&mut *string)
                    .context(FailedReadFileSnafu {
                        path: input.to_path_buf(),
                    })?;

                match input_fmt {
                    OutFormat::Xml => serde_hkx::from_str(&*string).context(DeSnafu {
                        input: input.to_path_buf(),
                    })?,
                    OutFormat::Json => {
                        let classes =
                            simd_json::from_slice::<ClassPtrMap>(unsafe { string.as_bytes_mut() })
                                .with_context(|_| JsonSnafu {
                                    input: input.to_path_buf(),
                                })?;
                        classes.into_class_map()
                    }
                    OutFormat::Toml => {
                        let classes =
                            basic_toml::from_str::<ClassPtrMap>(string).with_context(|_| {
                                TomlSnafu {
                                    input: input.to_path_buf(),
                                }
                            })?;
                        classes.into_class_map()
                    }
                    OutFormat::Yaml => {
                        let classes =
                            serde_yml::from_str::<ClassPtrMap>(string).with_context(|_| {
                                YamlSnafu {
                                    input: input.to_path_buf(),
                                }
                            })?;
                        classes.into_class_map()
                    }
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
