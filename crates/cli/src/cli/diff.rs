use crate::error::Result;
use crate::read_ext::ReadExt as _;
use std::path::Path;
use tokio::fs;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>diff -> stdout</blue!>
  <cyan!>hkxc diff</cyan!> ./defaultmale_x86_hexdump.txt ./defaultmale_x64_hexdump.txt

- <blue!>diff -> a file</blue!>
  <cyan!>hkxc diff</cyan!> ./defaultmale_x86_hexdump.txt ./defaultmale_x64_hexdump.txt <cyan!>-o</cyan!> diff.txt
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// old path
    pub old: String,
    /// new path
    pub new: String,

    /// If specified, write to a file (If not specified, stdout)
    #[clap(short, long)]
    pub output: Option<String>,
}

/// Show diff between two files.
pub async fn exec<I1, I2, O>(old: I1, new: I2, output: Option<O>) -> Result<()>
where
    I1: AsRef<Path>,
    I2: AsRef<Path>,
    O: AsRef<Path>,
{
    let old_s = old.read_any_string().await?;
    let new_s = new.read_any_string().await?;

    let diff_str = diff(old_s, new_s);
    match output {
        Some(output) => fs::write(output, &diff_str).await?,
        None => print!("{diff_str}"),
    };
    Ok(())
}

fn diff(old: impl AsRef<str>, new: impl AsRef<str>) -> String {
    let diff = similar::TextDiff::from_lines(old.as_ref(), new.as_ref());
    let mut output_diff = String::new();
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            similar::ChangeTag::Delete => "-",
            similar::ChangeTag::Insert => "+",
            similar::ChangeTag::Equal => " ",
        };
        output_diff += &format!("{sign}{change}");
    }
    output_diff
}
