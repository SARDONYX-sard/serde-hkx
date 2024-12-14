//! Parallel Convert hkx <-> xml

use super::{get_output_path, get_supported_files, process_serde, OutFormat};
use crate::{
    error::{FailedConvertFilesSnafu, Result},
    fs::write_sync,
    verify::ProgressHandler,
};
use rayon::prelude::*;
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
    let files: Vec<PathBuf> = get_supported_files(input_dir);

    if files.is_empty() {
        progress.on_empty();
        return Ok(());
    }

    let total_files = files.len();
    progress.on_set_total(total_files);

    let err_paths: Vec<PathBuf> = files
        .into_par_iter()
        .filter_map(|input| {
            progress.on_processing_path(&input);

            let output = get_output_path(input_dir, &input, &output_dir, format);
            let res = convert_file(&input, output, format);
            progress.inc(1);

            if matches!(res, Ok(())) {
                progress.success_inc(1);
                None
            } else {
                progress.failure_inc(1);
                Some(input)
            }
        })
        .collect();

    progress.on_finish();

    if err_paths.is_empty() {
        Ok(())
    } else {
        FailedConvertFilesSnafu {
            path: input_dir.to_path_buf(),
            total_files,
            err_paths,
        }
        .fail()
    }
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
    let out_bytes = process_serde(&in_bytes, input, format)?;

    Ok(write_sync(input, output, format.as_extension(), out_bytes)?)
}
