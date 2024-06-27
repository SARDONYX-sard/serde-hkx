pub mod cpp_info;
pub mod error;
mod gen_index;
mod rust_gen;

use convert_case::Casing;

use crate::{cpp_info::Class, error::*};
use std::path::Path;

/// Generate havok class code(For Rust) from class info json files.
pub fn generate_havok_class<P: AsRef<Path>>(classes_json_dir: P, out_dir: P) -> Result<()> {
    let class_out_dir = out_dir.as_ref().join("generated");

    use indexmap::IndexMap;
    use snafu::OptionExt;
    use std::fs;

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

        let json_str = fs::read_to_string(path)?;
        let class: Class = serde_json::from_str(&json_str)?;

        let struct_define = rust_gen::gen_struct(&class);
        let enum_defines = rust_gen::gen_enums(&class);
        let impl_ser_for_struct = rust_gen::impl_serialize_for_struct(&class);
        let impl_ser_for_enum = rust_gen::impl_serialize_for_enum(&class);

        let ast = match syn::parse2(quote::quote! {
            use super::class_requires::*;
            use super::*;
            #struct_define
            #impl_ser_for_struct

            #(#enum_defines)*
            #impl_ser_for_enum
        }) {
            Ok(ast) => ast,
            Err(err) => panic!("{path:?} span = {:?}, {err}", err.span()),
        };
        let formatted = prettyplease::unparse(&ast);
        std::fs::write(class_out_dir.join(format!("{class_name}.rs")), formatted)?;

        class_index.push({
            let mod_name = class_name.to_case(convert_case::Case::Snake);
            let class_ident = syn::Ident::new(&mod_name, proc_macro2::Span::call_site());

            let class_name = format!("/src/generated/{class_name}.rs");
            quote::quote! {
                pub mod #class_ident {
                    include!(concat!(env!("CARGO_MANIFEST_DIR"), #class_name));
                }
                pub use #class_ident::*;
            }
        });
        class_map.insert(class_name, class);
    }

    std::fs::write(
        out_dir.as_ref().join("generated.rs"),
        gen_index::gen_index(&class_index),
    )?;

    class_map.first();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[ignore = "because it generates code and is not strictly a test."]
    #[test]
    fn should_json_parse() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");

        let classes_json_dir = repo_root.join("assets").join("classes");
        let output_dir = repo_root.join("crates/havok_class/src");
        std::fs::create_dir_all(&output_dir).unwrap();

        if let Err(err) = generate_havok_class(classes_json_dir, output_dir) {
            dbg!(err);
        };
    }
}
