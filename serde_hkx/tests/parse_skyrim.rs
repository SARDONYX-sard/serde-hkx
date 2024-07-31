use havok_classes::Classes;
use pretty_assertions::assert_eq;
use serde_hkx::{
    bytes::serde::hkx_header::HkxHeader, from_bytes, from_str, to_bytes, to_string, HavokSort,
};
use std::path::Path;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, snafu::Snafu)]
enum ConvertError {
    #[snafu(transparent)]
    DeError {
        source: serde_hkx::errors::de::Error,
    },
    #[snafu(transparent)]
    SerError {
        source: serde_hkx::errors::ser::Error,
    },

    #[snafu(transparent)]
    IoError { source: std::io::Error },
}

type Result<T> = core::result::Result<T, ConvertError>;

#[tokio::test]
#[ignore = "Because it is impossible to test without a set of files in the game."]
#[quick_tracing::init(test = "should_parse_one_file", stdio = false)]
async fn should_parse_one_file() -> Result<()> {
    // let path = "./tests/data/meshes/interface/intperkline01.hkx";
    // let path = "./tests/data/meshes/actors/ambient/chicken/chickenproject.hkx";
    // let path = "./tests/data/meshes/actors/character/characters/defaultmale.hkx";
    // parse_to_xml(path).await.unwrap();

    // let xml = include_str!("../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml");
    let xml = include_str!("../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml");
    let bytes = match xml_to_bytes(xml) {
        Ok(bytes) => bytes,
        Err(err) => panic!("{err}"),
    };
    // tokio::fs::write("./output.hkx", xml_to_bytes(xml)?).await?;

    let expected = include_bytes!("../../docs/handson_hex_dump/wisp_skeleton/skeleton.hkx");
    assert_eq!(
        rhexdump::hexdump::RhexdumpString::new().hexdump_bytes(&bytes),
        rhexdump::hexdump::RhexdumpString::new().hexdump_bytes(expected),
    );
    tracing::debug!(
        "bytes = \n{}",
        rhexdump::hexdump::RhexdumpString::new().hexdump_bytes(&bytes),
    );
    Ok(())
}

fn xml_to_bytes(xml: &str) -> Result<Vec<u8>> {
    let mut classes: ClassMap = from_str(xml)?;
    classes.sort_for_bytes()?;
    tracing::debug!("sorted_classes = {classes:#?}");
    Ok(to_bytes(&classes, &HkxHeader::new_skyrim_se())?)
}

#[tokio::test]
#[ignore = "Because it is impossible to test without a set of files in the game."]
// #[quick_tracing::init(test = "from_bytes_skyrim_se_all_files", stdio = false)]
#[quick_tracing::init]
async fn test() -> std::io::Result<()> {
    let mut task_handles: Vec<tokio::task::JoinHandle<Result<()>>> = Vec::new();

    for path in jwalk::WalkDir::new("./tests/data") {
        let path = path?.path();
        let path = path.as_path();
        if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new("hkx")) {
            continue;
        }

        let path = path.to_path_buf();
        task_handles.push(tokio::spawn(async {
            let cloned_path = path.clone();
            match parse_to_xml(path).await {
                Ok(_) => Ok(()),
                Err(err) => {
                    tracing::error!("{cloned_path:?}\n{err}");
                    Err(err)
                }
            }
        }));
    }

    for task_handle in task_handles {
        if let Err(err) = task_handle.await {
            panic!("{err}")
        };
    }
    Ok(())
}

type ClassMap<'a> = indexmap::IndexMap<usize, Classes<'a>>;

async fn parse_to_xml(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let bytes = std::fs::read(path)?;

    // Verify bytes.
    let mut classes = from_bytes::<ClassMap>(&bytes)?;
    assert_eq!(
        rhexdump::hexdump::RhexdumpString::new()
            .hexdump_bytes(to_bytes(&classes, &HkxHeader::new_skyrim_se())?),
        rhexdump::hexdump::RhexdumpString::new().hexdump_bytes(&bytes),
        "path = {path:?}"
    );

    // Create output path
    let mut out_path = path.to_path_buf();
    let _ = out_path.set_extension("xml");
    let out_path = out_path
        .to_string_lossy()
        .replace("./tests/data", "./tests/output/xml");
    let out_path = Path::new(&out_path);

    // Write XML
    if let Some(parent) = out_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    classes.sort_for_xml()?;
    let top_ptr = classes.keys().min().copied().unwrap_or_default();
    let xml = to_string(&classes, top_ptr)?;
    tokio::fs::write(out_path, &xml).await?;

    // Verify ast
    let mut classes_from_xml: ClassMap = from_str(&xml)?;
    classes_from_xml.sort_keys();
    assert_eq!(classes_from_xml, classes, "path = {path:?}");

    Ok(())
}
