//! Dump binary data in hexadecimal
use std::path::PathBuf;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>hkx -> hexdump -> stdout</blue!>
  <cyan!>hkxc dump</cyan!> ./defaultmale.hkx

- <blue!>hkx -> hexdump -> a file</blue!>
  <cyan!>hkxc dump</cyan!> ./defaultmale.hkx <cyan!>-o</cyan!> hexdump.txt

 - <blue!>hexdump -> hkx -> a file</blue!><yellow>(NOTE: For reverse conversion, use `-o` to specify the output path.)</yellow>
  <cyan!>hkxc dump</cyan!> hexdump.txt <cyan!>-o</cyan!> ./defaultmale.hkx -r
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// hkx file path/hexdump file path(When use `--reverse`)
    pub input: PathBuf,
    /// If specified, write to a file (If not specified, stdout)
    #[clap(short, long)]
    #[clap(long_help = "Output path to write a file.
NOTE: Bytes cannot be written to stdout, so always use `-o` to specify the output when use `--reverse`.")]
    pub output: Option<PathBuf>,

    /// Reverse conversion from hexdump to bytes
    #[clap(short, long)]
    pub reserve: bool,
}
