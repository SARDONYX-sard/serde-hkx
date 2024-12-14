//! Parallel Convert hkx <-> xml

use super::OutFormat;
use crate::{
    error::{Result, SerSnafu},
    fs::write_sync,
    verify::ProgressHandler,
};
use rayon::prelude::*;
use serde_hkx::HavokSort as _;
use snafu::ResultExt as _;
use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
};

/// Convert dir or file (hkx, xml).
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub fn convert_progress<I, O, P>(
    input: I,
    output: Option<O>,
    format: OutFormat,
    progress: P,
) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path> + Sync,
    P: ProgressHandler + Sync,
{
    let input = input.as_ref();
    if input.is_dir() {
        convert_dir(input, output, format, progress)?;
    } else if input.is_file() {
        convert_file(input, output, format)?;
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("The path does not exist: {}", input.display()),
        ))?;
    }

    Ok(())
}

/// Convert directory.
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub fn convert_dir<I, O, P>(
    input_dir: I,
    output_dir: Option<O>,
    format: OutFormat,
    mut progress: P,
) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path> + Sync,
    P: ProgressHandler + Sync,
{
    let input_dir = input_dir.as_ref();

    let files: Vec<PathBuf> = jwalk::WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    if files.is_empty() {
        progress.on_empty();
        return Ok(());
    }

    progress.on_set_total(files.len());

    files.into_par_iter().try_for_each(|input| -> Result<()> {
        if let Some(ext) = input.extension() {
            progress.on_processing_path(&input);

            if OutFormat::from_extension(ext).is_ok() {
                let output = match output_dir.as_ref() {
                    Some(output_dir) => {
                        let input_inner_dir = input.strip_prefix(input_dir)?;
                        let mut output = output_dir.as_ref().join(input_inner_dir);
                        output.set_extension(format.as_extension());
                        Some(output)
                    }
                    None => None,
                };

                convert_file(input, output, format)?;
                progress.inc(1);
            } else {
                #[cfg(feature = "tracing")]
                tracing::info!("Skip unsupported extension: {}", input.display());
            }
        }
        Ok(())
    })?;

    progress.on_finish();
    Ok(())
}

/// Convert `hkx`/`xml` file and vice versa.
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub fn convert_file<I, O>(input: I, output: Option<O>, format: OutFormat) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();
    let in_bytes = fs::read(input)?;
    let mut string = String::new();

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
    let out_bytes = match format {
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
            let mut classes = crate::types_wrapper::ClassPtrMap::from_class_map(classes);
            crate::serde_extra::ser::to_bytes(input, format, &mut classes)?
        }
    };

    write_sync(input, output, format.as_extension(), out_bytes)?;
    Ok(())
}
