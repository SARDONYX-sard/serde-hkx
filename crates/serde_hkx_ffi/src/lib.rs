mod error;

use std::ffi::{CStr, c_char, c_int};
use std::path::PathBuf;

use serde_hkx_features::progress::DefaultProgressMonitor;

use crate::error::SerdeHkxError;

/// Output format for FFI.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OutputFormat(c_int);

impl TryFrom<OutputFormat> for serde_hkx_features::convert::OutFormat {
    type Error = SerdeHkxError;

    #[inline]
    fn try_from(v: OutputFormat) -> Result<Self, Self::Error> {
        match v.0 {
            0 => Ok(Self::Amd64),
            1 => Ok(Self::Win32),
            2 => Ok(Self::Xml),
            _ => Err(SerdeHkxError::InvalidArgument),
        }
    }
}

/// Convert a directory or file (hkx, xml).
///
/// - `input_path` : Path to input file or directory (UTF-8).
/// - `output_path`: Path to output file or directory (UTF-8). null then same place as `input_path`.
/// - `format`     : Output format.
///
/// # Returns
/// - `SerdeHkxError::Ok` on success.
/// - Other error codes on failure.
#[unsafe(no_mangle)]
pub extern "C" fn serde_hkx_ffi_convert(
    input_path: *const c_char,
    output_path: *const c_char,
    format: OutputFormat,
) -> SerdeHkxError {
    let input = match input_cstr_to_path(input_path) {
        Ok(input) => input,
        Err(err) => return err,
    };
    let output = match output_cstr_to_path(output_path) {
        Ok(output) => output,
        Err(err) => return err,
    };
    let format = match format.try_into() {
        Ok(format) => format,
        Err(err) => return err,
    };
    let result = serde_hkx_features::convert::rayon::convert_progress(
        input,
        output,
        format,
        DefaultProgressMonitor,
    );

    serde_hkx_result_to_ffi(result)
}

fn input_cstr_to_path(ptr: *const c_char) -> Result<PathBuf, SerdeHkxError> {
    if ptr.is_null() {
        return Err(SerdeHkxError::InvalidArgument);
    }

    let s = unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| SerdeHkxError::InvalidArgument)?;

    Ok(PathBuf::from(s))
}

fn output_cstr_to_path(ptr: *const c_char) -> Result<Option<PathBuf>, SerdeHkxError> {
    if ptr.is_null() {
        return Ok(None);
    }

    let s = unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| SerdeHkxError::InvalidArgument)?;

    Ok(Some(PathBuf::from(s)))
}

fn serde_hkx_result_to_ffi(result: Result<(), serde_hkx_features::error::Error>) -> SerdeHkxError {
    match result {
        Ok(()) => SerdeHkxError::Ok,
        Err(err) => SerdeHkxError::from(err),
    }
}
