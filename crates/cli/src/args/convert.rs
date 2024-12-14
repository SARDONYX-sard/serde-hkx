//! Convert hkx <-> xml
use super::progress_handler::CliProgressHandler;
use clap::ValueEnum;
use serde_hkx_features::{convert::OutFormat, error::Result};
use std::path::{Path, PathBuf};

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>hkx -> xml</blue!>
  <cyan!>hkxc convert --input</cyan!> ./defaultmale.hkx <cyan!>--format</cyan!> xml

- <blue!>xml -> hkx(64bit)</blue!>
  <cyan!>hkxc convert -i</cyan!> ./defaultmale.xml <cyan!>-v</cyan!> amd64 <cyan!>--stdout --log-level</cyan!> trace

- <blue!>hkx(32bit) -> hkx(64bit)</blue!>
  <cyan!>hkxc convert -i</cyan!> ./defaultmale_x86.hkx <cyan!>-o</cyan!> ./defaultmale_x64.hkx <cyan!>-v</cyan!> amd64 <cyan!>--log-level</cyan!> debug <cyan!>--log-file</cyan!> "./convert_x86_to_x64_bytes.log"

- <blue!>hkx(64bit) -> hkx(32bit)</blue!>
  <cyan!>hkxc convert -i</cyan!> ./defaultmale_x64.hkx <cyan!>-o</cyan!> ./defaultmale_x86.hkx <cyan!>-v</cyan!> win32 <cyan!>--log-level</cyan!> trace <cyan!>--log-file</cyan!> ./convert_x64_to_x86_bytes.log
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// Path containing the hkx/xml file/directory
    #[clap(short, long)]
    pub input: PathBuf,
    /// Output path (If not specified, it is automatically created in the same location as the input path.)
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// File format to output
    #[clap(short = 'v', long, ignore_case = true)]
    pub format: OutFormat,

    /// Execution runtime: async (Tokio) or parallel (Rayon) (NOTE: Effective only when dir path is selected.)
    #[arg(short, long, ignore_case = true, default_value = "rayon")]
    pub runtime: Runtime,
}

/// Runtime options
#[derive(ValueEnum, Clone, Debug)]
pub enum Runtime {
    /// Uses Rayon for multi-threaded parallel processing & Enumerate error paths
    Rayon,
    /// Uses Tokio for async processing(M:N thread) & fail fast
    Tokio,
}

pub async fn convert<I, O>(
    input: I,
    output: Option<O>,
    format: OutFormat,
    runtime: Runtime,
) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path> + Sync,
{
    match runtime {
        Runtime::Rayon => rayon_convert(&input, output, format),
        Runtime::Tokio => tokio_convert(&input, output, format).await,
    }
}

fn rayon_convert<I, O>(input: I, output: Option<O>, format: OutFormat) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path> + Sync,
{
    serde_hkx_features::convert::rayon::convert_progress(
        input,
        output,
        format,
        CliProgressHandler::new(),
    )
}

/// convert with cli progress.
pub async fn tokio_convert<I, O>(input: I, output: Option<O>, format: OutFormat) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    serde_hkx_features::convert::tokio::convert_progress(
        input,
        output,
        format,
        CliProgressHandler::new(),
    )
    .await
}
