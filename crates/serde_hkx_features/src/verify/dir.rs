use crate::error::{ReproduceHkxFilesSnafu, Result};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

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

use super::verify_inner;

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

    let success_count = Arc::new(AtomicUsize::new(0));
    let failure_count = Arc::new(AtomicUsize::new(0));

    progress.start_progress_monitoring(Arc::clone(&success_count), Arc::clone(&failure_count));

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
                success_count.fetch_add(1, Ordering::AcqRel);
            } else {
                failure_count.fetch_add(1, Ordering::AcqRel);
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
        ReproduceHkxFilesSnafu {
            path: dir.to_path_buf(),
            all_len: total_files,
            err_paths,
        }
        .fail()
    }
}

/// A trait for handling progress updates during a file verification process.
pub trait ProgressHandler {
    /// Called when no files are found in the directory.
    /// The default implementation does nothing.
    fn on_empty(&self) {}

    /// Sets the total number of items to process.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `total` - The total number of items to process.
    fn on_set_total(&mut self, total: usize) {
        let _ = total; // No-op by default
    }

    /// Increments the progress by a specified amount.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `progress` - The amount by which to increment the progress.
    fn inc(&self, progress: u64) {
        let _ = progress; // No-op by default
    }

    /// pat for the current progress state.
    /// The default implementation does nothing.
    fn on_processing_path(&self, path: &Path) {
        let _ = path; // No-op by default
    }

    /// Called when all files have been processed.
    /// The default implementation does nothing.
    fn on_finish(&self) {}

    /// Starts monitoring the progress with success and failure counts.
    /// The default implementation does nothing.
    fn start_progress_monitoring(
        &self,
        success_count: Arc<AtomicUsize>,
        failure_count: Arc<AtomicUsize>,
    ) {
        let _ = success_count; // No-op by default
        let _ = failure_count; // No-op by default
    }
}

/// A default implementation of the `ProgressHandler` trait that does nothing.
pub struct DefaultProgressMonitor;

impl ProgressHandler for DefaultProgressMonitor {}
