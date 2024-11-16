//! Json schema generator.
use crate::error::{Error, Result};
use crate::types_wrapper::ClassPtrMap;

/// Generate json schema.
///
/// # Errors
/// - When parse Json failed.
pub fn generate_json_schema() -> Result<String> {
    let schema_settings = schemars::generate::SchemaSettings::draft2020_12();
    let mut generator = schemars::SchemaGenerator::new(schema_settings);
    let def = generator.definitions_mut();
    def.insert("$schema".into(), Some("").into());
    let schema = generator.into_root_schema_for::<ClassPtrMap>();
    let schema = simd_json::to_string_pretty(&schema).map_err(|e| Error::JsonError {
        input: std::path::PathBuf::new(),
        source: e,
    })?;
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
        std::fs::write("../../tests/schema.json", json).unwrap();
    }
}
