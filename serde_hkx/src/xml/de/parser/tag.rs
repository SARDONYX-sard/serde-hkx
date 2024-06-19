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
}

/// Parses the array start tag `<hkparam name="key" numelements="3">`
///
/// # Returns
/// (name, numelements) -> e.g. (`key`, 3)
pub fn array_start_tag<'a>() -> impl Parser<&'a str, (&'a str, u64), ContextError> {
    seq!(
        _: delimited_comment_multispace0("<"),
        _: delimited_with_multispace0("hkparam"),
        _: delimited_with_multispace0("name"),
        _: delimited_with_multispace0("="),
        attr_string(), // e.g. "key"

        _: delimited_with_multispace0("numelements"),
        _: delimited_with_multispace0("="),
        number_in_string(), // e.g. "8"
        _: delimited_multispace0_comment(">")
    )
    .context(StrContext::Label("Array start tag"))
    .context(StrContext::Expected(StrContextValue::Description(
        "e.g. `<hkparam name=\"key\" numelements=\"3\">`",
    )))
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
        radix_digits().map(|digits| Signature::new(digits as u32)),
        _: delimited_multispace0_comment(">")
    )
    .context(StrContext::Label("Class start tag"))
    .context(StrContext::Expected(StrContextValue::Description(
        r##"Class start(e.g. `<hkobject name="#0010" class="hkbProjectData" signature="0x13a39ba7">`)"##,
    )))
}

/// Parses the field of class start tag `<hkparam name="key">`
///
/// # Returns
/// name -> e.g. `key`
pub fn class_field_start_tag<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    seq!(
        _: delimited_comment_multispace0("<"),
        _: delimited_with_multispace0("hkparam"),
        _: delimited_with_multispace0("name"),
        _: delimited_with_multispace0("="),
        attr_string(), // e.g. "key"
        _: delimited_multispace0_comment(">")
    )
    .map(|s| s.0)
    .context(StrContext::Label("Array start tag"))
    .context(StrContext::Expected(StrContextValue::Description(
        "e.g. `<hkparam name=\"key\" numelements=\"3\">`",
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

/// Parses a xml attribute string, e.g. `"string"`
fn attr_string<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    delimited("\"", take_until(0.., "\""), "\"")
        .context(StrContext::Label("String in XML attribute"))
        .context(StrContext::Expected(Description(r#"String(e.g. `"Str"`)"#)))
}

/// Parser a xml attribute pointer in string, e.g. `"#0050"`
fn attr_ptr<'a>() -> impl Parser<&'a str, Pointer, ContextError> {
    delimited(
        delimited_with_multispace0("\""),
        pointer(),
        delimited_with_multispace0("\""),
    )
}

fn radix_digits<'a>() -> impl Parser<&'a str, usize, ContextError> {
    dispatch!(take(2usize);
        "0b" => digit1.try_map(|s| usize::from_str_radix(s, 2)),
        "0o" => oct_digit1.try_map(|s| usize::from_str_radix(s, 8)),
        "0d" => digit1.try_map(|s| usize::from_str_radix(s, 10)),
        "0x" => hex_digit1.try_map(|s| usize::from_str_radix(s, 16)),
        _ => fail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::de::parser::error::ReadableError;

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
        match end_tag("hkparam").parse(input).map_err(|e| {
            crate::xml::de::parser::error::ReadableError::from_parse(e, input).to_string()
        }) {
            Ok(res) => assert_eq!(res, ()),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_parse_array_start_tag() {
        fn test_parse(input: &str, expected: (&str, u64)) {
            match array_start_tag()
                .parse(input)
                .map_err(|e| ReadableError::from_parse(e, input).to_string())
            {
                Ok(res) => assert_eq!(res, expected),
                Err(err) => panic!("{err}"),
            }
        }

        let ideal_input = r#"<hkparam name="key" numelements="3">"#;
        test_parse(ideal_input, ("key", 3));

        let indent_input = r#"


        <      hkparam name  =  "key!"
          numelements
  = "85"

>"#;
        test_parse(indent_input, ("key!", 85));
    }

    #[test]
    fn test_parse_number_in_string() {
        assert_eq!(number_in_string().parse(r#""33""#), Ok(33));
        assert_eq!(number_in_string().parse(r#""100""#), Ok(100));
        assert_eq!(number_in_string().parse(r#""0""#), Ok(0));
    }
}
