//! Progress bar

use std::{
    path::Path,
    sync::{atomic::AtomicUsize, Arc},
};

/// A trait for handling progress updates during a file verification process.
pub trait ProgressHandler {
    /// Called when no files are found in the directory.
    /// The default implementation does nothing.
    #[inline]
    fn on_empty(&self) {}

    /// Sets the total number of items to process.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `total` - The total number of items to process.
    #[inline]
    fn on_set_total(&mut self, total: usize) {
        let _ = total; // No-op by default
    }

    /// Increments the progress by a specified amount.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `progress` - The amount by which to increment the progress.
    #[inline]
    fn inc(&self, progress: u64) {
        let _ = progress; // No-op by default
    }

    /// pat for the current progress state.
    /// The default implementation does nothing.
    #[inline]
    fn on_processing_path(&self, path: &Path) {
        let _ = path; // No-op by default
    }

    /// Called when all files have been processed.
    /// The default implementation does nothing.
    #[inline]
    fn on_finish(&self) {}

    /// Starts monitoring the progress with success and failure counts.
    /// The default implementation does nothing.
    #[inline]
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultProgressMonitor;

impl ProgressHandler for DefaultProgressMonitor {}
