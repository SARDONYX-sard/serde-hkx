use crate::{
    cpp_info::{Class, Member, TypeKind},
    error::*,
    ClassMap,
};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use snafu::OptionExt as _;
use std::{borrow::Cow, fs, path::Path};

/// Generate havok class code(For Rust) from class info json files.
///
/// - `classes_json_dir`: `assets/`
pub fn generate_havok_class_table<P: AsRef<Path>>(classes_json_dir: P, out_dir: P) -> Result<()> {
    let class_out_dir = out_dir.as_ref().join("generated");
    std::fs::create_dir_all(&class_out_dir)?;

    let mut class_map = IndexMap::new();

    let classes_json_dir = classes_json_dir.as_ref();
    insert_classes_from_json(classes_json_dir, &mut class_map)?;
    class_map.sort_keys();

    let mut classes = vec![];
    for (_, class) in &class_map {
        classes.push(from_cpp_class(class));
    }

    let rust_code = quote::quote! {
        /// key: className, value: (fieldName, fieldType)
        pub type ClassInfo = (&'static str, &'static [(&'static str, &'static str)]);
        pub type ClassTable = &'static [ClassInfo];

        const CLASS_TABLE: ClassTable = &[#(#classes,)*];

        pub fn find_class_info(class_name: &str) -> Option<&'static ClassInfo> {
            CLASS_TABLE.binary_search_by(|&(name, _)| name.cmp(class_name))
                .ok()
                .map(|index| &CLASS_TABLE[index])
        }
    };

    let output_file = class_out_dir.join("class_table.rs");

    std::fs::write(
        output_file,
        prettyplease::unparse(&syn::parse2(rust_code).unwrap()),
    )?;

    Ok(())
}

fn insert_classes_from_json<P>(classes_json_dir: P, class_map: &mut ClassMap) -> Result<()>
where
    P: AsRef<Path>,
{
    let classes_json_dir = classes_json_dir.as_ref();

    // 1 json file
    for path in jwalk::WalkDir::new(classes_json_dir) {
        let path = path?.path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if !ext.eq_ignore_ascii_case("json") {
                continue;
            }

            if path.ends_with("havok_class_schema.json") {
                continue;
            }
        } else {
            continue;
        }

        let class_name = path.file_stem().context(NotFoundFileStemSnafu {
            path: path.to_string_lossy().into_owned(),
        })?;
        let class_name = class_name.to_string_lossy().into_owned();

        let class: Class = serde_json::from_str(&fs::read_to_string(path)?)?;
        class_map.insert(class_name, class);
    }

    Ok(())
}

pub fn from_cpp_class(class: &Class) -> TokenStream {
    let Class {
        name: class_name,
        members,
        ..
    } = class;

    let fields: Vec<_> = members
        .iter()
        .map(|member| {
            let Member {
                name,
                vtype,
                class_ref,
                ..
            } = member;

            let json_type = to_json_type(class_ref, vtype)
                .unwrap_or_else(|| panic!("Couldn't convert json type: {vtype}"));

            // e.g.
            // - ("field1", "i64"),
            // - ("field2", "object|hello"),
            quote::quote! {
                (#name, #json_type)
            }
        })
        .collect();

    quote::quote! { (#class_name, &[#(#fields,)*]) }
}

/// - Null
/// - I64
/// - U64
/// - F64
/// - String
/// - Bool
/// - Object(val)
/// - Array(val)
fn to_json_type<'a>(name: &Option<Cow<'a, str>>, ty: &TypeKind) -> Option<Cow<'a, str>> {
    Some(match ty {
        TypeKind::Void => "Null".into(),
        TypeKind::Bool => "Bool".into(),
        TypeKind::Char => "String".into(), // JSON typically represents characters as strings
        TypeKind::Int8 | TypeKind::Int16 | TypeKind::Int32 | TypeKind::Int64 => "I64".into(),
        TypeKind::Uint8 | TypeKind::Uint16 | TypeKind::Uint32 | TypeKind::Uint64 => "U64".into(),
        TypeKind::Real => "F64".into(),
        TypeKind::Vector4 => "Object|Vector4".into(),
        TypeKind::Quaternion => "Object|Quaternion".into(),
        TypeKind::Matrix3 => "Object|Matrix3".into(),
        TypeKind::Rotation => "Object|Rotation".into(),
        TypeKind::QsTransform => "Object|QsTransform".into(),
        TypeKind::Matrix4 => "Object|Matrix4".into(),
        TypeKind::Transform => "Object|Transform".into(),
        TypeKind::Pointer => "String".into(), // e.g. #0000
        TypeKind::Enum | TypeKind::Flags => "String".into(), // e.g. `TYPE_ROLE`
        TypeKind::Struct => {
            if let Some(name) = name {
                format!("Object|{name}").into()
            } else {
                panic!("Struct needs a name, but `None` was taken.")
            }
        }
        TypeKind::Array | TypeKind::SimpleArray => "Array".into(),
        TypeKind::Variant => "Object|Variant".into(),
        TypeKind::CString | TypeKind::StringPtr => "String".into(),
        TypeKind::Ulong => "U64".into(),
        TypeKind::Half => "F64".into(),
        _ => return None,
    })
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

        let classes_json_dir = repo_root.join("assets");
        let output_dir = repo_root.join("");
        std::fs::create_dir_all(&output_dir).unwrap();

        if let Err(err) = generate_havok_class_table(classes_json_dir, output_dir) {
            dbg!(err);
        };
    }
}
