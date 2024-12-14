//! Json schema generator.
use crate::error::Result;
use crate::types_wrapper::ClassPtrMap;
use schemars::schema_for;

/// Generate json schema.
///
/// # Errors
/// - When parse Json failed.
pub fn generate_json_schema() -> Result<String> {
    let schema = schema_for!(ClassPtrMap);
    let schema = simd_json::to_string_pretty(&schema)?;
    Ok(schema)
}

#[cfg(test)]
mod tests {

    // NOTE: For some reason, using miri and feature together makes it unworkable.
    // Comment out miri when executing.
    #[cfg_attr(miri, ignore)] // `create_dir_all` is error.
    #[cfg(feature = "json_schema")]
    #[test]
    fn gen_schema() {
        std::fs::create_dir_all("../../tests").unwrap();
        let json = super::generate_json_schema().unwrap();
        std::fs::write("../../assets/serde_hkx_schema.json", json).unwrap();
    }
}
