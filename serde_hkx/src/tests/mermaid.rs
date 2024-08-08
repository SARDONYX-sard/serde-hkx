use crate::{errors::SerdeHkxError, from_str, tests::ClassMap};
use havok_serde::HavokClass as _;

/// Output markdown mermaids from Behavior's state machine.
pub fn gen_mermaid_from(classes: ClassMap) -> String {
    let mut mermaid_markdown = String::new();

    for (index, class) in &classes {
        for dep in class.deps_indexes().iter().filter(|&&dep| dep != 0) {
            let current_class_name = class.name();
            let deps_name = classes[dep].name();
            mermaid_markdown +=
                &format!("    {index}_{current_class_name} --> {dep}_{deps_name}\n");
        }
    }

    format!(
        r#"# State machine behavior

```mermaid
flowchart LR
{mermaid_markdown}
```
"#
    )
}

#[test]
fn should_gen_inheritance_tree() -> Result<(), SerdeHkxError> {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let out_path = repo_root.join("logs").join("deps.md");

    let classes = from_str(include_str!(
        "../../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml"
    ))?;

    let markdown = gen_mermaid_from(classes);
    Ok(std::fs::write(out_path, markdown)?)
}
