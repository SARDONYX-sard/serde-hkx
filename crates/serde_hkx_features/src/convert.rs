//! Convert hkx <-> xml
use super::ClassMap;
#[cfg(feature = "extra_fmt")]
use crate::error::{JsonSnafu, TomlSerSnafu, YamlSnafu};
use crate::{
    error::{DeSnafu, Error, FailedReadFileSnafu, Result, SerSnafu},
    read_ext::ReadExt as _,
};
use core::str::FromStr;
use parse_display::Display;
use serde_hkx::{
    bytes::serde::hkx_header::HkxHeader, from_bytes, from_str, to_bytes, to_string, HavokSort,
};
use snafu::ResultExt as _;
use std::{
    borrow::Cow,
    ffi::OsStr,
    io::{self, Read},
    path::Path,
};
use tokio::fs;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[display(style = "camelCase")]
#[non_exhaustive]
pub enum Format {
    /// XML
    Xml,
    /// 32bit
    Win32,
    /// 64bit
    Amd64,

    #[cfg(feature = "extra_fmt")]
    /// json
    Json,
    #[cfg(feature = "extra_fmt")]
    /// yaml
    Yaml,
    #[cfg(feature = "extra_fmt")]
    /// toml
    Toml,
}

impl Format {
    /// Return the file extension corresponding to the format.
    #[inline]
    const fn as_str(&self) -> &str {
        match *self {
            Self::Xml => "xml",
            Self::Win32 => "hkx",
            Self::Amd64 => "hkx",

            #[cfg(feature = "extra_fmt")]
            Self::Json => "json",
            #[cfg(feature = "extra_fmt")]
            Self::Yaml => "yaml",
            #[cfg(feature = "extra_fmt")]
            Self::Toml => "toml",
        }
    }

    /// Return output format from input path.
    ///
    /// # Note
    /// Unknown extension => `Amd64`
    pub fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let ext = path.extension().ok_or(Error::UnsupportedExtensionPath {
            path: path.to_path_buf(),
        })?;
        Self::from_extension(ext).map_err(|_| Error::UnsupportedExtensionPath {
            path: path.to_path_buf(),
        })
    }

    pub fn from_extension(ext: &OsStr) -> Result<Self> {
        Ok(match ext {
            ext if ext.eq_ignore_ascii_case("hkx") => Self::Xml,
            ext if ext.eq_ignore_ascii_case("xml") => Self::Amd64,

            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("json") => Self::Json,
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => {
                Self::Yaml
            }
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("toml") => Self::Toml,
            _ => {
                return Err(Error::UnsupportedExtension {
                    ext: ext.to_string_lossy().to_string(),
                })
            }
        })
    }
}

impl FromStr for Format {
    type Err = Error;

    #[inline]
    fn from_str(ext: &str) -> core::result::Result<Self, Self::Err> {
        Ok(match ext {
            ext if ext.eq_ignore_ascii_case("hkx") => Self::Xml,
            ext if ext.eq_ignore_ascii_case("xml") => Self::Amd64,

            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("json") => Self::Json,
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => {
                Self::Yaml
            }
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("toml") => Self::Toml,
            _ => {
                return Err(Error::UnsupportedExtension {
                    ext: ext.to_string(),
                })
            }
        })
    }
}

/// Convert dir or file(hkx, xml).
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub async fn convert<I, O>(input: I, output: Option<O>, format: Format) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();
    if input.is_dir() {
        convert_dir(input, output, format).await?;
    } else if input.is_file() {
        convert_file(input, output, format).await?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("The path does not exist: {}", input.display()),
        ))?;
    }

    Ok(())
}

/// Convert dir.
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub async fn convert_dir<I, O>(input_dir: I, output_dir: Option<O>, format: Format) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input_dir = input_dir.as_ref();

    let mut task_handles: Vec<tokio::task::JoinHandle<_>> = Vec::new();
    for path in jwalk::WalkDir::new(input_dir) {
        let path = path?.path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy();
            match ext.as_ref() {
                "hkx" | "xml" => {}
                _ => continue,
            }
        }
        let input = path.to_path_buf();

        // If output_dir is specified, make it the root dir to maintain the hierarchy and output.
        let output = match output_dir.as_ref() {
            Some(output_dir) => {
                let relative_path = input.strip_prefix(input_dir)?;
                let mut output = output_dir.as_ref().join(relative_path);
                output.set_extension(format.as_str());
                Some(output)
            }
            None => None,
        };

        task_handles.push(tokio::spawn(async move {
            convert_file(&input, output, format).await
        }));
    }

    for task_handle in task_handles {
        task_handle.await??;
    }
    Ok(())
}

/// Convert `hkx`/`xml` file to `hkx`/`xml` file.
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub async fn convert_file<I, O>(input: I, output: Option<O>, format: Format) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();
    let bytes = input.read_bytes().await?;
    let extension = input.extension();
    let mut xml = String::new(); // To avoid ownership errors, declare it here, but since it is a 0-allocation, there is no problem.

    // Deserialize
    let mut classes: ClassMap = if let Some(ext) = extension {
        let fmt = Format::from_extension(ext).map_err(|_| Error::UnsupportedExtensionPath {
            path: input.to_path_buf(),
        })?;

        match fmt {
            Format::Amd64 | Format::Win32 => from_bytes(&bytes).context(DeSnafu {
                input: input.to_path_buf(),
            })?,

            _ => {
                let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
                decoder
                    .read_to_string(&mut xml)
                    .context(FailedReadFileSnafu {
                        path: input.to_path_buf(),
                    })?;

                match fmt {
                    Format::Xml => from_str(&xml).context(DeSnafu {
                        input: input.to_path_buf(),
                    })?,

                    #[cfg(feature = "extra_fmt")]
                    Format::Json => {
                        simd_json::from_slice(unsafe { xml.as_bytes_mut() }).context(JsonSnafu {
                            input: input.to_path_buf(),
                        })?
                    }
                    #[cfg(feature = "extra_fmt")]
                    Format::Yaml => serde_yml::from_str(&xml).context(YamlSnafu {
                        input: input.to_path_buf(),
                    })?,
                    #[cfg(feature = "extra_fmt")]
                    Format::Toml => toml::from_str(&xml).map_err(|err| Error::TomlDeError {
                        input: input.to_path_buf(),
                        source: Box::new(err),
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

    // Serialize
    match format {
        Format::Xml => {
            let top_ptr = classes.sort_for_xml().context(SerSnafu {
                input: input.to_path_buf(),
            })?;
            let xml = to_string(&classes, top_ptr).context(SerSnafu {
                input: input.to_path_buf(),
            })?;
            fs_write(input, output, "xml", xml).await?;
            Ok(())
        }
        _ => {
            classes.sort_for_bytes();
            let binary_data = match format {
                Format::Win32 => to_bytes(&classes, &HkxHeader::new_skyrim_le()),
                Format::Amd64 => to_bytes(&classes, &HkxHeader::new_skyrim_se()),

                #[cfg(feature = "extra_fmt")]
                Format::Json | Format::Yaml | Format::Toml => {
                    let contents = match format {
                        Format::Json => simd_json::to_string(&classes).context(JsonSnafu {
                            input: input.to_path_buf(),
                        })?,
                        Format::Toml => toml::to_string_pretty(&classes).context(TomlSerSnafu {
                            input: input.to_path_buf(),
                        })?,
                        Format::Yaml => serde_yml::to_string(&classes).context(YamlSnafu {
                            input: input.to_path_buf(),
                        })?,
                        _ => unreachable!(),
                    };
                    fs_write(input, output, format.as_str(), contents).await?;
                    return Ok(());
                }
                _ => unreachable!(),
            }
            .context(SerSnafu {
                input: input.to_path_buf(),
            })?;

            fs_write(input, output, "hkx", binary_data).await?;
            Ok(())
        }
    }
}

/// Write specified or same location.
async fn fs_write<I, O>(
    input: I,
    output: Option<O>,
    ext: &str,
    contents: impl AsRef<[u8]>,
) -> io::Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();

    let output = output
        .as_ref()
        .map(|output| Cow::Borrowed(output.as_ref()))
        .unwrap_or({
            let mut output = input.to_path_buf();
            output.set_extension(ext);
            Cow::Owned(output)
        });

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(&output, contents).await?;

    #[cfg(feature = "tracing")]
    tracing::info!("Converted {} -> {}", input.display(), output.display());
    Ok(())
}
