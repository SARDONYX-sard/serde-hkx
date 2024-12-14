//! Async Convert hkx <-> xml
use super::OutFormat;
use crate::{
    error::{Result, SerSnafu},
    fs::{write, ReadExt as _},
    verify::{DefaultProgressMonitor, ProgressHandler},
};
use serde_hkx::HavokSort as _;
use snafu::ResultExt as _;
use std::{
    io::{self},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

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
    format: OutFormat,
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
    format: OutFormat,
    mut progress: P,
) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
    P: ProgressHandler + Send + Clone + 'static,
{
    let input_dir = input_dir.as_ref();

    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();

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
    let success_count = Arc::new(AtomicUsize::new(0));
    let failure_count = Arc::new(AtomicUsize::new(0));
    progress.start_progress_monitoring(Arc::clone(&success_count), Arc::clone(&failure_count));

    for input in files {
        // Convert only if there is a supported extension.
        if let Some(ext) = input.extension() {
            progress.on_processing_path(&input);

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

            let progress = progress.clone();
            let success_count = Arc::clone(&success_count);
            let failure_count = Arc::clone(&failure_count);
            task_handles.push(tokio::spawn(async move {
                match convert_file(&input, output, format).await {
                    Ok(_) => {
                        success_count.fetch_add(1, Ordering::AcqRel);
                        Ok(())
                    }
                    Err(err) => {
                        failure_count.fetch_add(1, Ordering::AcqRel);
                        Err(err)
                    }
                }?;
                progress.inc(1);
                Result::Ok(())
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
