use crate::error::Result;
use havok_classes::Classes;
use serde_hkx::{bytes::serde::hkx_header::HkxHeader, from_bytes, from_str, to_bytes, to_string};
use std::{io::Read, path::Path};
use tokio::fs;

#[derive(Debug, clap::Args)]
pub(crate) struct Convert {
    #[clap(short, long)]
    #[clap(value_parser)]
    /// Path containing the hkx/xml file/directory.
    pub input: String,
    #[clap(short, long)]
    /// Directory output destination.
    pub output: Option<String>,

    /// File format to output.
    ///
    /// xml | win32 | amd64
    #[clap(short = 'v', long)]
    pub format: Format,

    // ---logger
    #[clap(long)]
    /// Log output to standard output as well
    pub stdout: bool,
    #[clap(long, default_value = "error")]
    /// Log level
    ///
    /// trace | debug | info | warn | error
    pub log_level: String,
    #[clap(long)]
    /// Output path of log file
    pub log_file: Option<String>,
}

type ClassMap<'a> = indexmap::IndexMap<usize, Classes<'a>>;

#[derive(Debug, clap::ValueEnum, Clone, Copy, parse_display::Display)]
pub enum Format {
    #[display("xml")]
    Xml,
    #[display("win32")]
    Win32,
    #[display("amd64")]
    Amd64,
}

impl core::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(if s.eq_ignore_ascii_case("xml") {
            Self::Xml
        } else if s.eq_ignore_ascii_case("win32") {
            Self::Win32
        } else if s.eq_ignore_ascii_case("amd64") {
            Self::Amd64
        } else {
            return Err("Invalid format {s}".to_string());
        })
    }
}

pub async fn convert(
    input: impl AsRef<Path>,
    output: Option<impl AsRef<Path>>,
    format: Format,
) -> Result<()> {
    if input.as_ref().is_dir() {
        convert_dir(input, output, format).await?;
    } else if input.as_ref().is_file() {
        convert_file(input, output, format).await?;
    }

    Ok(())
}

pub async fn convert_dir(
    input_dir: impl AsRef<Path>,
    output_dir: Option<impl AsRef<Path>>,
    format: Format,
) -> Result<()> {
    let target_ext = format.to_string();

    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();
    for path in jwalk::WalkDir::new(input_dir) {
        let path = path?.path();
        let path = path.as_path();
        if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new(&target_ext)) {
            continue;
        }

        let input = path.to_path_buf();
        let output = match output_dir {
            Some(ref output) => output.as_ref().join(input.strip_prefix(&input)?),
            None => {
                let mut output = input.clone();
                output.set_extension(match format {
                    Format::Xml => "xml",
                    _ => "hkx",
                });
                output
            }
        };

        task_handles.push(tokio::spawn(async move {
            let cloned_path = input.clone();
            match convert_file(input, Some(output), format).await {
                Ok(_) => Ok(()),
                Err(err) => {
                    tracing::error!("{cloned_path:?}\n{err}");
                    Err(err)
                }
            }
        }));
    }

    for task_handle in task_handles {
        if let Err(err) = task_handle.await {
            panic!("{err}")
        };
    }
    Ok(())
}

pub async fn convert_file<I, O>(input: I, output: Option<O>, format: Format) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();
    let output = output;
    let bytes = fs::read(input).await?;

    match format {
        Format::Xml => {
            let mut classes: ClassMap = from_bytes(&bytes)?;

            let mut top_ptr = None;
            if !classes.is_empty() {
                if let Some((first_key, first_value)) = classes.shift_remove_index(0) {
                    classes.insert(first_key, first_value);
                    top_ptr = Some(first_key);
                }
            };

            let xml = to_string(&classes, top_ptr.unwrap_or_default())?;

            let output = output
                .map(|output| output.as_ref().to_path_buf())
                .unwrap_or({
                    let mut output = input.to_path_buf();
                    output.set_extension("xml");
                    output
                });

            if let Some(parent) = output.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            fs::write(output, xml).await?;
        }
        Format::Win32 | Format::Amd64 => {
            let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
            let mut xml = String::new();
            decoder.read_to_string(&mut xml)?;
            let mut classes: ClassMap = from_str(&xml)?;
            classes.sort_keys();

            let binary_data = match format {
                Format::Win32 => to_bytes(&classes, &HkxHeader::new_skyrim_le()),
                Format::Amd64 => to_bytes(&classes, &HkxHeader::new_skyrim_se()),
                Format::Xml => unreachable!(),
            }?;

            let output = output
                .map(|output| output.as_ref().to_path_buf())
                .unwrap_or({
                    let mut output = input.to_path_buf();
                    output.set_extension("hkx");
                    output
                });
            fs::write(output, binary_data).await?;
        }
    }

    Ok(())
}
