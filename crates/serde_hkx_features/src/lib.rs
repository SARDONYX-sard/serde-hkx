pub mod convert;
pub mod diff;
pub mod dump;
pub mod error;
pub mod fs;
pub mod serde;
pub mod tree;

/// - key: class index(e.g `#0001`)
/// - value: C++ Class
pub type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;

/// Generate json schema.
///
/// # Errors
/// - When parse Json failed.
#[cfg(feature = "json_schema")]
pub fn generate_json_schema() -> error::Result<String> {
    let schema = schemars::schema_for!(ClassMap);
    let schema = simd_json::to_string_pretty(&schema).map_err(|e| error::Error::JsonError {
        input: std::path::PathBuf::new(),
        source: e,
    })?;
    Ok(schema)
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "json_schema")]
    #[cfg_attr(miri, ignore = "`create_dir_all` is error.")]
    #[test]
    fn gen_schema() {
        std::fs::create_dir_all("../../tests").unwrap();
        std::fs::write(
            "../../tests/schema.json",
            super::generate_json_schema().unwrap(),
        )
        .unwrap();
    }
}
