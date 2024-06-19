pub mod error;
pub mod tag;
pub mod type_kind;

use winnow::combinator::{alt, delimited, repeat};
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::token::take_until;
use winnow::Parser;

/// Parses a string with surrounding whitespace & comments.
/// - (0 or more times)
///
/// # Note
/// Has expected info.
pub fn delimited_with_multispace0<'a>(
    s: &'static str,
) -> impl Parser<&'a str, &'a str, ContextError> {
    delimited(xml_multispace0(), s, xml_multispace0())
        .context(StrContext::Expected(StrContextValue::StringLiteral(s)))
}

/// Skip comments and whitespace (0 or more times).
pub fn xml_multispace0<'a>() -> impl Parser<&'a str, (), ContextError> {
    repeat(0.., alt((' ', '\t', '\r', '\n', comment().map(|_| ' '))))
}

/// Parses a XML comment.
fn comment<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    delimited("<!--", take_until(0.., "-->"), "-->").context(StrContext::Expected(
        StrContextValue::Description("comment"),
    ))
}

// <!-- memSizeAndFlags SERIALIZE_IGNORED -->
