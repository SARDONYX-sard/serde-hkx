//! XML tag parsers
use crate::lib::*;

use super::{
    delimited_comment_multispace0, delimited_multispace0_comment, delimited_with_multispace0,
    type_kind::pointer,
};
use havok_types::{Pointer, Signature};
use winnow::ascii::{digit1, hex_digit1, oct_digit1};
use winnow::combinator::{delimited, dispatch, fail, seq};
use winnow::error::{ContextError, StrContext, StrContextValue, StrContextValue::*};
use winnow::token::{take, take_until};
use winnow::{ModalResult, Parser};

/// Parses the start tag `<tag>`
pub fn start_tag<'a>(tag: &'static str) -> impl Parser<&'a str, (), ContextError> {
    seq!(
        _: delimited_comment_multispace0("<"),
        _: delimited_with_multispace0(tag),
        _: delimited_multispace0_comment(">")
    )
    .context(StrContext::Label("start tag"))
    .context(StrContext::Label(tag))
}

/// Parses the end tag `</tag>`
pub fn end_tag<'a>(tag: &'static str) -> impl Parser<&'a str, (), ContextError> {
    seq!(
        _: delimited_comment_multispace0("<"),
        _: delimited_with_multispace0("/"),
        _: delimited_with_multispace0(tag),
        _: delimited_multispace0_comment(">")
    )
    .context(StrContext::Label("end tag"))
    .context(StrContext::Label(tag))
}

/// Parses the array start tag (e.g. `<hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7">`)
///
/// # Returns
/// ([`Pointer`], ClassName, [`Signature`]) -> e.g. (`#0010`, `"hkbProjectData"`, `0x13a39ba7`)
///
/// # Errors
/// When parse failed.
pub fn class_start_tag<'a>(input: &mut &'a str) -> ModalResult<(Pointer, &'a str, Signature)> {
    seq!(
        _: delimited_comment_multispace0("<"),
        _: delimited_with_multispace0("hkobject"),
        _: delimited_with_multispace0("name"),
        _: delimited_with_multispace0("="),
        attr_ptr,

        _: delimited_with_multispace0("class"),
        _: delimited_with_multispace0("="),
        attr_string,

        _: delimited_with_multispace0("signature"),
        _: delimited_with_multispace0("="),
        _: delimited_with_multispace0("\""),
        radix_digits.map(|digits| Signature::new(digits as u32)),
        _: delimited_with_multispace0("\""),
        _: delimited_multispace0_comment(">")
    )
    .context(StrContext::Label("Class start tag"))
    .context(StrContext::Expected(StrContextValue::Description(
        r##"e.g. `<hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7">`"##,
    )))
    .parse_next(input)
}

/// Parses the field of class start opening tag `<hkparam name=`
///
/// # Note
/// All arguments are used only for clarity of error reporting.
pub fn field_start_open_tag<'a>(
    class_name: &'static str,
) -> impl Parser<&'a str, (), ContextError> {
    seq!(
        _: delimited_comment_multispace0("<"),
        _: delimited_with_multispace0("hkparam"),
        _: delimited_with_multispace0("name"),
        _: delimited_with_multispace0("="),
    )
    .context(StrContext::Label("field of class: start opening tag"))
    .context(StrContext::Label(class_name))
    .context(StrContext::Expected(StrContextValue::Description(
        "e.g. `<hkparam name=`",
    )))
}

/// Parses the field of class start closing tag `>`, `numelements="0">`, `/>`, or `numelements="0" />`
///
/// # Returns
/// (numelements, is_self_closing)
///
/// # Errors
/// When parse failed.
pub fn field_start_close_tag(input: &mut &str) -> ModalResult<(Option<u64>, bool)> {
    use winnow::combinator::alt;

    alt((
        // Handle self-closing tag with numelements: numelements="0" />
        seq!(
            seq!(
                _: delimited_with_multispace0("numelements"),
                _: delimited_with_multispace0("="),
                number_in_string::<u64>, // e.g. "8"
            ),
            _: delimited_with_multispace0("/"),
            _: delimited_multispace0_comment(">")
        )
        .map(|((n,),)| (Some(n), true)),
        // Handle regular closing tag with numelements: numelements="0">
        seq!(
            seq!(
                _: delimited_with_multispace0("numelements"),
                _: delimited_with_multispace0("="),
                number_in_string::<u64>, // e.g. "8"
            ),
            _: delimited_multispace0_comment(">")
        )
        .map(|((n,),)| (Some(n), false)),
        // Handle self-closing tag without numelements: />
        seq!(
            _: delimited_with_multispace0("/"),
            _: delimited_multispace0_comment(">")
        )
        .map(|()| (None, true)),
        // Handle regular closing tag without numelements: >
        seq!(
            _: delimited_multispace0_comment(">")
        )
        .map(|()| (None, false)),
    ))
    .context(StrContext::Label("field of class: start closing tag"))
    .context(StrContext::Expected(StrContextValue::Description(
        "e.g. `>`, `/>`, `numelements=\"0\">`, or `numelements=\"0\" />`",
    )))
    .parse_next(input)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// There are support functions that exists only to parse the attributes in the tag.

/// Parses a number inside a string, e.g., `"64"`
///
/// # Errors
/// When parse failed.
pub fn number_in_string<Num>(input: &mut &str) -> ModalResult<Num>
where
    Num: FromStr,
{
    attr_string
        .parse_to()
        .context(StrContext::Label("number in string"))
        .context(StrContext::Expected(Description(r#"Number(e.g. `"64"`)"#)))
        .parse_next(input)
}

/// Parses a xml attribute string(surrounded double quotes), e.g. `"string"`
///
/// # Errors
/// When parse failed.
pub fn attr_string<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    delimited("\"", take_until(0.., "\""), "\"")
        .context(StrContext::Label("String in XML attribute"))
        .context(StrContext::Expected(Description(r#"String(e.g. `"Str"`)"#)))
        .parse_next(input)
}

/// Parser a xml attribute pointer in string, e.g. `"#0050"`
///
/// # Errors
/// When parse failed.
fn attr_ptr(input: &mut &str) -> ModalResult<Pointer> {
    delimited("\"", pointer, "\"").parse_next(input)
}

/// Parse radix digits. e.g. `0b101`, `0xff`
///
/// # Errors
/// When parse failed.
fn radix_digits(input: &mut &str) -> ModalResult<usize> {
    dispatch!(take(2_usize);
        "0b" | "0B" => digit1.try_map(|s| usize::from_str_radix(s, 2))
                        .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("binary"))),
        "0o" | "0O" => oct_digit1.try_map(|s| usize::from_str_radix(s, 8))
                        .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("octal"))),
        "0d" | "0D" => digit1.try_map(|s: &str| s.parse::<usize>())
                        .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("decimal"))),
        "0x" | "0X" => hex_digit1.try_map(|s|usize::from_str_radix(s, 16))
                        .context(StrContext::Label("digit")).context(StrContext::Expected(StrContextValue::Description("hexadecimal"))),
        _ => fail.context(StrContext::Label("radix prefix"))
                .context(StrContext::Expected(StrContextValue::StringLiteral("0b")))
                .context(StrContext::Expected(StrContextValue::StringLiteral("0o")))
                .context(StrContext::Expected(StrContextValue::StringLiteral("0d")))
                .context(StrContext::Expected(StrContextValue::StringLiteral("0x"))),
    ).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::readable::ReadableError;

    #[test]
    fn test_radix_digits() {
        assert_eq!(radix_digits.parse("0b001010"), Ok(10));
        assert_eq!(radix_digits.parse("0o57"), Ok(47));
        assert_eq!(radix_digits.parse("0x1234"), Ok(4660));
    }

    #[test]
    fn test_parse_start_tag() {
        assert!(start_tag("tag").parse("<tag>").is_ok());
        assert!(start_tag("hkparam").parse("< hkparam \n\n>").is_ok());
        assert!(start_tag("tag").parse("<tag   >").is_ok());
    }

    #[test]
    fn test_parse_end_tag() {
        assert!(end_tag("tag").parse("</tag>").is_ok());
        assert!(end_tag("tag").parse("</ tag >").is_ok());
        assert!(end_tag("tag").parse("</  tag  >").is_ok());

        let input = "</ hkparam >\n";
        if let Err(err) = end_tag("hkparam")
            .parse(input)
            .map_err(|e| ReadableError::from_parse(e, input).to_string())
        {
            panic!("{err}");
        };
    }

    #[test]
    fn test_parse_array_start_close_tag() {
        fn test_parse(input: &str, expected: (Option<u64>, bool)) {
            match field_start_close_tag
                .parse(input)
                .map_err(|e| ReadableError::from_parse(e, input).to_string())
            {
                Ok(res) => assert_eq!(res, expected),
                Err(err) => panic!("{err}"),
            }
        }

        // Test regular closing tags
        let ideal_input = r#" numelements="3">"#;
        test_parse(ideal_input, (Some(3), false));

        let indent_input = r#"

          numelements
  = "85"

>"#;
        test_parse(indent_input, (Some(85), false));

        let simple_closing_input = r#" >"#;
        test_parse(simple_closing_input, (None, false));

        // Test self-closing tags
        let self_closing_input = r#" numelements="5" />"#;
        test_parse(self_closing_input, (Some(5), true));

        let simple_self_closing_input = r#" />"#;
        test_parse(simple_self_closing_input, (None, true));

        // Test self-closing tags with spaces
        let spaced_self_closing_input = r#" numelements="0"  /  >"#;
        test_parse(spaced_self_closing_input, (Some(0), true));

        let spaced_simple_self_closing_input = r#"  /  >"#;
        test_parse(spaced_simple_self_closing_input, (None, true));
    }

    #[test]
    fn test_parse_number_in_string() {
        assert_eq!(number_in_string.parse(r#""33""#), Ok(33));
        assert_eq!(number_in_string.parse(r#""100""#), Ok(100));
        assert_eq!(number_in_string.parse(r#""0""#), Ok(0));
    }
}
