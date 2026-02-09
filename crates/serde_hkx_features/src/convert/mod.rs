pub mod hkx_checker;
pub mod rayon;
pub mod tokio;

use crate::error::{Error, Result};
use parse_display::{Display, FromStr};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// An enum used to specify input/output formats
///
/// # Default
/// `Amd64`
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, FromStr)]
#[display(style = "camelCase")]
#[non_exhaustive]
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

    /// Return path format of this path.
    ///
    /// # Examples
    /// ```edition2021
    /// use serde_hkx_features::convert::Format;
    ///
    /// assert_eq!(Format::from_current_format("example.hkx").unwrap(), Format::Amd64);
    /// assert_eq!(Format::from_current_format("example.xml").unwrap(), Format::Xml);
    /// ```
    ///
    /// When enable `extra_fmt` feature.
    /// - `json`, `yaml` -> `Self::Amd64`
    ///
    /// # Errors
    /// Unknown extension.
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

fn process_serde<I>(in_bytes: &Vec<u8>, input: I, format: Format) -> Result<Vec<u8>>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();
    let mut string = String::new();

    // Deserialize
    let mut classes = {
        #[cfg(not(feature = "extra_fmt"))]
        {
            crate::serde::de::deserialize(in_bytes, &mut string, input)?
        }
        #[cfg(feature = "extra_fmt")]
        {
            crate::serde_extra::de::deserialize(in_bytes, &mut string, input)?
        }
    };

    // Serialize
    let out_bytes = match format {
        Format::Amd64 | Format::Win32 | Format::Xml => {
            crate::serde::ser::to_bytes(input, format, &mut classes)?
        }
        #[cfg(feature = "extra_fmt")]
        Format::Json | Format::Toml | Format::Yaml => {
            let mut classes = crate::types_wrapper::ClassPtrMap::from_class_map(classes);
            crate::serde_extra::ser::to_bytes(input, format, &mut classes)?
        }
    };

    Ok(out_bytes)
}
