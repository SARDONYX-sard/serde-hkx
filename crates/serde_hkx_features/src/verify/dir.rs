use super::verify_inner;
use crate::error::{FailedReproduceFilesSnafu, Result};
use crate::progress::ProgressHandler;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

/// Parallel checks reproduction for hkx files.
///
/// # Errors
/// If an error occurs, returns an array of error paths.
pub fn verify_dir<D, P>(dir: D, progress: P) -> Result<()>
where
    D: AsRef<Path>,
    P: ProgressHandler + Send + Sync,
{
    let dir = dir.as_ref();
    verify_dir_(dir, progress)
}

fn filter_hkx_files(dir: &Path) -> Vec<PathBuf> {
    jwalk::WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("hkx"))
        })
        .map(|entry| entry.path())
        .collect()
}

fn verify_dir_<P>(dir: &Path, mut progress: P) -> Result<()>
where
    P: ProgressHandler + Sync + Send,
{
    let filtered_entries = filter_hkx_files(dir);
    let total_files = filtered_entries.len();

    if total_files == 0 {
        progress.on_empty();
        return Ok(());
    }

    progress.on_set_total(total_files);

    let results: Vec<_> = filtered_entries
        .into_par_iter()
        .map(|path| {
            let result = verify_inner(&path);

            let is_valid = if let Ok((expected_bytes, actual_bytes)) = &result {
                expected_bytes == actual_bytes
            } else {
                false
            };

            if is_valid {
                progress.success_inc(1);
            } else {
                progress.failure_inc(1);
            }

            progress.inc(1);
            progress.on_processing_path(path.as_path());

            (path, is_valid)
        })
        .collect();

    progress.on_finish();

    let err_paths: Vec<PathBuf> = results
        .into_par_iter()
        .filter_map(|(path, is_valid)| if !is_valid { Some(path) } else { None })
        .collect();

    if err_paths.is_empty() {
        Ok(())
    } else {
        FailedReproduceFilesSnafu {
            path: dir.to_path_buf(),
            total_files,
            err_paths,
        }
        .fail()
    }
}
