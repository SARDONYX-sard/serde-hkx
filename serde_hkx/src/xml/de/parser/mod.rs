pub mod error;
pub mod tag;
pub mod type_kind;

use winnow::ascii::multispace0;
use winnow::combinator::delimited;
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::Parser;

/// Parses a string with surrounding whitespace(0 or more times)
///
/// # Arguments
///
/// * `s` - The string to parse.
///
/// # Note
/// Has expected info.
pub fn delimited_with_multispace0<'a>(
    s: &'static str,
) -> impl Parser<&'a str, &'a str, ContextError> {
    delimited(multispace0, s, multispace0)
        .context(StrContext::Expected(StrContextValue::StringLiteral(s)))
}
