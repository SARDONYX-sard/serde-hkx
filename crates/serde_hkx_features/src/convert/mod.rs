pub mod rayon;
pub mod tokio;

use crate::error::{Error, Result};
use parse_display::{Display, FromStr};
use std::{ffi::OsStr, path::Path};

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
