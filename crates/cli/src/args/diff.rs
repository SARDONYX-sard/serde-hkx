use std::path::PathBuf;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>XML diff -> stdout</blue!>
  <cyan!>hkxc diff</cyan!> ./defaultmale_by_hkxcmd.xml ./defaultmale_by_HavokBehaviorProcess.xml

- <blue!>hkx hexdump diff -> stdout</blue!>
  <cyan!>hkxc diff</cyan!> ./defaultmale_by_hkxcmd.hkx ./defaultmale_by_HavokBehaviorProcess.hkx

- <blue!>hexdump diff -> a file</blue!>
  <cyan!>hkxc diff</cyan!> ./defaultmale_x86_hexdump.txt ./defaultmale_x64_hexdump.txt <cyan!>-o</cyan!> diff.txt
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// Old path
    pub old: PathBuf,
    /// New path
    pub new: PathBuf,

    /// Output with ANSI color (For standard output)
    #[clap(long)]
    pub color: bool,

    /// If specified, write to a file (If not specified, stdout)
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}
