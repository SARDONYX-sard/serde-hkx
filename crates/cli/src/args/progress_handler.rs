use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde_hkx_features::verify::ProgressHandler;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// For CLI progress bar
#[derive(Debug, Clone)] // need clone for `tokio::spawn`
pub struct CliProgressHandler {
    progress_bar: ProgressBar,
    success_bar: ProgressBar,
    failure_bar: ProgressBar,
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

        let default_spinner = ProgressStyle::default_spinner();
        match default_spinner.clone().template("✔ Success: {pos}") {
            Ok(style) => success_bar.set_style(style),
            Err(e) => {
                success_bar.set_style(default_spinner.clone());
                tracing::error!("Failed to set success bar template: {e}");
            }
        }

        match default_spinner.clone().template("✖ Failed: {pos}") {
            Ok(style) => success_bar.set_style(style),
            Err(e) => {
                success_bar.set_style(default_spinner);
                tracing::error!("Failed to set failure bar template: {e}");
            }
        }

        Self {
            progress_bar,
            success_bar,
            failure_bar,
        }
    }
}

impl ProgressHandler for CliProgressHandler {
    fn on_set_total(&mut self, total: usize) {
        self.progress_bar.set_length(total as u64);
        self.success_bar.set_length(total as u64);
        self.failure_bar.set_length(total as u64);
    }

    fn inc(&self, progress: u64) {
        self.progress_bar.inc(progress);
    }

    fn on_processing_path(&self, path: &Path) {
        self.progress_bar.set_message(format!("{}", path.display()));
    }

    fn on_finish(&self) {
        self.progress_bar.finish_with_message("All files processed");
    }

    fn start_progress_monitoring(
        &self,
        success_count: Arc<AtomicUsize>,
        failure_count: Arc<AtomicUsize>,
    ) {
        let success_bar = self.success_bar.clone();
        let failure_bar = self.failure_bar.clone();
        std::thread::spawn({
            move || loop {
                success_bar.set_position(success_count.load(Ordering::Acquire) as u64);
                failure_bar.set_position(failure_count.load(Ordering::Acquire) as u64);
            }
        });
    }
}
