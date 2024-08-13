use crate::{
    bytes::{hexdump, serde::hkx_header::HkxHeader},
    errors::SerdeHkxError,
    from_bytes, from_str,
    tests::ClassMap,
    to_bytes, to_string, HavokSort,
};
use pretty_assertions::assert_eq;

type Result<T> = core::result::Result<T, SerdeHkxError>;

#[tokio::test]
#[ignore = "Because it can't be fully reproduced yet"]
#[quick_tracing::try_init(test = "should_reproduce_bytes", stdio = false)]
async fn should_reproduce_bytes() -> Result<()> {
    let xml = {
        // include_str!("../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml")
        include_str!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml")
    };
    let expected_bytes = {
        // include_bytes!("../../docs/handson_hex_dump/defaultmale/defaultmale.hkx")
        // include_bytes!("../../docs/handson_hex_dump/wisp_skeleton/skeleton.hkx")
        // include_bytes!("../../docs/handson_hex_dump/wisp_skeleton/skeleton_x86_reconverted.hkx")
        include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton_x64_reconverted.hkx")
    };

    let actual_bytes = {
        let xml_to_bytes = || -> Result<Vec<u8>> {
            let mut actual_classes: ClassMap = from_str(xml)?;
            actual_classes.sort_for_bytes();
            Ok(to_bytes(&actual_classes, &HkxHeader::new_skyrim_se())?)
        };

        xml_to_bytes().map_err(|err| {
            tracing::error!("{err}");
            err
        })?
    };

    // Ast diff
    {
        let actual_classes: ClassMap = from_bytes(&actual_bytes)?;
        let expected_classes: ClassMap = from_bytes(expected_bytes)?;
        let ast_diff = diff(
            format!("{expected_classes:#?}"),
            format!("{actual_classes:#?}"),
        );
        tracing::debug!("ast_diff = \n{ast_diff}");
    }

    // Assert hexdump
    {
        let actual_hex_dump = hexdump::to_string(&actual_bytes);
        let expected_hex_dump = hexdump::to_string(expected_bytes);
        let hexdump_diff = diff(&expected_hex_dump, &actual_hex_dump);
        tracing::debug!("hexdump_diff = \n{hexdump_diff}");
        assert_eq!(actual_hex_dump, expected_hex_dump);
    }

    Ok(())
}

#[tokio::test]
#[ignore = "Because it can't be fully reproduced yet"]
#[quick_tracing::try_init(test = "should_reproduce_xml", stdio = false)]
async fn should_reproduce_xml() -> Result<()> {
    let bytes = {
        // include_bytes!("../../../docs/handson_hex_dump/defaultmale/defaultmale.hkx")
        // include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.hkx")
        include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton_x64_reconverted.hkx")
    };

    let expected = {
        // include_str!("../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml")
        include_str!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml")
    };

    let bytes_to_xml = || {
        let mut actual_classes: ClassMap = from_bytes(bytes)?;
        let top_ptr = actual_classes.sort_for_xml()?;
        Result::Ok(to_string(&actual_classes, top_ptr)?)
    };

    let actual = match bytes_to_xml() {
        Ok(xml) => xml,
        Err(err) => {
            tracing::error!("{err}");
            panic!("{err}")
        }
    };
    let xml_diff = diff(expected, actual);
    tracing::debug!("map_diff = \n{xml_diff}");

    Ok(())
}

fn diff(old: impl AsRef<str>, new: impl AsRef<str>) -> String {
    let diff = similar::TextDiff::from_lines(old.as_ref(), new.as_ref());
    let mut output_diff = String::new();
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            similar::ChangeTag::Delete => "-",
            similar::ChangeTag::Insert => "+",
            similar::ChangeTag::Equal => " ",
        };
        output_diff += &format!("{sign}{change}");
    }
    output_diff
}
