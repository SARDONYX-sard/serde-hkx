use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde_hkx_features::verify::ProgressHandler;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// For CLI progress bar
#[derive(Debug, Clone)] // need clone for `tokio::spawn`
pub struct CliProgressHandler {
    progress_bar: ProgressBar,
    success_bar: ProgressBar,
    failure_bar: ProgressBar,
    success_count: Arc<AtomicU64>,
    failure_count: Arc<AtomicU64>,
}

impl CliProgressHandler {
    /// Create a new `CliProgressHandler`.
    pub fn new() -> Self {
        let multi_progress = MultiProgress::new();
        let progress_bar = multi_progress.add(ProgressBar::no_length());
        let success_bar = multi_progress.add(ProgressBar::no_length());
        let failure_bar = multi_progress.add(ProgressBar::no_length());

        // Attempt to set the template and handle errors
        match ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
        {
            Ok(style) => {
                progress_bar.set_style(style.progress_chars("#>-"));
            }
            Err(e) => {
                progress_bar.set_style(ProgressStyle::default_bar().progress_chars("#>-"));
                tracing::error!("Failed to set progress bar template: {e}");
            }
        }

        match ProgressStyle::default_spinner().template("✔ Success: {pos}") {
            Ok(style) => success_bar.set_style(style),
            Err(e) => {
                success_bar.set_style(ProgressStyle::default_spinner());
                tracing::error!("Failed to set success bar template: {e}");
            }
        }

        match ProgressStyle::default_spinner().template("✖ Failed: {pos}") {
            Ok(style) => failure_bar.set_style(style),
            Err(e) => {
                failure_bar.set_style(ProgressStyle::default_spinner());
                tracing::error!("Failed to set failure bar template: {e}");
            }
        }

        Self {
            progress_bar,
            success_bar,
            failure_bar,
            success_count: Arc::new(AtomicU64::new(0)),
            failure_count: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl ProgressHandler for CliProgressHandler {
    fn on_empty(&self) {
        println!("No files found in the directory to process. Please check if the directory contains valid files.");
    }

    fn on_set_total(&mut self, total: usize) {
        let total = total as u64;
        self.progress_bar.set_length(total);
        self.success_bar.set_length(total);
        self.failure_bar.set_length(total);

        let success_bar = self.success_bar.clone();
        let failure_bar = self.failure_bar.clone();
        let success_count = Arc::clone(&self.success_count);
        let failure_count = Arc::clone(&self.failure_count);
        std::thread::spawn({
            move || loop {
                success_bar.set_position(success_count.load(Ordering::Acquire));
                failure_bar.set_position(failure_count.load(Ordering::Acquire));
            }
        });
    }

    fn inc(&self, progress: u64) {
        self.progress_bar.inc(progress);
    }

    fn success_inc(&self, progress: u64) {
        self.success_count.fetch_add(progress, Ordering::AcqRel);
    }

    fn failure_inc(&self, progress: u64) {
        self.failure_count.fetch_add(progress, Ordering::AcqRel);
    }

    fn on_processing_path(&self, path: &Path) {
        self.progress_bar.set_message(format!("{}", path.display()));
    }

    fn on_finish(&self) {
        self.progress_bar.finish_with_message("All files processed");
    }
}
