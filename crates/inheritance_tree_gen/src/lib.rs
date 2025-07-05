use class_generator::cpp_info::Class;
use std::path::Path;

pub fn generate_inheritance_tree(classes_json_dir: impl AsRef<Path>) -> std::io::Result<String> {
    let mut mermaid_markdown = String::new();
    for path in jwalk::WalkDir::new(classes_json_dir) {
        let path = path?.path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }

        let class: Class = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        let class_name = class.name;
        mermaid_markdown += &match class.parent {
            Some(parent) => {
                format!("    {class_name} --> {parent}\n")
            }
            None => format!("    {class_name}\n"),
        };
    }

    Ok(format!(
        r#"# Inheritance tree for havok classes

This file was automatically created from json by `crates/inheritance_tree_gen`.

```mermaid
flowchart LR
{mermaid_markdown}
```
"#
    ))
}

#[cfg_attr(miri, ignore)] //  unsupported operation: `statx` system call
#[test]
fn should_gen_inheritance_tree() -> std::io::Result<()> {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    let classes_json_dir = repo_root.join("assets").join("classes");
    let out_path = repo_root
        .join("docs")
        .join("specification")
        .join("inheritance_tree")
        .join("hk_2010_2_0_r1.md");

    std::fs::write(out_path, generate_inheritance_tree(classes_json_dir)?)
}
