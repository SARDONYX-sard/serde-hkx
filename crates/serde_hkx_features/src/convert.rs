//! Convert hkx <-> xml
use crate::{
    error::{Error, Result},
    fs::{write, ReadExt as _},
    serde::{deserialize, serialize_to_bytes},
};
use core::str::FromStr;
use parse_display::Display;
use std::{
    ffi::OsStr,
    io::{self},
    path::Path,
};

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
        }
    }

    /// Return output format from input path.
    /// - "hkx" => `Self::Xml`
    /// - "xml" => `Self::Amd64`
    ///
    /// # Errors
    /// Unknown extension.
    #[inline]
    pub fn from_input<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let ext = path.extension().ok_or(Error::UnsupportedExtensionPath {
            path: path.to_path_buf(),
        })?;

        Ok(match ext {
            ext if ext.eq_ignore_ascii_case("hkx") => Self::Xml,
            ext if ext.eq_ignore_ascii_case("xml") => Self::Amd64,

            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => {
                Self::Amd64
            }
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("json") => Self::Amd64,
            _ => {
                return Err(Error::UnsupportedExtensionPath {
                    path: path.to_path_buf(),
                })
            }
        })
    }

    /// Determine format from extension.
    #[inline]
    pub fn from_extension(ext: &OsStr) -> Result<Self> {
        Ok(match ext {
            ext if ext.eq_ignore_ascii_case("hkx") => Self::Amd64,
            ext if ext.eq_ignore_ascii_case("xml") => Self::Xml,

            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("json") => Self::Json,
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => {
                Self::Yaml
            }
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
            ext if ext.eq_ignore_ascii_case("hkx") => Self::Amd64,
            ext if ext.eq_ignore_ascii_case("xml") => Self::Xml,

            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("json") => Self::Json,
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => {
                Self::Yaml
            }
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
            if Format::from_extension(ext).is_err() {
                continue;
            };
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

/// Convert `hkx`/`xml` file and vice vasa.
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
    let mut string = String::new(); // To avoid ownership errors, declare it here, but since it is a 0-allocation, there is no problem.
    let mut classes = deserialize(&bytes, &mut string, input)?;
    let bytes = serialize_to_bytes(input, format, &mut classes).await?;
    Ok(write(input, output, format.as_str(), bytes).await?)
}
