//! Convert hkx <-> xml
use super::ClassMap;
use crate::{
    error::{DeSnafu, Error, FailedReadFileSnafu, Result, SerSnafu},
    read_ext::ReadExt as _,
};
use parse_display::{Display, FromStr};
use serde_hkx::{
    bytes::serde::hkx_header::HkxHeader, from_bytes, from_str, to_bytes, to_string, HavokSort,
};
use snafu::ResultExt as _;
use std::{
    borrow::Cow,
    io::{self, Read},
    path::Path,
};
use tokio::fs;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, FromStr)]
pub enum Format {
    /// XML
    #[display("xml")]
    Xml,
    /// 32bit
    #[display("win32")]
    Win32,
    /// 64bit
    #[display("amd64")]
    Amd64,
}

impl Format {
    /// Return the file extension corresponding to the format.
    #[inline]
    const fn as_extension(&self) -> &str {
        match *self {
            Format::Xml => "xml",
            Format::Win32 => "hkx",
            Format::Amd64 => "hkx",
        }
    }

    /// Return output format from input path.
    ///
    /// # Note
    /// Unknown extension => `Amd64`
    pub fn from_input<P>(input: P) -> Self
    where
        P: AsRef<Path>,
    {
        if let Some(extension) = input.as_ref().extension() {
            let extension = extension.to_ascii_lowercase();
            match extension.to_string_lossy().as_ref() {
                "hkx" => Format::Xml,
                "xml" => Format::Amd64,
                _ => Format::Amd64,
            }
        } else {
            Format::Amd64
        }
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
                output.set_extension(format.as_extension());
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

    let mut classes: ClassMap = if let Some(ext) = extension {
        if ext.eq_ignore_ascii_case("hkx") {
            from_bytes(&bytes).context(DeSnafu {
                input: input.to_path_buf(),
            })?
        } else if ext.eq_ignore_ascii_case("xml") {
            let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
            decoder
                .read_to_string(&mut xml)
                .context(FailedReadFileSnafu {
                    path: input.to_path_buf(),
                })?;
            from_str(&xml).context(DeSnafu {
                input: input.to_path_buf(),
            })?
        } else {
            return Err(Error::UnsupportedExtension {
                path: input.to_path_buf(),
            });
        }
    } else {
        return Err(Error::MissingExtension {
            path: input.to_path_buf(),
        });
    };

    match format {
        Format::Xml => {
            let top_ptr = classes.sort_for_xml().context(SerSnafu {
                input: input.to_path_buf(),
            })?;
            let xml = to_string(&classes, top_ptr).context(SerSnafu {
                input: input.to_path_buf(),
            })?;

            let output = output
                .as_ref()
                .map(|output| Cow::Borrowed(output.as_ref()))
                .unwrap_or({
                    let mut output = input.to_path_buf();
                    output.set_extension("xml");
                    Cow::Owned(output)
                });

            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(output, xml).await?;
        }

        Format::Win32 | Format::Amd64 => {
            classes.sort_for_bytes();
            let binary_data = match format {
                Format::Win32 => to_bytes(&classes, &HkxHeader::new_skyrim_le()),
                Format::Amd64 => to_bytes(&classes, &HkxHeader::new_skyrim_se()),
                Format::Xml => unreachable!(),
            }
            .context(SerSnafu {
                input: input.to_path_buf(),
            })?;

            let output = output
                .as_ref()
                .map(|output| Cow::Borrowed(output.as_ref()))
                .unwrap_or({
                    let mut output = input.to_path_buf();
                    output.set_extension("hkx");
                    Cow::Owned(output)
                });

            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(&output, binary_data).await?;
            #[cfg(feature = "tracing")]
            tracing::info!("Converted {} -> {}", input.display(), output.display());
        }
    }

    Ok(())
}
