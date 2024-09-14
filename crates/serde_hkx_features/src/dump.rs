//! Dump binary data in hexadecimal
use crate::{
    error::{Error, Result},
    read_ext::ReadExt as _,
};
pub use serde_hkx::bytes::hexdump;
use std::path::Path;
use tokio::fs;

/// Output bytes to stdout/file.
/// - `output`: If not provided, then stdout.
pub async fn to_bytes<I, O>(input: I, output: Option<O>) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    match output {
        Some(output) => {
            let bytes = hexdump::to_bytes(&input.read_any_string().await?);
            fs::write(output, &bytes).await?
        }
        None => return Err(Error::InvalidStdout),
    };
    Ok(())
}

/// Output hexdump to stdout/file.
/// - `output`: If not provided, then stdout.
pub async fn to_string<I, O>(input: I, output: Option<O>) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let hexdump = hexdump::to_string(input.read_bytes().await?); // NOTE: With newline.
    match output {
        Some(output) => fs::write(output, &hexdump).await?,
        None => print!("{hexdump}"),
    };
    Ok(())
}
