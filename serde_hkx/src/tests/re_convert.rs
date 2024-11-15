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
    quick_tracing::init(test = "should_match_re_convert_hkx", stdio = false)
)]
fn should_match_re_convert_hkx() {
    let expected_bytes = {
        // include_bytes!("../../../tests/test/DMCO.hkx")
        // include_bytes!("../../../tests/test/DMCO_TransitionPowerAttackToAMCO.hkx")
        // include_bytes!("../../../tests/test/DMCO_TransitionAttackToAMCO.hkx")
        // include_bytes!("../../../tests/test/re_DMCO.hkx")
        include_bytes!("../../../docs/handson_hex_dump/wisp_skeleton/skeleton_x64_reconverted.hkx")
    };

    if let Err(err) = reproduce_bytes(expected_bytes) {
        tracing::error!("{err}");
        panic!("{err}")
    }
}

fn reproduce_bytes(expected_bytes: &[u8]) -> Result<()> {
    // bytes => XML
    let xml = {
        let mut actual_classes: ClassMap = from_bytes(expected_bytes)?;
        let top_ptr = actual_classes.sort_for_xml()?;
        to_string(&actual_classes, top_ptr)?
    };

    // XML => bytes
    let actual_bytes = {
        let mut actual_classes: ClassMap = from_str(&xml)?;
        actual_classes.sort_for_bytes();

        let (_remain, header) = HkxHeader::from_bytes().parse_peek(expected_bytes).unwrap();
        to_bytes(&actual_classes, &header)?
    };

    // Assert actual & expected bytes with hexdump.
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
