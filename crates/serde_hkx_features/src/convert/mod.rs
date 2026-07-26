pub mod hkx_checker;
pub mod rayon;
pub mod tokio;

use snafu::ResultExt as _;

use crate::error::{DeSnafu, Error, Result};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// An enum used to specify input/output formats
///
/// # Default
/// `Amd64`
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Format {
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

impl core::fmt::Display for Format {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Amd64 => write!(f, "amd64"),
            Self::Win32 => write!(f, "win32"),
            Self::Xml => write!(f, "xml"),
            #[cfg(feature = "extra_fmt")]
            Self::Json => write!(f, "json"),
            #[cfg(feature = "extra_fmt")]
            Self::Toml => write!(f, "toml"),
            #[cfg(feature = "extra_fmt")]
            Self::Yaml => write!(f, "yaml"),
        }
    }
}

/// invalid format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseFormatError;

impl core::fmt::Display for ParseFormatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid format")
    }
}

impl core::error::Error for ParseFormatError {}

impl core::str::FromStr for Format {
    type Err = ParseFormatError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            _ if value.eq_ignore_ascii_case("amd64") => Ok(Self::Amd64),
            _ if value.eq_ignore_ascii_case("win32") => Ok(Self::Win32),
            _ if value.eq_ignore_ascii_case("xml") => Ok(Self::Xml),
            #[cfg(feature = "extra_fmt")]
            _ if value.eq_ignore_ascii_case("json") => Ok(Self::Json),
            #[cfg(feature = "extra_fmt")]
            _ if value.eq_ignore_ascii_case("toml") => Ok(Self::Toml),
            #[cfg(feature = "extra_fmt")]
            _ if value.eq_ignore_ascii_case("yaml") => Ok(Self::Yaml),
            _ => Err(ParseFormatError),
        }
    }
}

impl Format {
    /// Return the file extension corresponding to the format.
    ///
    /// # Examples
    /// ```edition2021
    /// use serde_hkx_features::convert::Format;
    ///
    /// assert_eq!(Format::Amd64.as_extension(), "hkx");
    /// assert_eq!(Format::Win32.as_extension(), "hkx");
    /// assert_eq!(Format::Xml.as_extension(), "xml");
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

    /// Return current path format of this path.
    ///
    /// # Examples
    /// ```edition2021 ,no_run
    /// use serde_hkx_features::convert::Format;
    ///
    /// assert_eq!(Format::from_current_format("amd64.hkx").unwrap(), Format::Amd64);
    /// assert_eq!(Format::from_current_format("win32.hkx").unwrap(), Format::Win32);
    /// assert_eq!(Format::from_current_format("example.xml").unwrap(), Format::Xml);
    /// ```
    ///
    /// When enable `extra_fmt` feature.
    /// - `json` -> `Self::Json`
    /// - `yaml` -> `Self::Yaml`
    ///
    /// Internally, when the file extension is .hkx, the first 17 bytes of the file are examined to determine the architecture.
    ///
    /// # Errors
    /// In the case of unsupported file extensions or invalid hkx files.
    #[inline]
    pub fn from_current_format<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let ext = path.extension().ok_or(Error::UnsupportedExtensionPath {
            path: path.to_path_buf(),
        })?;

        Ok(match ext {
            ext if ext.eq_ignore_ascii_case("hkx") => hkx_checker::detect_hkx_format(path)?,
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
                return Err(Error::UnsupportedExtensionPath {
                    path: path.to_path_buf(),
                });
            }
        })
    }

    /// Return output format from input path.
    ///
    /// # Examples
    /// ```edition2021
    /// use serde_hkx_features::convert::Format;
    ///
    /// assert_eq!(Format::infer_output_from_input("example.hkx").unwrap(), Format::Xml);
    /// assert_eq!(Format::infer_output_from_input("example.xml").unwrap(), Format::Amd64);
    /// ```
    ///
    /// When enable `extra_fmt` feature.
    /// - `json`, `yaml` -> `Self::Amd64`
    ///
    /// # Errors
    /// Unknown extension.
    #[inline]
    pub fn infer_output_from_input<P>(path: P) -> Result<Self>
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
                });
            }
        })
    }

    /// Determine format from extension.
    ///
    /// # Examples
    /// ```edition2021
    /// use serde_hkx_features::convert::Format;
    ///
    /// assert_eq!(Format::from_extension("hkx").unwrap(), Format::Amd64);
    /// assert_eq!(Format::from_extension("xml").unwrap(), Format::Xml);
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
                });
            }
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// tokio & rayon common code

fn get_output_path<D, I, O>(
    input_dir: D,
    input: I,
    output_dir: &Option<O>,
    format: Format,
) -> Option<PathBuf>
where
    D: AsRef<Path>,
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();
    let input_dir = input_dir.as_ref();

    match output_dir {
        Some(output_dir) => {
            let input_inner_dir = input.strip_prefix(input_dir).ok()?;
            let mut output = output_dir.as_ref().join(input_inner_dir);
            output.set_extension(format.as_extension());
            Some(output)
        }
        None => None,
    }
}

fn filter_supported_files(entry: &jwalk::DirEntry<((), ())>) -> bool {
    let path = entry.path();

    if !path.is_file() {
        return false;
    }

    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            if Format::from_extension(ext).is_err() {
                #[cfg(feature = "tracing")]
                tracing::info!("Skip this unsupported extension: {}", path.display());
                false
            } else {
                true
            }
        })
}

fn get_supported_files(input_dir: &Path) -> Vec<PathBuf> {
    jwalk::WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(filter_supported_files)
        .map(|entry| entry.path())
        .collect()
}

/// Deserializes serialized HKX data into a [`ClassMap`] and applies a
/// format-specific transformation.
///
/// This is useful when the deserialized class data needs to be modified
/// before constructing higher-level types such as `Hkanno`.
///
/// # Errors
///
/// Returns an error if:
///
/// * the input path has no extension.
/// * the extension is unsupported.
/// * deserialization fails.
/// * the handler returns an error.
pub fn process_serde_with<'a, I, F, G, T>(
    bytes: &'a [u8],
    input: I,
    on_xml: G,   // ClassMap borrows from local String, must return T directly
    on_other: F, // ClassMap borrows from bytes ('a), T can carry 'a
) -> Result<T, Error>
where
    I: AsRef<Path>,
    F: FnOnce(crate::ClassMap<'a>) -> Result<T, Error>,
    G: FnOnce(crate::ClassMap<'_>) -> Result<T, Error>, // '_ = local String lifetime
{
    let input = input.as_ref();
    let input_fmt = {
        let Some(input_ext) = input.extension() else {
            return Err(Error::MissingExtension {
                path: input.to_path_buf(),
            });
        };
        Format::from_extension(input_ext).map_err(|_| Error::UnsupportedExtensionPath {
            path: input.to_path_buf(),
        })?
    };

    let classes = match input_fmt {
        Format::Amd64 | Format::Win32 => serde_hkx::from_bytes(bytes)
            .context(crate::serde::de::HkxSnafu {})
            .with_context(|_| DeSnafu {
                input: input.to_path_buf(),
            })?,
        Format::Xml => {
            let string = auto_charset::decode_str_to_utf8(bytes)?;
            let classes = serde_hkx::from_str(&string)
                .context(crate::serde::de::XmlSnafu {})
                .with_context(|_| DeSnafu {
                    input: input.to_path_buf(),
                })?;

            return on_xml(classes);
        }
        #[cfg(feature = "extra_fmt")]
        Format::Json => {
            use crate::types_wrapper::ClassPtrMap;
            let classes = sonic_rs::from_slice::<ClassPtrMap>(bytes)
                .context(crate::serde::de::JsonSnafu {})
                .with_context(|_| crate::error::DeSnafu {
                    input: input.to_path_buf(),
                })?;
            classes.into_class_map()
        }
        #[cfg(feature = "extra_fmt")]
        Format::Toml => {
            use crate::types_wrapper::ClassPtrMap;
            let classes = basic_toml::from_slice::<ClassPtrMap>(bytes)
                .context(crate::serde::de::TomlSnafu {})
                .with_context(|_| crate::error::DeSnafu {
                    input: input.to_path_buf(),
                })?;
            classes.into_class_map()
        }
        #[cfg(feature = "extra_fmt")]
        Format::Yaml => {
            use crate::types_wrapper::ClassPtrMap;
            let classes = serde_norway::from_slice::<ClassPtrMap>(bytes)
                .context(crate::serde::de::YamlSnafu {})
                .with_context(|_| crate::error::DeSnafu {
                    input: input.to_path_buf(),
                })?;
            classes.into_class_map()
        }
    };

    on_other(classes)
}

/// bytes(input) -> output_format
pub(crate) fn process_serde<I>(
    bytes: Vec<u8>,
    input: I,
    output_format: Format,
) -> Result<Vec<u8>, Error>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();

    process_serde_with(
        &bytes,
        input,
        |mut classes| {
            match output_format {
                Format::Amd64 | Format::Win32 | Format::Xml => {
                    crate::serde::ser::to_bytes(&mut classes, output_format)
                }
                #[cfg(feature = "extra_fmt")]
                Format::Json | Format::Toml | Format::Yaml => {
                    let mut classes = crate::types_wrapper::ClassPtrMap::from_class_map(classes);
                    crate::serde_extra::ser::to_bytes(&mut classes, output_format)
                }
            }
            .with_context(|_| crate::error::SerSnafu {
                input: input.to_path_buf(),
            })
        },
        |mut classes| {
            match output_format {
                Format::Amd64 | Format::Win32 | Format::Xml => {
                    crate::serde::ser::to_bytes(&mut classes, output_format)
                }
                #[cfg(feature = "extra_fmt")]
                Format::Json | Format::Toml | Format::Yaml => {
                    let mut classes = crate::types_wrapper::ClassPtrMap::from_class_map(classes);
                    crate::serde_extra::ser::to_bytes(&mut classes, output_format)
                }
            }
            .with_context(|_| crate::error::SerSnafu {
                input: input.to_path_buf(),
            })
        },
    )
}
