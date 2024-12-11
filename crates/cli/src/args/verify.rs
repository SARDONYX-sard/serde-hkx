use std::path::PathBuf;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"In case of error, dir and file behave differently
    - dir: All error paths are displayed
    - file: Error path + diff displayed

<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>Test the reproducibility of hkx</blue!>
  <cyan!>hkxc verify</cyan!> ./defaultmale.hkx

- <blue!>Test the reproducibility of hkx files</blue!>
  <cyan!>hkxc verify</cyan!> ./
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// hkx file path or dir for which you want to verify reproducibility.
    pub path: PathBuf,

    /// Option to make the diff display colorful in case of error when file is specified.
    #[clap(long)]
    pub color: bool,
}
