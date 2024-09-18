//! Show dependency tree from havok behavior state machine (hkx/xml file)
use crate::{error::Result, fs::ReadExt, serde::deserialize};
use serde_hkx::tree::HavokTree as _;
use std::path::Path;
use tokio::fs;

/// Output reference tree to stdout/file.
/// - `output`: If not provided, then stdout.
///
/// # Errors
/// If the extension is not `hkx` or `xml`.
pub async fn output<I, O>(input: I, output: Option<O>) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let tree = gen(input).await?; // NOTE: With newline
    match output.as_ref() {
        Some(output) => fs::write(output, &tree).await?,
        None => print!("{tree}"),
    };
    Ok(())
}

/// Generate reference tree.
///
/// # Errors
/// If the unknown extension. (Not `hkx`, `xml`...).
pub async fn gen<P>(input: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let bytes = input.read_bytes().await?;
    let mut string = String::new();
    Ok(deserialize(&bytes, &mut string, input)?.tree_for_bytes())
}
