pub mod cpp_info;
pub mod error;
mod gen_index;
mod rust_gen;

use crate::{cpp_info::Class, error::*};
use indexmap::IndexMap;
use snafu::OptionExt as _;
use std::{fs, path::Path};

/// Generate havok class code(For Rust) from class info json files.
pub fn generate_havok_class<P: AsRef<Path>>(classes_json_dir: P, out_dir: P) -> Result<()> {
    let class_out_dir = out_dir.as_ref().join("generated");
    std::fs::create_dir_all(&class_out_dir)?;

    //? Tips: Breaking through the lifetime constraint of Cow<'a, str> by caching the ownership type in the first round loop.
    let mut class_map = IndexMap::new();
    let mut class_index = Vec::new();

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

        let class: Class = serde_json::from_str(&fs::read_to_string(path)?)?;
        class_map.insert(class_name, class);
    }

    for (class_name, class) in &class_map {
        let class_name = format!("{class_name}_");
        let output_file = class_out_dir.join(format!("{class_name}.rs"));
        let output_file = output_file.to_string_lossy();
        tracing::debug!(?output_file);

        let rust_code = prettyplease::unparse(&rust_gen::from_cpp_class(&class, &class_map));
        std::fs::write(output_file.as_ref(), rust_code)?;

        class_index.push({
            let mod_name = quote::format_ident!("{class_name}");
            quote::quote! {
                pub mod #mod_name;
                pub use #mod_name::*;
            }
        });
    }

    std::fs::write(
        out_dir.as_ref().join("generated.rs"),
        gen_index::gen_index(&class_index),
    )?;

    Ok(())
}

pub type ClassMap<'a> = IndexMap<String, Class<'a>>;

/// Enumerate C++ class information by recursively tracing from the current class.
/// - current class name -> Myself and all parent classes.
/// - parent class name -> All parent classes.
///
/// # Returns
/// Vec sorted by deepest parent class.
pub fn get_inherited_class<'a>(class_name: &str, classes_map: &'a ClassMap) -> Vec<&'a Class<'a>> {
    // Cache variables
    let mut current_class_name = class_name;
    let mut inherited_class = Vec::new();

    // Get all parents
    while let Some(class) = classes_map.get(current_class_name) {
        inherited_class.push(class);

        if let Some(parent_name) = &class.parent {
            current_class_name = parent_name;
        } else {
            break; // No more parent to process
        }
    }

    inherited_class.reverse(); // This is because binary reads must be read from the most root parent class.
    inherited_class
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[ignore = "because it generates code and is not strictly a test."]
    // #[quick_tracing::init(test = "should_generate_classes")]
    #[test]
    fn should_generate_classes() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");

        let classes_json_dir = repo_root.join("assets").join("classes");
        let output_dir = repo_root.join("crates/havok_classes/src");
        std::fs::create_dir_all(&output_dir).unwrap();

        if let Err(err) = generate_havok_class(classes_json_dir, output_dir) {
            dbg!(err);
        };
    }
}
