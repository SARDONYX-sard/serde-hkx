use crate::error::{Error, Result};
use crate::read_ext::ReadExt as _;
use serde_hkx::bytes::hexdump;
use std::path::{Path, PathBuf};
use tokio::fs;

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

    /// If specified, write to a file (If not specified, stdout)
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

/// Show diff between two files.
pub async fn exec<I1, I2, O>(old: I1, new: I2, output: Option<O>) -> Result<()>
where
    I1: AsRef<Path>,
    I2: AsRef<Path>,
    O: AsRef<Path>,
{
    let old_str = read_any_to_string(old).await?;
    let new_str = read_any_to_string(new).await?;

    let diff_str = diff(old_str, new_str);
    match output {
        Some(output) => fs::write(output, &diff_str).await?,
        None => print!("{diff_str}"),
    };
    Ok(())
}

/// extension
/// - `hkx` -> Hexdump string
/// - else -> Any encode string
async fn read_any_to_string<I>(path: I) -> Result<String>
where
    I: AsRef<Path>,
{
    let path = path.as_ref();
    let ext = path.extension();

    if let Some(ext) = ext {
        if ext.eq_ignore_ascii_case("hkx") {
            Ok(hexdump::to_string(path.read_bytes().await?))
        } else {
            path.read_any_string().await
        }
    } else {
        Err(Error::MissingExtension {
            path: path.to_path_buf(),
        })
    }
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
