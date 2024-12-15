//! HexDump Display(For binary)/XML human-readable error message
use crate::lib::*;
use winnow::error::{ContextError, ErrMode, ParseError, StrContext};

/// Error struct to represent parsing errors in a more user-friendly way.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ReadableError {
    title: String,
    message: String,
    span: Range<usize>,
    input: String,
}

impl ReadableError {
    /// Constructs [`Self`] from parse error & input.
    pub fn from_parse(error: ParseError<&str, ContextError>, input: &str) -> Self {
        let message = error.inner().to_string();
        let input = input.to_string();
        let start = error.offset();
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);
        Self {
            title: "Parse error".to_string(),
            message,
            span: start..end,
            input,
        }
    }

    /// Constructs [`Self`] from parse error & input.
    pub fn from_context<T>(error: ErrMode<ContextError>, input: T, err_pos: usize) -> Self
    where
        T: core::fmt::Display,
    {
        let (labels, message) = error
            .map(|ctx_err| {
                ctx_err.cause().map_or_else(
                    || {
                        let mut labels = String::new();
                        let mut msg = "expected ".to_string();

                        for ctx in ctx_err.context() {
                            match ctx {
                                StrContext::Label(label) => {
                                    labels += " <- ";
                                    labels += label;
                                }
                                StrContext::Expected(expected) => {
                                    msg += &expected.to_string();
                                }
                                _ => (),
                            }
                        }
                        (labels, msg)
                    },
                    |cause| (String::new(), cause.to_string()),
                )
            })
            .into_inner()
            .unwrap_or_default();

        let input = input.to_string();
        let start = err_pos;
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);

        Self {
            title: labels,
            message,
            span: start..end,
            input,
        }
    }

    pub fn from_display<T, U>(message: T, input: U, err_pos: usize) -> Self
    where
        T: core::fmt::Display,
        U: core::fmt::Display,
    {
        let input = input.to_string();
        let start = err_pos;
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);

        Self {
            title: "Validation Error".to_string(),
            message: message.to_string(),
            span: start..end,
            input,
        }
    }
}

impl fmt::Display for ReadableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = annotate_snippets::Level::Error.title(&self.title).snippet(
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
