use super::ClassMap;
use crate::error::{Error, Result};
use serde_hkx::{from_bytes, from_str, tree};
use std::{ffi::OsStr, io::Read as _, path::Path};
use tokio::fs;
use tree::HavokTree as _;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>xml -> dependencies tree</blue!>
  <cyan!>hkxc tree</cyan!> ./defaultmale.xml

- <blue!>hkx -> dependencies tree</blue!>
  <cyan!>hkxc tree</cyan!> ./defaultmale.hkx <cyan!>--log-level</cyan!> trace <cyan!>--log-file</cyan!> ./hkx_tree.log
"#
);

#[derive(Debug, clap::Args)]
pub(crate) struct Tree {
    /// hkx/xml file path.
    pub input: String,
}

pub async fn print<P>(input: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let tree = gen(input).await?;
    tracing::trace!("tree = \n{tree}");
    println!("{tree}");
    Ok(())
}

/// Generate tree.
async fn gen<P>(input: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let input = input.as_ref();
    let extension = input.extension();
    let bytes = fs::read(input).await?;
    let mut xml = String::new();

    if extension == Some(OsStr::new("hkx")) {
        let mut classes: ClassMap = from_bytes(&bytes)?;
        Ok(classes.tree_for_bytes())
        //
    } else if extension == Some(OsStr::new("xml")) {
        let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
        decoder.read_to_string(&mut xml)?;
        let mut classes: ClassMap = from_str(&xml)?;
        Ok(classes.tree_for_bytes()) // TODO: implement `tree_for_xml`
                                     //
    } else {
        return Err(Error::UnknownExtension {
            path: input.to_string_lossy().to_string(),
        });
    }
}
