pub mod cpp_info;
pub mod error;

use crate::{cpp_info::Class, error::*};
use std::path::Path;

/// Generate havok class code(For Rust) from class info json files.
pub fn generate_havok_class<P: AsRef<Path>>(classes_json_dir: P, output_dir: P) -> Result<()> {
    use indexmap::IndexMap;
    use snafu::OptionExt;
    use std::fs;

    fs::create_dir_all(&output_dir)?;

    //? Tips: Breaking through the lifetime constraint of Cow<'a, str> by caching the ownership type in the first round loop.
    let mut json_map = IndexMap::new();
    for path in jwalk::WalkDir::new(classes_json_dir) {
        let path = path?.path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }

        let class_name = path.file_stem().context(NotFoundFileStemSnafu {
            path: path.to_string_lossy().into_owned(),
        })?;
        let class_name = class_name.to_string_lossy().into_owned();

        let json_str = fs::read_to_string(path)?;
        json_map.insert(class_name, json_str);
    }

    let mut class_map = IndexMap::new();
    for (class_name, json_str) in &json_map {
        let class: Class = serde_json::from_str(json_str)?;
        dbg!(&class);
        class_map.insert(class_name.as_str(), class);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn should_json_parse() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");

        let classes_json_dir = repo_root.join("assets").join("classes");
        let output_dir = repo_root
            .join("crates")
            .join("serde_hkx")
            .join("src")
            .join("classes")
            .join("generated");

        if let Err(err) = generate_havok_class(classes_json_dir, output_dir) {
            dbg!(err.location_string(), err);
        };
    }
}
