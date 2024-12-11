use std::path::PathBuf;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"In case of error, dir and file behave differently
    - dir: All error paths are displayed
    - file: Error path + diff displayed

<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>XML verify -> stdout</blue!>
  <cyan!>hkxc verify</cyan!> ./defaultmale_by_hkxcmd.xml ./defaultmale_by_HavokBehaviorProcess.xml

- <blue!>hkx hexdump verify -> stdout</blue!>
  <cyan!>hkxc verify</cyan!> ./defaultmale_by_hkxcmd.hkx ./defaultmale_by_HavokBehaviorProcess.hkx

- <blue!>hexdump verify -> a file</blue!>
  <cyan!>hkxc verify</cyan!> ./defaultmale_x86_hexdump.txt ./defaultmale_x64_hexdump.txt <cyan!>-o</cyan!> verify.txt
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
