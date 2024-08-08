//! Show dependency tree from havok behavior state machine (hkx/xml file)
use super::ClassMap;
use crate::error::{Error, FailedReadFileSnafu, Result};
use serde_hkx::{from_bytes, from_str, tree};
use snafu::ResultExt as _;
use std::{ffi::OsStr, io::Read as _, path::Path};
use tokio::fs;
use tree::HavokTree as _;

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
    pub input: String,
    /// If specified, write to a file (If not specified, stdout)
    #[clap(short, long)]
    pub output: Option<String>,
}

/// Output tree to stdout/file.
pub async fn output<I, O>(input: I, output: Option<O>) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let tree = gen(input).await?; // NOTE: With newline
    match output.as_ref() {
        Some(output) => tokio::fs::write(output, &tree).await?,
        None => print!("{tree}"),
    };
    Ok(())
}

/// Generate tree.
async fn gen<P>(input: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let input = input.as_ref();
    let extension = input.extension();
    let bytes = fs::read(input).await.context(FailedReadFileSnafu {
        path: input.to_path_buf(),
    })?;
    let mut xml = String::new();

    if extension == Some(OsStr::new("hkx")) {
        let mut classes: ClassMap = from_bytes(&bytes)?;
        Ok(classes.tree_for_bytes())
    } else if extension == Some(OsStr::new("xml")) {
        let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
        decoder.read_to_string(&mut xml)?;
        let mut classes: ClassMap = from_str(&xml)?;
        Ok(classes.tree_for_bytes()) // TODO: implement `tree_for_xml`
    } else {
        return Err(Error::UnsupportedExtension {
            path: input.to_string_lossy().to_string(),
        });
    }
}
