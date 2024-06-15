use crate::lib::*;
use winnow::error::{ContextError, ParseError};

/// Error struct to represent parsing errors in a more user-friendly way.
#[derive(Debug)]
pub struct ReadableError {
    message: String,
    span: std::ops::Range<usize>,
    input: String,
}

impl ReadableError {
    /// Constructs [`Self`] from parse error & input.
    pub fn from_parse(error: ParseError<&str, ContextError>, input: &str) -> Self {
        let message = error.inner().to_string();
        let input = input.to_owned();
        let start = error.offset();
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);
        Self {
            message,
            span: start..end,
            input,
        }
    }
}

impl fmt::Display for ReadableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = annotate_snippets::Level::Error
            .title(&self.message)
            .snippet(
                annotate_snippets::Snippet::source(&self.input)
                    .fold(true)
                    .annotation(
                        annotate_snippets::Level::Error
                            .span(self.span.clone())
                            .label(&self.message),
                    ),
            );
        let renderer = annotate_snippets::Renderer::plain();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl std::error::Error for ReadableError {}

#[cfg(test)]
macro_rules! readable_assert_eq {
    ($parser:tt($input:ident), $expected:expr) => {
        match $parser().parse($input).map_err(|e| {
            crate::xml::de::parser::error::ReadableError::from_parse(e, $input).to_string()
        }) {
            Ok(res) => assert_eq!(res, $expected),
            Err(err) => panic!("{err}"),
        }
    };
}
#[cfg(test)]
pub(crate) use readable_assert_eq;
