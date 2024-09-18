//! Show diff between two files.

use crate::error::{Error, Result};
use crate::fs::ReadExt as _;
use serde_hkx::bytes::hexdump;
use std::path::Path;
use tokio::fs;

/// Show diff between two files.
/// - `use_color`: ANSI color diff. (red & green)
pub async fn exec<I1, I2, O>(old: I1, new: I2, output: Option<O>, use_color: bool) -> Result<()>
where
    I1: AsRef<Path>,
    I2: AsRef<Path>,
    O: AsRef<Path>,
{
    let old_str = read_any_to_string(old).await?;
    let new_str = read_any_to_string(new).await?;

    let diff_str = diff(old_str, new_str, use_color);
    match output {
        Some(output) => fs::write(output, &diff_str).await?,
        None => print!("{diff_str}"),
    };
    Ok(())
}

/// extension
/// - `hkx` -> Hexdump string
/// - else -> Any encode string
///
/// # Errors
/// Not found extension.
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

/// Show diff between two files.
///
/// - `color`: ANSI color diff. (red & green)
pub fn diff(old: impl AsRef<str>, new: impl AsRef<str>, color: bool) -> String {
    let mut output_diff = String::new();

    let diff = similar::TextDiff::from_lines(old.as_ref(), new.as_ref());
    for change in diff.iter_all_changes() {
        let (sign, end) = if color {
            const DELETE: &str = "\u{1b}[31m-"; // 31 is red
            const INSERT: &str = "\u{1b}[32m+"; // 32 is green
            const RESET_COLOR: &str = "\u{1b}[39m";

            let sign = match change.tag() {
                similar::ChangeTag::Delete => DELETE,
                similar::ChangeTag::Insert => INSERT,
                similar::ChangeTag::Equal => " ",
            };
            (sign, RESET_COLOR)
        } else {
            let sign = match change.tag() {
                similar::ChangeTag::Delete => "-",
                similar::ChangeTag::Insert => "+",
                similar::ChangeTag::Equal => " ",
            };
            (sign, "")
        };
        output_diff += &format!("{sign}{change}{end}");
    }
    output_diff
}
