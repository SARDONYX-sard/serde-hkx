use crate::lib::*;

use winnow::combinator::delimited;
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::token::take_until;
use winnow::Parser;

use super::delimited_with_multispace0;

/// Parses the start tag `<tag>`
pub fn start_tag<'a>(tag: &'static str) -> impl Parser<&'a str, (), ContextError> {
    move |input: &mut &'a str| {
        delimited(
            delimited_with_multispace0("<"),
            delimited_with_multispace0(tag),
            delimited_with_multispace0(">"),
        )
        .parse_next(input)?;
        Ok(())
    }
}

/// Parses the end tag `</tag>`
pub fn end_tag<'a>(tag: &'static str) -> impl Parser<&'a str, (), ContextError> {
    move |input: &mut &'a str| {
        let _ = '<'.parse_next(input)?;

        delimited(
            delimited_with_multispace0("/"),
            delimited_with_multispace0(tag),
            delimited_with_multispace0(">"),
        )
        .parse_next(input)?;
        Ok(())
    }
}

/// Parses the array start tag `<hkparam name="key" numelements="3">`
///
/// # Returns
/// (name, numelements) -> e.g. (`key`, 3)
pub fn array_start_tag<'a>() -> impl Parser<&'a str, (&'a str, u64), ContextError> {
    move |input: &mut &'a str| {
        delimited_with_multispace0("<")
            .context(StrContext::Expected(StrContextValue::CharLiteral('<')))
            .parse_next(input)?;

        delimited_with_multispace0("hkparam").parse_next(input)?;
        delimited_with_multispace0("name").parse_next(input)?;
        delimited_with_multispace0("=").parse_next(input)?;
        let name = attr_string().parse_next(input)?;
        delimited_with_multispace0("numelements").parse_next(input)?;
        delimited_with_multispace0("=").parse_next(input)?;
        let num = number_in_string().parse_next(input)?;
        delimited_with_multispace0(">").parse_next(input)?;
        Ok((name, num))
    }
}

// There are support functions that exists only to parse the attributes in the tag.

/// Parses a number inside a string, e.g., `"64"`
fn number_in_string<'a, Num>() -> impl Parser<&'a str, Num, ContextError>
where
    Num: AddAssign<Num> + MulAssign<Num> + From<u8> + FromStr,
{
    |input: &mut &'a str| {
        let num = delimited("\"", take_until(0.., "\""), "\"")
            .parse_to()
            .context(StrContext::Label("number in string"))
            .context(StrContext::Expected(StrContextValue::Description(
                "e.g. \"64\"",
            )))
            .parse_next(input)?;
        Ok(num)
    }
}

/// Parses a xml attribute string, e.g., `"string"`
fn attr_string<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    delimited("\"", take_until(0.., "\""), "\"")
        .context(StrContext::Label("attribute string in XML"))
        .context(StrContext::Expected(StrContextValue::Description(
            "e.g. \"String\"",
        )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::de::parser::error::ReadableError;

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

        let input = "</  hkparam >\n";
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
        assert_eq!(number_in_string().parse("\"33\""), Ok(33));
        assert_eq!(number_in_string().parse("\"100\""), Ok(100));
        assert_eq!(number_in_string().parse("\"0\""), Ok(0));
    }
}
