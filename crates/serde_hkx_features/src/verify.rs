//! Verification of hkx -> Rust data type -> hkx conversion to original hkx

use crate::{
    diff,
    error::{
        DeSnafu, Error, FailedReadFileSnafu, ReproduceHkxFilesSnafu, ReproduceHkxSnafu, Result,
        SerSnafu,
    },
    ClassMap,
};
use rayon::prelude::*;
use serde_hkx::bytes::{hexdump, serde::hkx_header::HkxHeader};
use snafu::ResultExt as _;
use std::path::{Path, PathBuf};

/// Parallel hkx reproduction checks.
///
/// # Errors
/// If an error occurs, return a diff showing the location of each error.
pub fn verify<I>(input: I, color: bool) -> Result<()>
where
    I: AsRef<Path>,
{
    let path = input.as_ref();

    if path.is_file() {
        verify_file(path, color)
    } else {
        verify_dir(path, color)
    }
}

/// Parallel hkx reproduction checks.
///
/// # Errors
/// If an error occurs, return a diff showing the location of each error.
pub fn verify_dir(input: &Path, color: bool) -> Result<()> {
    let entries: Vec<_> = jwalk::WalkDir::new(input).into_iter().collect();

    let results: Vec<(PathBuf, Result<(), Error>)> = entries
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
            (input.clone(), verify_file(&input, color))
        })
        .collect();
    let all_len = results.len();

    let errors: Vec<PathBuf> = results
        .into_par_iter()
        .filter_map(|(path, result)| result.err().map(|_| path))
        .collect();

    #[cfg(feature = "tracing")]
    tracing::error!(
        "Reproduce err_paths(err count: {}/{all_len}): {errors:#?}",
        errors.len()
    );

    if errors.is_empty() {
        Ok(())
    } else {
        ReproduceHkxFilesSnafu {
            path: input.to_path_buf(),
            err_paths: errors,
        }
        .fail()
    }
}

/// Check hkx reproduction.
///
/// # Errors
/// If an error occurs, return a input hexdump diff.
pub fn verify_file<I>(input: I, color: bool) -> Result<()>
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

    // Verify
    if actual_bytes != expected_bytes {
        let actual = hexdump::to_string(&actual_bytes);
        let expected = hexdump::to_string(&expected_bytes);
        return ReproduceHkxSnafu {
            path: input.to_path_buf(),
            diff: diff::diff(expected, actual, color),
        }
        .fail();
    }

    #[cfg(feature = "tracing")]
    tracing::error!("OK reproducible: {}", input.display());
    Ok(())
}
