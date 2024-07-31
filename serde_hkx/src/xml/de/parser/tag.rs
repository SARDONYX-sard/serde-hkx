//! XML tag parsers
use crate::lib::*;

use super::type_kind::pointer;
use super::{
    delimited_comment_multispace0, delimited_multispace0_comment, delimited_with_multispace0,
};
use havok_types::{Pointer, Signature};
use winnow::ascii::{digit1, hex_digit1, oct_digit1};
use winnow::combinator::{delimited, dispatch, fail, seq};
use winnow::error::{ContextError, StrContext, StrContextValue, StrContextValue::*};
use winnow::token::{take, take_until};
use winnow::Parser;

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
pub fn class_start_tag<'a>() -> impl Parser<&'a str, (Pointer, &'a str, Signature), ContextError> {
    seq!(
        _: delimited_comment_multispace0("<"),
        _: delimited_with_multispace0("hkobject"),
        _: delimited_with_multispace0("name"),
        _: delimited_with_multispace0("="),
        attr_ptr(),

        _: delimited_with_multispace0("class"),
        _: delimited_with_multispace0("="),
        attr_string(),

        _: delimited_with_multispace0("signature"),
        _: delimited_with_multispace0("="),
        _: delimited_with_multispace0("\""),
        radix_digits().map(|digits| Signature::new(digits as u32)),
        _: delimited_with_multispace0("\""),
        _: delimited_multispace0_comment(">")
    )
    .context(StrContext::Label("Class start tag"))
    .context(StrContext::Expected(StrContextValue::Description(
        r##"e.g. `<hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7">`"##,
    )))
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

/// Parses the field of class start closing tag `>`
pub fn field_start_close_tag<'a>() -> impl Parser<&'a str, Option<u64>, ContextError> {
    seq!(
        winnow::combinator::opt(
            seq!(
                _: delimited_with_multispace0("numelements"),
                _: delimited_with_multispace0("="),
                number_in_string::<u64>(), // e.g. "8"
            )
        ),
        _: delimited_multispace0_comment(">")
    )
    .map(|(n,)| n.map(|n| n.0))
    .context(StrContext::Label("field of class: start closing tag"))
    .context(StrContext::Expected(StrContextValue::Description(
        "e.g. `>`",
    )))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// There are support functions that exists only to parse the attributes in the tag.

/// Parses a number inside a string, e.g., `"64"`
fn number_in_string<'a, Num>() -> impl Parser<&'a str, Num, ContextError>
where
    Num: AddAssign<Num> + MulAssign<Num> + From<u8> + FromStr,
{
    attr_string()
        .parse_to()
        .context(StrContext::Label("number in string"))
        .context(StrContext::Expected(Description(r#"Number(e.g. `"64"`)"#)))
}

/// Parses a xml attribute string(surrounded double quotes), e.g. `"string"`
pub fn attr_string<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    delimited("\"", take_until(0.., "\""), "\"")
        .context(StrContext::Label("String in XML attribute"))
        .context(StrContext::Expected(Description(r#"String(e.g. `"Str"`)"#)))
}

/// Parser a xml attribute pointer in string, e.g. `"#0050"`
fn attr_ptr<'a>() -> impl Parser<&'a str, Pointer, ContextError> {
    delimited("\"", pointer(), "\"")
}

/// Parse radix digits. e.g. `0b101`, `0xff`
fn radix_digits<'a>() -> impl Parser<&'a str, usize, ContextError> {
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
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::readable::ReadableError;

    #[test]
    fn test_radix_digits() {
        assert_eq!(radix_digits().parse("0b001010"), Ok(10));
        assert_eq!(radix_digits().parse("0o57"), Ok(47));
        assert_eq!(radix_digits().parse("0x1234"), Ok(4660));
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
        fn test_parse(input: &str, expected: Option<u64>) {
            match field_start_close_tag()
                .parse(input)
                .map_err(|e| ReadableError::from_parse(e, input).to_string())
            {
                Ok(res) => assert_eq!(res, expected),
                Err(err) => panic!("{err}"),
            }
        }

        let ideal_input = r#" numelements="3">"#;
        test_parse(ideal_input, Some(3));

        let indent_input = r#"

          numelements
  = "85"

>"#;
        test_parse(indent_input, Some(85));
    }

    #[test]
    fn test_parse_number_in_string() {
        assert_eq!(number_in_string().parse(r#""33""#), Ok(33));
        assert_eq!(number_in_string().parse(r#""100""#), Ok(100));
        assert_eq!(number_in_string().parse(r#""0""#), Ok(0));
    }
}
