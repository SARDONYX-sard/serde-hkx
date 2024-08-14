//! Show dependency tree from havok behavior state machine (hkx/xml file)
use super::ClassMap;
use crate::{
    error::{DeSnafu, Error, Result},
    read_ext::ReadExt,
};
use serde_hkx::{from_bytes, from_str, tree::HavokTree as _};
use snafu::ResultExt as _;
use std::{
    io::Read as _,
    path::{Path, PathBuf},
};

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
    let bytes = input.read_bytes().await?;
    let mut xml = String::new();

    if let Some(extension) = extension {
        if extension.eq_ignore_ascii_case("hkx") {
            let mut classes: ClassMap = from_bytes(&bytes).context(DeSnafu {
                input: input.to_path_buf(),
            })?;
            Ok(classes.tree_for_bytes())
            //
        } else if extension.eq_ignore_ascii_case("xml") {
            let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
            decoder.read_to_string(&mut xml)?;
            let mut classes: ClassMap = from_str(&xml).context(DeSnafu {
                input: input.to_path_buf(),
            })?;
            Ok(classes.tree_for_bytes())
            //
        } else {
            Err(Error::UnsupportedExtension {
                path: input.to_path_buf(),
            })
        }
    } else {
        Err(Error::UnsupportedExtension {
            path: input.to_path_buf(),
        })
    }
}
