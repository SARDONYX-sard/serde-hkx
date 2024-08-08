use crate::error::{FailedReadFileSnafu, Result};
use snafu::ResultExt as _;
use std::{future::Future, io::Read as _, path::Path};
use tokio::fs;

/// Reads a file with some encoded string trait.
pub trait ReadExt {
    /// Reads a file with some encoded string.
    fn read_any_string(&self) -> impl Future<Output = Result<String>>;

    /// Read bytes.
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
