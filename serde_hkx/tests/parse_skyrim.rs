use havok_classes::Classes;
use serde_hkx::{from_bytes, to_string};

#[quick_tracing::init(test = "from_bytes_skyrim_se_all_files")]
#[ignore = "Because it is impossible to test without a set of files in the game."]
#[test]
fn test() -> std::io::Result<()> {
    for path in jwalk::WalkDir::new("./tests/data") {
        let path = path?.path();
        let path = path.as_path();
        if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new("hkx")) {
            continue;
        }

        tracing::debug!(?path);
        let bytes = std::fs::read(path)?;

        match from_bytes::<indexmap::IndexMap<usize, Classes>>(&bytes) {
            Ok(mut classes) => {
                // hkRootContainer" is processed last.
                classes.sort_keys();
                let mut top_ptr = None;
                if !classes.is_empty() {
                    if let Some((first_key, first_value)) = classes.shift_remove_index(0) {
                        classes.insert(first_key, first_value);
                        top_ptr = Some(first_key);
                    }
                }

                let file_name = path.file_stem().unwrap().to_string_lossy();

                tracing::debug!("{classes:#?}");
                std::fs::write(
                    &format!("./{file_name}.xml"),
                    &to_string(&classes, top_ptr.unwrap_or_default()).unwrap(),
                )
                .unwrap();
            }
            Err(err) => {
                tracing::error!("{err}");
                panic!("{err}")
            }
        };
    }

    Ok(())
}
