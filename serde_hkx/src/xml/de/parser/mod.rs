//! XML parser combinator
pub mod error;
pub mod tag;
pub mod type_kind;

use winnow::ascii::multispace0;
use winnow::combinator::{alt, delimited, fail, repeat};
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::token::take_until;
use winnow::Parser;

/// Parses a string with surrounding whitespace(0 or more times)
///
/// # Note
/// Has expected info.
pub fn delimited_with_multispace0<'a>(
    s: &'static str,
) -> impl Parser<&'a str, &'a str, ContextError> {
    delimited(multispace0, s, multispace0)
        .context(StrContext::Expected(StrContextValue::StringLiteral(s)))
}

/// Analyze in the following order.
/// - `Comments or multi spaces` -> `s` -> `multi spaces`
///
/// # Note
/// Has expected info.
pub fn delimited_comment_multispace0<'a>(
    s: &'static str,
) -> impl Parser<&'a str, &'a str, ContextError> {
    delimited(comment_multispace0(), s, multispace0)
        .context(StrContext::Expected(StrContextValue::StringLiteral(s)))
}

/// Analyze in the following order.
/// - `multi spaces` -> `s` -> `Comments or multi spaces`
///
/// # Note
/// Has expected info.
pub fn delimited_multispace0_comment<'a>(
    s: &'static str,
) -> impl Parser<&'a str, &'a str, ContextError> {
    delimited(multispace0, s, comment_multispace0())
        .context(StrContext::Expected(StrContextValue::StringLiteral(s)))
}

/// Skip comments and whitespace (0 or more times).
pub fn comment_multispace0<'a>() -> impl Parser<&'a str, (), ContextError> {
    repeat(
        0..,
        alt((
            ' ',
            '\t',
            '\r',
            '\n',
            comment().map(|_| ' '),
            fail.context(StrContext::Expected(StrContextValue::CharLiteral(' ')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\t')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\r')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\n')))
                .context(StrContext::Expected(StrContextValue::Description(
                    "Comment: (e.g. `<!-- comment -->`",
                ))),
        )),
    )
}

/// Skip comments and whitespace (1 or more times).
pub fn comment_multispace1<'a>() -> impl Parser<&'a str, (), ContextError> {
    repeat(
        1..,
        alt((
            ' ',
            '\t',
            '\r',
            '\n',
            comment().map(|_| ' '),
            fail.context(StrContext::Expected(StrContextValue::CharLiteral(' ')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\t')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\r')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\n')))
                .context(StrContext::Expected(StrContextValue::Description(
                    "Comment: (e.g. `<!-- comment -->`",
                ))),
        )),
    )
}

/// Parses a XML comment.
/// - e.g. ` memSizeAndFlags SERIALIZE_IGNORED ` in `<!-- memSizeAndFlags SERIALIZE_IGNORED -->`
pub fn comment<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    delimited("<!--", take_until(0.., "-->"), "-->").context(StrContext::Expected(
        StrContextValue::Description("comment"),
    ))
}
