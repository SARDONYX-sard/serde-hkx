use crate::{
    diff,
    error::{FailedReproduceFileSnafu, Result},
};
use serde_hkx::bytes::hexdump;
use std::path::Path;

use super::verify_inner;

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
    FailedReproduceFileSnafu {
        path: input.to_path_buf(),
        diff: diff::diff(expected, actual, color),
    }
    .fail()
}

// NOTE: We don't try it except local PC because MIRI makes an error. Therefore, leave it commented out.
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "need templates"]
    #[test]
    #[quick_tracing::init(file = "../../tests/verify_file.log", stdio = false)]
    fn test_verify_file() {
        let input = "../../tests/input\\x86\\meshes\\actors\\ambient\\chicken\\animations\\idle_fulbody3.hkx";
        // let input = "../../tests/input\\x86\\meshes\\actors\\ambient\\chicken\\animations\\idle_sitdpeck.hkx";
        verify_file(input, true).unwrap_or_else(|err| panic!("{err}"));
    }
}
