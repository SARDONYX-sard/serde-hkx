use crate::error::{FailedReadFileSnafu, Result};
use snafu::ResultExt as _;
use std::{future::Future, io::Read as _, path::Path};
use tokio::fs;

/// Reads a file with some encoded string trait.
pub trait EncodeRead {
    /// Reads a file with some encoded string.
    fn read_any_string(&self) -> impl Future<Output = Result<String>>;
}

impl<P> EncodeRead for P
where
    P: AsRef<Path>,
{
    async fn read_any_string(&self) -> Result<String> {
        let input = self.as_ref();
        let mut string = String::new();

        let bytes = fs::read(input).await.context(FailedReadFileSnafu {
            path: input.to_path_buf(),
        })?;
        let mut decoder = encoding_rs_io::DecodeReaderBytes::new(bytes.as_slice());
        decoder.read_to_string(&mut string)?;
        Ok(string)
    }
}
