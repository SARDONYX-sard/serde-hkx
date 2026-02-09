//! Async Convert hkx <-> xml
use super::{Format, get_output_path, get_supported_files, process_serde};
use crate::{
    error::Result,
    fs::{ReadExt as _, write},
    progress::{DefaultProgressMonitor, ProgressHandler},
};
use std::{
    io::{self},
    path::{Path, PathBuf},
};

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
    convert_progress(input, output, format, DefaultProgressMonitor).await
}

/// Convert dir or file(hkx, xml).
///
/// # Note
/// If `output` is not specified, the output is placed at the same level as `input`.
///
/// # Errors
/// Failed to convert.
pub async fn convert_progress<I, O, P>(
    input: I,
    output: Option<O>,
    format: Format,
    progress: P,
) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
    P: ProgressHandler + Send + Clone + 'static,
{
    let input = input.as_ref();
    if input.is_dir() {
        convert_dir(input, output, format, progress).await?;
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
pub async fn convert_dir<I, O, P>(
    input_dir: I,
    output_dir: Option<O>,
    format: Format,
    mut progress: P,
) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
    P: ProgressHandler + Send + Clone + 'static,
{
    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();

    let input_dir = input_dir.as_ref();
    let files: Vec<PathBuf> = get_supported_files(input_dir);

    if files.is_empty() {
        progress.on_empty();
        return Ok(());
    }

    progress.on_set_total(files.len());

    for input in files {
        // Convert only if there is a supported extension.
        progress.on_processing_path(&input);

        // If `out_dir` is specified, join the internal dirs of `input_dir` with it as root.
        let output = get_output_path(input_dir, &input, &output_dir, format);

        let progress = progress.clone();
        task_handles.push(tokio::spawn(async move {
            match convert_file(&input, output, format).await {
                Ok(_) => {
                    progress.success_inc(1);
                    Ok(())
                }
                Err(err) => {
                    progress.failure_inc(1);
                    #[cfg(feature = "tracing")]
                    tracing::error!("Failed to convert: {}", input.display());
                    Err(err)
                }
            }?;
            progress.inc(1);
            Result::Ok(())
        }));
    }

    for task_handle in task_handles {
        task_handle.await??;
    }

    progress.on_finish();
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
    let in_bytes = input.read_bytes().await?;
    let out_bytes = process_serde(&in_bytes, input, format)?;

    Ok(write(input, output, format.as_extension(), out_bytes).await?)
}
