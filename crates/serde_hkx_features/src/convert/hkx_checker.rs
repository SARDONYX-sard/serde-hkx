//! Check the HKX file header to detect its format.
use std::path::{Path, PathBuf};

use crate::Format;

/// Check the HKX file header to detect its format(`Win32`/`Amd64`).
///
/// # Errors
/// - If tag file magic number
/// - If hkx magic number is invalid
pub fn detect_hkx_format(input_path: &Path) -> Result<Format, Error> {
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
        return Err(Error::UnsupportedTagFile {
            path: input_path.to_path_buf(),
        });
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
            magic_bytes: header,
        });
    }

    // check ptr size
    let ptr_size = header[16];
    let current_format = match ptr_size {
        4 => Format::Win32,
        8 => Format::Amd64,
        _ => {
            return Err(Error::HkxInvalidHeader {
                input_path: input_path.to_path_buf(),
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
    #[snafu(display("serde-hkx does not currently support tag files. path: {}", path.display()))]
    UnsupportedTagFile { path: PathBuf },

    /// Failed to read this path.
    #[snafu(display("Failed to read this path: {}", path.display()))]
    FailedReadFile {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display(
        "HKX file {} did not have the expected Havok magic numbers. \
        Expected magic=[0x57, 0xe0, 0xe0, 0x57, 0x10, 0xc0, 0xc0, 0x10, ...], \
        but got {magic_bytes:x?}. \
        This file is not a valid Havok animation or may be from an unsupported version.",
        input_path.display()
    ))]
    HkxInvalidMagic {
        input_path: PathBuf,
        magic_bytes: [u8; 17],
    },

    #[snafu(display(
        "HKX pointer size check failed for {}. \
        Expected pointer size 4/8-byte, but could not determine a valid header or got {actual}-byte. \
        The HKX may be malformed or from an incompatible platform.",
        input_path.display()
    ))]
    HkxInvalidHeader { input_path: PathBuf, actual: u8 },
}
