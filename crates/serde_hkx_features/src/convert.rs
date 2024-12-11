//! Convert hkx <-> xml
use crate::{
    error::{Error, Result, SerSnafu},
    fs::{write, ReadExt as _},
};
use parse_display::{Display, FromStr};
use serde_hkx::HavokSort as _;
use snafu::ResultExt as _;
use std::{
    ffi::OsStr,
    io::{self},
    path::Path,
};

/// Output format
///
/// # Default
/// `Amd64`
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr)]
#[display(style = "camelCase")]
#[non_exhaustive]
pub enum OutFormat {
    /// 64bit hkx
    #[default]
    Amd64,
    /// 32bit hkx
    Win32,
    /// XML
    Xml,

    #[cfg(feature = "extra_fmt")]
    /// json
    Json,
    #[cfg(feature = "extra_fmt")]
    /// yaml
    Toml,
    #[cfg(feature = "extra_fmt")]
    /// yaml
    Yaml,
}

impl OutFormat {
    /// Return the file extension corresponding to the format.
    ///
    /// # Examples
    /// ```edition2021
    /// use serde_hkx_features::convert::OutFormat;
    ///
    /// assert_eq!(OutFormat::Amd64.as_extension(), "hkx");
    /// assert_eq!(OutFormat::Win32.as_extension(), "hkx");
    /// assert_eq!(OutFormat::Xml.as_extension(), "xml");
    /// ```
    #[inline]
    pub const fn as_extension(&self) -> &str {
        match *self {
            Self::Amd64 => "hkx",
            Self::Win32 => "hkx",
            Self::Xml => "xml",

            #[cfg(feature = "extra_fmt")]
            Self::Json => "json",
            #[cfg(feature = "extra_fmt")]
            Self::Toml => "toml",
            #[cfg(feature = "extra_fmt")]
            Self::Yaml => "yaml",
        }
    }

    /// Return output format from input path.
    ///
    /// # Examples
    /// ```edition2021
    /// use serde_hkx_features::convert::OutFormat;
    ///
    /// assert_eq!(OutFormat::from_input("example.hkx").unwrap(), OutFormat::Xml);
    /// assert_eq!(OutFormat::from_input("example.xml").unwrap(), OutFormat::Amd64);
    /// ```
    ///
    /// When enable `extra_fmt` feature.
    /// - `json`, `yaml` -> `Self::Amd64`
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
            ext if ext.eq_ignore_ascii_case("json") => Self::Amd64,
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("toml") => Self::Amd64,
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => {
                Self::Amd64
            }
            _ => {
                return Err(Error::UnsupportedExtensionPath {
                    path: path.to_path_buf(),
                })
            }
        })
    }

    /// Determine format from extension.
    ///
    /// # Examples
    /// ```edition2021
    /// use serde_hkx_features::convert::OutFormat;
    ///
    /// assert_eq!(OutFormat::from_extension("hkx").unwrap(), OutFormat::Amd64);
    /// assert_eq!(OutFormat::from_extension("xml").unwrap(), OutFormat::Xml);
    /// ```
    ///
    /// When enable `extra_fmt` feature.
    /// - `json` -> `Self::Json`
    /// - `toml` -> `Self::Toml`
    /// - `yaml` -> `Self::Yaml`
    ///
    /// # Errors
    /// Unknown extension.
    #[inline]
    pub fn from_extension<S>(ext: S) -> Result<Self>
    where
        S: AsRef<OsStr>,
    {
        let ext = ext.as_ref();
        Ok(match ext {
            ext if ext.eq_ignore_ascii_case("hkx") => Self::Amd64,
            ext if ext.eq_ignore_ascii_case("xml") => Self::Xml,

            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("json") => Self::Json,
            #[cfg(feature = "extra_fmt")]
            ext if ext.eq_ignore_ascii_case("toml") => Self::Toml,
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

/// Convert dir or file(hkx, xml).
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub async fn convert<I, O>(input: I, output: Option<O>, format: OutFormat) -> Result<()>
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
pub async fn convert_dir<I, O>(input_dir: I, output_dir: Option<O>, format: OutFormat) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input_dir = input_dir.as_ref();

    let mut task_handles: Vec<tokio::task::JoinHandle<_>> = Vec::new();
    for path in jwalk::WalkDir::new(input_dir) {
        let input = path?.path();
        if !input.is_file() {
            continue;
        }

        // Convert only if there is a supported extension.
        if let Some(ext) = input.extension() {
            if OutFormat::from_extension(ext).is_err() {
                #[cfg(feature = "tracing")]
                tracing::info!("Skip this unsupported extension`: {}", input.display());
                continue;
            };

            // If `out_dir` is specified, join the internal dirs of `input_dir` with it as root.
            let output = match output_dir.as_ref() {
                Some(output_dir) => {
                    let input_inner_dir = input.strip_prefix(input_dir)?;
                    let mut output = output_dir.as_ref().join(input_inner_dir);
                    output.set_extension(format.as_extension());
                    Some(output)
                }
                None => None,
            };

            task_handles.push(tokio::spawn(async move {
                convert_file(&input, output, format).await
            }));
        };
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
pub async fn convert_file<I, O>(input: I, output: Option<O>, format: OutFormat) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();
    let in_bytes = input.read_bytes().await?;
    let mut string = String::new(); // To avoid ownership errors, declare it here, but since it is a 0-allocation, there is no problem.

    // Deserialize
    let mut classes = {
        #[cfg(not(feature = "extra_fmt"))]
        {
            crate::serde::de::deserialize(&in_bytes, &mut string, input)?
        }
        #[cfg(feature = "extra_fmt")]
        {
            crate::serde_extra::de::deserialize(&in_bytes, &mut string, input)?
        }
    };

    // Serialize
    let out_bytes = {
        match format {
            OutFormat::Amd64 | OutFormat::Win32 => {
                crate::serde::ser::to_bytes(input, format, &mut classes)?
            }
            OutFormat::Xml => {
                let top_ptr = classes.sort_for_xml().context(SerSnafu {
                    input: input.to_path_buf(),
                })?;
                let xml = serde_hkx::to_string(&classes, top_ptr).context(SerSnafu {
                    input: input.to_path_buf(),
                })?;
                xml.into_bytes()
            }
            #[cfg(feature = "extra_fmt")]
            OutFormat::Json | OutFormat::Toml | OutFormat::Yaml => {
                // NOTE: Use a number (e.g. `1`) as a key, which is not supported by TOML, so use a `Pointer`(e.g. `#0001`).
                let mut classes = crate::types_wrapper::ClassPtrMap::from_class_map(classes);
                crate::serde_extra::ser::to_bytes(input, format, &mut classes)?
            }
        }
    };

    Ok(write(input, output, format.as_extension(), out_bytes).await?)
}
