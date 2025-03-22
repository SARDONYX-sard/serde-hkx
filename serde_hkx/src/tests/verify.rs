use crate::{
    bytes::{hexdump, serde::hkx_header::HkxHeader},
    errors::SerdeHkxError,
    from_bytes, from_str,
    tests::{diff, ClassMap},
    to_bytes, to_string, HavokSort as _,
};
use pretty_assertions::assert_eq;
use winnow::Parser;

type Result<T> = core::result::Result<T, SerdeHkxError>;

// #[cfg_attr(miri, ignore)] // Unexplained hang
#[test]
#[cfg_attr(
    all(feature = "tracing", not(miri)),
    quick_tracing::init(test = "should_reproduce_xml_to_amd64", stdio = false)
)]
fn should_reproduce_xml_to_amd64() {
    let xml = {
        // include_str!("../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml")
        include_str!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml")
        // include_str!("../../../tests/test/test.xml")
    };
    let expected_bytes = {
        // include_bytes!("../../../docs/handson_hex_dump/defaultmale/defaultmale.hkx")
        // include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.hkx")
        include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton_x64_reconverted.hkx")
        // include_bytes!("../../../tests/test/test_x64_reconverted.hkx")
    };

    if let Err(err) = assert_bytes(xml, expected_bytes) {
        tracing::error!("{err}");
        panic!("{err}")
    }
}

#[cfg_attr(miri, ignore)] // Unexplained hang
#[test]
#[cfg_attr(
    all(feature = "tracing", not(miri)),
    quick_tracing::init(test = "should_reproduce_xml_to_win32", stdio = false)
)]
fn should_reproduce_xml_to_win32() {
    let xml = {
        // include_str!("../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml")
        include_str!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml")
        // include_str!("../../../tests/test/test.xml")
    };
    let expected_bytes =
        include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton_x86_reconverted.hkx");

    if let Err(err) = assert_bytes(xml, expected_bytes) {
        tracing::error!("{err}");
        panic!("{err}")
    }
}

fn assert_bytes(xml: &str, expected_bytes: &[u8]) -> Result<()> {
    let actual_bytes = {
        let mut actual_classes: ClassMap = from_str(xml)?;
        actual_classes.sort_for_bytes();

        let (_remain, header) = HkxHeader::parser().parse_peek(expected_bytes).unwrap();
        to_bytes(&actual_classes, &header)?
    };

    // Assert hexdump
    {
        let actual_hex_dump = hexdump::to_string(&actual_bytes);
        let expected_hex_dump = hexdump::to_string(expected_bytes);
        let hexdump_diff = diff(&actual_hex_dump, &expected_hex_dump);
        tracing::debug!("hexdump_diff = \n{hexdump_diff}");
        assert_eq!(actual_hex_dump, expected_hex_dump);
    }

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

    Ok(())
}

#[test]
#[ignore = "Because it can't be fully reproduced yet"] // Should ignore miri test, because unexplained hang.
#[quick_tracing::try_init(test = "should_reproduce_xml", stdio = false)]
fn should_reproduce_xml() -> Result<()> {
    let bytes = {
        // include_bytes!("../../../docs/handson_hex_dump/defaultmale/defaultmale.hkx")
        // include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.hkx")
        include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton_x64_reconverted.hkx")
    };

    let expected = {
        // include_str!("../../../docs/handson_hex_dump/defaultmale/defaultmale_x86.xml")
        include_str!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml")
    };

    let bytes_to_xml = || {
        let mut actual_classes: ClassMap = from_bytes(bytes)?;
        let top_ptr = actual_classes.sort_for_xml()?;
        Result::Ok(to_string(&actual_classes, &top_ptr)?)
    };

    let actual = match bytes_to_xml() {
        Ok(xml) => xml,
        Err(err) => {
            tracing::error!("{err}");
            panic!("{err}")
        }
    };
    let xml_diff = diff(actual, expected);
    tracing::debug!("map_diff = \n{xml_diff}");

    Ok(())
}
