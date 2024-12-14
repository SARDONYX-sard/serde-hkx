use indicatif::ProgressBar;
use serde_hkx_features::{error::Result, verify::ProgressHandler};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"In case of error, dir and file behave differently
    - dir: All error paths are displayed
    - file: Error path + diff displayed

<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>Test the reproducibility of hkx</blue!>
  <cyan!>hkxc verify</cyan!> ./defaultmale.hkx

- <blue!>Test the reproducibility of hkx files</blue!>
  <cyan!>./hkxc verify</cyan!> ./input --log-level info --log-file "./verify_inputs.log"
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// hkx file path or dir for which you want to verify reproducibility. It is recommended to use log in INFO mode.
    pub path: PathBuf,
    /// Option to make the diff display colorful in case of error when file is specified.
    #[clap(long)]
    pub color: bool,
}

pub fn verify<P>(path: P, color: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    println!("Verifying...");
    serde_hkx_features::verify::verify(path, color, CliProgressHandler::new()).map(|_| {
        color_print::cprintln!(
            "<green>Complete hkx reproduction: {}</green>",
            path.display()
        );
    })
}

struct CliProgressHandler {
    progress_bar: ProgressBar,
    success_bar: ProgressBar,
    failure_bar: ProgressBar,
}

impl CliProgressHandler {
    fn new() -> Self {
        use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

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

// NOTE: We don't try it except local PC because MIRI makes an error. Therefore, leave it commented out.
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(miri, ignore = "need templates")]
    #[test]
    fn test_verify_dir() {
        let input = "../../tests/input";
        verify(input, false).unwrap_or_else(|err| panic!("{err}"));
    }
}
