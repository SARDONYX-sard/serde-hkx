//! Convert hkx <-> xml
use serde_hkx_features::convert::OutFormat;
use std::path::PathBuf;

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
}
