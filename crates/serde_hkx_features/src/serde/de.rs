//! Deserialize ClassMap
#[cfg(feature = "extra_fmt")]
use crate::error::UnsupportedExtensionPathSnafu;
use crate::{
    convert::OutFormat,
    error::{DeSnafu, Error, FailedReadFileSnafu, Result},
};
use serde_hkx::{from_bytes, from_str};
use snafu::ResultExt as _;
use std::{io::Read, path::Path};

/// Deserialize bytes(file contents) to ClassMap.
/// - `string`: new String(To avoid XML ownership error.)
///
/// # Errors
/// - Missing extension.
/// - If the input extension is not `hkx` or `xml`.
/// - Incorrect syntax in the input path file.(`DeError`)
///
/// See `serde_hkx::errors::de::Error` for possible errors that may occur.
pub fn deserialize<'a, I, T>(bytes: &'a Vec<u8>, string: &'a mut String, input: I) -> Result<T>
where
    I: AsRef<Path>,
    T: havok_serde::Deserialize<'a>,
{
    let input = input.as_ref();
    let input_ext = input.extension();

    // Deserialize
    let classes = if let Some(input_ext) = input_ext {
        let fmt =
            OutFormat::from_extension(input_ext).map_err(|_| Error::UnsupportedExtensionPath {
                path: input.to_path_buf(),
            })?;

        match fmt {
            OutFormat::Amd64 | OutFormat::Win32 => from_bytes(bytes).context(DeSnafu {
                input: input.to_path_buf(),
            })?,

            OutFormat::Xml => {
                let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
                decoder
                    .read_to_string(&mut *string)
                    .context(FailedReadFileSnafu {
                        path: input.to_path_buf(),
                    })?;
                from_str(&*string).context(DeSnafu {
                    input: input.to_path_buf(),
                })?
            }
            #[cfg(feature = "extra_fmt")]
            _ => {
                return UnsupportedExtensionPathSnafu {
                    path: input.to_path_buf(),
                }
                .fail();
            }
        }
    } else {
        return Err(Error::MissingExtension {
            path: input.to_path_buf(),
        });
    };

    Ok(classes)
}
