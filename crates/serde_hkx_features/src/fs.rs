//! Convenience file I/O.

use crate::error::{FailedReadFileSnafu, Result};
use snafu::ResultExt as _;
use std::borrow::Cow;
use std::io;
use std::{future::Future, io::Read as _, path::Path};
use tokio::fs;

/// Reads a file with some encoded string trait.
pub trait ReadExt {
    /// Reads a file with some encoded string.
    fn read_any_string(&self) -> impl Future<Output = Result<String>>;

    /// Read bytes.
    ///
    /// # Errors
    /// - If the path does not exist.
    /// - When an interrupt is received during reading.
    fn read_bytes(&self) -> impl Future<Output = Result<Vec<u8>>>;
}

impl<P> ReadExt for P
where
    P: AsRef<Path>,
{
    async fn read_any_string(&self) -> Result<String> {
        let input = self.as_ref();
        let mut string = String::new();

        let bytes = input.read_bytes().await?;
        let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
        decoder.read_to_string(&mut string)?;
        Ok(string)
    }

    async fn read_bytes(&self) -> Result<Vec<u8>> {
        let input = self.as_ref();
        fs::read(input).await.context(FailedReadFileSnafu {
            path: input.to_path_buf(),
        })
    }
}

/// Write specified or same location.
///
/// - `ext`: If `output` is unspecified, rewrite the `input` extension and make it the `output`. In that case, the extension.
///
/// # Errors
/// - Conflict error due to simultaneous dir creation.
/// - No write permission to the given path.
pub async fn write<I, O>(
    input: I,
    output: Option<O>,
    ext: &str,
    contents: impl AsRef<[u8]>,
) -> io::Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();

    let output = output.as_ref().map_or_else(
        || {
            let mut output = input.to_path_buf();
            output.set_extension(ext);
            Cow::Owned(output)
        },
        |output| Cow::Borrowed(output.as_ref()),
    );

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(&output, contents).await?;

    #[cfg(feature = "tracing")]
    tracing::info!("Input: {} -> Output: {}", input.display(), output.display());
    Ok(())
}

/// Write specified or same location.
///
/// - `ext`: If `output` is unspecified, rewrite the `input` extension and make it the `output`. In that case, the extension.
///
/// # Errors
/// - Conflict error due to simultaneous dir creation.
/// - No write permission to the given path.
pub fn write_sync<I, O>(
    input: I,
    output: Option<O>,
    ext: &str,
    contents: impl AsRef<[u8]>,
) -> io::Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();

    let output = output.as_ref().map_or_else(
        || {
            let mut output = input.to_path_buf();
            output.set_extension(ext);
            Cow::Owned(output)
        },
        |output| Cow::Borrowed(output.as_ref()),
    );

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&output, contents)?;

    #[cfg(feature = "tracing")]
    tracing::info!("Input: {} -> Output: {}", input.display(), output.display());
    Ok(())
}
