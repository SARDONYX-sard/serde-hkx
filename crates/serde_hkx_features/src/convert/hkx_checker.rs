//! Check the HKX file header to determine its format.
use std::path::{Path, PathBuf};

use crate::OutFormat;

/// Check the HKX file header to determine its format.
///
/// # Note
/// use IO operations internally; avoid calling this function within `rayon::par_iter` directly.
///
/// # Errors
/// - If hkx magic number is invalid
pub fn check_hkx_header(input_path: &Path, output_format: OutFormat) -> Result<OutFormat, Error> {
    let header = {
        let mut header = [0_u8; 17];
        let mut file = std::fs::File::open(input_path).map_err(|source| Error::FailedReadFile {
            source,
            path: input_path.to_path_buf(),
        })?;
        std::io::Read::read_exact(&mut file, &mut header).map_err(|source| {
            Error::FailedReadFile {
                source,
                path: input_path.to_path_buf(),
            }
        })?;
        header
    };

    // Actually, both LE and SE versions of hkt can be loaded, and there are mods disguised as hkx files. Example: Ride Sharing's `rsh_horsepinion.hkx`
    // This is the processing for that.
    // NOTE: Tag files cannot be converted by serde_hkx, so they are skipped.
    let is_tag_file = {
        /// Tag file(.hkt) magic bytes
        const EXPECTED_MAGIC: [u8; 8] = [
            0x1E, 0x0D, 0xB0, 0xCA, // magic0
            0xCE, 0xFA, 0x11, 0xD0, // magic1
        ];
        header[0..8] == EXPECTED_MAGIC
    };
    if is_tag_file {
        #[cfg(feature = "tracing")]
        tracing::info!(
            path = %input_path.display(),
            "Tag files cannot be converted by serde_hkx, so they are skipped."
        );
        return Ok(output_format);
    }

    let is_hkx = {
        /// .hkx magic bytes
        const EXPECTED_MAGIC: [u8; 8] = [
            0x57, 0xE0, 0xE0, 0x57, // magic0
            0x10, 0xC0, 0xC0, 0x10, // magic1
        ];
        header[0..8] == EXPECTED_MAGIC
    };
    if !is_hkx {
        return Err(Error::HkxInvalidMagic {
            input_path: input_path.to_path_buf(),
            target: output_format,
            magic_bytes: header,
        });
    }

    // check ptr size
    let ptr_size = header[16];
    let current_format = match ptr_size {
        4 => OutFormat::Win32,
        8 => OutFormat::Amd64,
        _ => {
            return Err(Error::HkxInvalidHeader {
                input_path: input_path.to_path_buf(),
                target: output_format,
                actual: ptr_size,
            });
        }
    };

    Ok(current_format)
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// Failed to read this path.
    #[snafu(display("Failed to read this path: {}", path.display()))]
    FailedReadFile {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display(
        "HKX for target {target:?}, the file {} did not have the expected Havok magic numbers. \
        Expected magic=[0x57, 0xe0, 0xe0, 0x57, 0x10, 0xc0, 0xc0, 0x10, ...], \
        but got {magic_bytes:x?}. \
        This file is not a valid Havok animation or may be from an unsupported version.",
        input_path.display()
    ))]
    HkxInvalidMagic {
        input_path: PathBuf,
        target: OutFormat,
        magic_bytes: [u8; 17],
    },

    #[snafu(display(
        "HKX for target {target:?}, pointer size check failed for {}. \
        Expected pointer size 4/8-byte for {target:?}, \
        but could not determine a valid header or got {actual}-byte. \
        The HKX may be malformed or from an incompatible platform.",
        input_path.display()
    ))]
    HkxInvalidHeader {
        input_path: PathBuf,
        target: OutFormat,
        actual: u8,
    },
}
