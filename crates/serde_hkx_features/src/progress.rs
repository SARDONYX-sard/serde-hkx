//! Progress bar trait

use std::path::Path;

/// A trait for handling progress updates during a file verification process.
pub trait ProgressHandler {
    /// Called when no files are found in the directory.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn on_empty(&self) {}

    /// Sets the total number of items to process.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn on_set_total(&mut self, total: usize) {
        let _ = total;
    }

    /// Increments the progress by a specified amount.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn inc(&self, progress: u64) {
        let _ = progress;
    }

    /// Increments success count.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn success_inc(&self, progress: u64) {
        let _ = progress;
    }

    /// Increments failure count.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn failure_inc(&self, progress: u64) {
        let _ = progress;
    }

    /// The path for the current progress state.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn on_processing_path(&self, path: &Path) {
        let _ = path; // No-op by default
    }

    /// Called when all files have been processed.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn on_finish(&self) {}
}

/// A default implementation of the `ProgressHandler` trait that does nothing.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultProgressMonitor;
impl ProgressHandler for DefaultProgressMonitor {}
