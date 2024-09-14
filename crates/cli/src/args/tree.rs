//! Show dependency tree from havok behavior state machine (hkx/xml file)
use std::path::PathBuf;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>xml -> dependencies tree -> stdout</blue!>
  <cyan!>hkxc tree</cyan!> ./defaultmale.xml

- <blue!>hkx -> dependencies tree -> stdout(+log)</blue!>
  <cyan!>hkxc tree</cyan!> ./defaultmale.hkx <cyan!>--log-level</cyan!> trace <cyan!>--log-file</cyan!> ./hkx_tree.log

- <blue!>hkx -> dependencies tree -> a file</blue!>
  <cyan!>hkxc tree</cyan!> ./defaultmale.hkx <cyan!>-o</cyan!> tree.txt
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// hkx/xml file path
    pub input: PathBuf,
    /// If specified, write to a file (If not specified, stdout)
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}
