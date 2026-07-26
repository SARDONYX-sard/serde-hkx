//! Show dependency tree from havok behavior state machine (hkx/xml file)
use crate::{
    error::{Error, SerSnafu},
    fs::ReadExt as _,
    serde::ser::HkxSnafu,
};
use serde_hkx::tree::HavokTree as _;
use snafu::ResultExt;
use std::path::Path;
use tokio::fs;

/// Output reference tree to stdout/file.
/// - `output`: If not provided, then stdout.
///
/// # Errors
/// If the extension is not `hkx` or `xml`.
pub async fn write_tree<I, O>(input: I, output: Option<O>) -> Result<(), Error>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let tree = generate(input).await?; // NOTE: With newline
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
#[inline]
pub async fn generate<P>(input: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let input = input.as_ref();

    let bytes = input.read_bytes().await?;
    crate::convert::process_serde_with(
        &bytes,
        input,
        |mut c| {
            c.tree_for_bytes()
                .with_context(|_| HkxSnafu {})
                .with_context(|_| SerSnafu { input })
        },
        |mut c| {
            c.tree_for_bytes()
                .with_context(|_| HkxSnafu {})
                .with_context(|_| SerSnafu { input })
        },
    )
}
