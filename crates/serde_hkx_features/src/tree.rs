//! Show dependency tree from havok behavior state machine (hkx/xml file)
use super::ClassMap;
use crate::{
    error::{DeSnafu, Error, Result},
    read_ext::ReadExt,
};
use serde_hkx::{from_bytes, from_str, tree::HavokTree as _};
use snafu::ResultExt as _;
use std::{io::Read as _, path::Path};

/// Output reference tree to stdout/file.
/// - `output`: If not provided, then stdout.
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

/// Generate reference tree.
pub async fn gen<P>(input: P) -> Result<String>
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
