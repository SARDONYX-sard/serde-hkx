//! XML parser combinator
pub mod tag;
pub mod type_kind;

use winnow::ascii::multispace0;
use winnow::combinator::{alt, delimited, fail, repeat};
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::token::take_until;
use winnow::{PResult, Parser};

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
    delimited(comment_multispace0, s, multispace0)
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
    delimited(multispace0, s, comment_multispace0)
        .context(StrContext::Expected(StrContextValue::StringLiteral(s)))
}

/// Skip comments and whitespace (0 or more times).
///
/// # Errors
/// When parse failed.
pub fn comment_multispace0(input: &mut &str) -> PResult<()> {
    repeat(
        0..,
        alt((
            ' ',
            '\t',
            '\r',
            '\n',
            comment.map(|_| ' '),
            fail.context(StrContext::Expected(StrContextValue::CharLiteral(' ')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\t')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\r')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\n')))
                .context(StrContext::Expected(StrContextValue::Description(
                    "Comment: (e.g. `<!-- comment -->`",
                ))),
        )),
    )
    .parse_next(input)
}

/// Skip comments and whitespace (1 or more times).
///
/// # Errors
/// When parse failed.
pub fn comment_multispace1(input: &mut &str) -> PResult<()> {
    repeat(
        1..,
        alt((
            ' ',
            '\t',
            '\r',
            '\n',
            comment.map(|_| ' '),
            fail.context(StrContext::Expected(StrContextValue::CharLiteral(' ')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\t')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\r')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('\n')))
                .context(StrContext::Expected(StrContextValue::Description(
                    "Comment: (e.g. `<!-- comment -->`",
                ))),
        )),
    )
    .parse_next(input)
}

/// Parses a XML comment.
/// - e.g. ` memSizeAndFlags SERIALIZE_IGNORED ` in `<!-- memSizeAndFlags SERIALIZE_IGNORED -->`
///
/// # Errors
/// When parse failed.
pub fn comment<'a>(input: &mut &'a str) -> PResult<&'a str> {
    delimited("<!--", take_until(0.., "-->"), "-->")
        .context(StrContext::Expected(StrContextValue::Description(
            "comment",
        )))
        .parse_next(input)
}
