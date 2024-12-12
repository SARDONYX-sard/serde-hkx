//! Verification of hkx -> Rust data type -> hkx conversion to original hkx

use crate::{
    diff,
    error::{
        DeSnafu, FailedReadFileSnafu, ReproduceHkxFilesSnafu, ReproduceHkxSnafu, Result, SerSnafu,
    },
    ClassMap,
};
use rayon::prelude::*;
use serde_hkx::bytes::{hexdump, serde::hkx_header::HkxHeader};
use snafu::ResultExt as _;
use std::path::{Path, PathBuf};

/// Checks reproduction for file/dir hkx.
///
/// # Errors
/// If an error occurs, return a diff showing the location of each error.(If `input` is one file path)
pub fn verify<I>(input: I, color: bool) -> Result<()>
where
    I: AsRef<Path>,
{
    let path = input.as_ref();

    if path.is_file() {
        verify_file(path, color)
    } else {
        verify_dir(path)
    }
}

/// Parallel checks reproduction for hkx files.
///
/// # Errors
/// If an error occurs, returns an array of error paths.
pub fn verify_dir(dir: &Path) -> Result<()> {
    let entries: Vec<_> = jwalk::WalkDir::new(dir).into_iter().collect();

    let results: Vec<(PathBuf, bool)> = entries
        .into_par_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let is_hkx = entry
                .path()
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("hkx"))
                .unwrap_or_default();
            entry.path().is_file() && is_hkx
        })
        .map(|entry| {
            let input = entry.path();
            let verify_result = verify_inner(&input);

            let is_valid = if let Ok((expected_bytes, actual_bytes)) = &verify_result {
                let is_valid = expected_bytes == actual_bytes;
                if is_valid {
                    #[cfg(feature = "tracing")]
                    tracing::info!("OK reproducible: {}", input.display());
                }
                is_valid
            } else {
                false
            };
            (input, is_valid)
        })
        .collect();
    let all_len = results.len();

    let err_paths: Vec<PathBuf> = results
        .into_par_iter()
        .filter_map(|(path, is_valid)| if !is_valid { Some(path) } else { None })
        .collect();

    if err_paths.is_empty() {
        Ok(())
    } else {
        ReproduceHkxFilesSnafu {
            path: dir.to_path_buf(),
            all_len,
            err_paths,
        }
        .fail()
    }
}

/// Check hkx reproduction.
///
/// # Errors
/// If an error occurs, return a input hexdump diff.
pub fn verify_file<I>(path: I, color: bool) -> Result<()>
where
    I: AsRef<Path>,
{
    let input = path.as_ref();

    let (expected_bytes, actual_bytes) = verify_inner(input)?;

    // Verify
    if actual_bytes == expected_bytes {
        #[cfg(feature = "tracing")]
        tracing::info!("OK reproducible: {}", input.display());
        return Ok(());
    }

    let actual = hexdump::to_string(&actual_bytes);
    let expected = hexdump::to_string(&expected_bytes);
    ReproduceHkxSnafu {
        path: input.to_path_buf(),
        diff: diff::diff(expected, actual, color),
    }
    .fail()
}

fn verify_inner<I>(input: I) -> Result<(Vec<u8>, Vec<u8>)>
where
    I: AsRef<Path>,
{
    let input = input.as_ref();

    let expected_bytes = std::fs::read(input).with_context(|_| FailedReadFileSnafu {
        path: input.to_path_buf(),
    })?;

    let actual_bytes = {
        let classes: ClassMap =
            serde_hkx::from_bytes(&expected_bytes).with_context(|_| DeSnafu { input })?;
        let header = HkxHeader::from_bytes(&expected_bytes).with_context(|_| DeSnafu { input })?;

        serde_hkx::to_bytes(&classes, &header).with_context(|_| SerSnafu { input })?
    };

    Ok((expected_bytes, actual_bytes))
}
