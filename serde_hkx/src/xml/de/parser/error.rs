use crate::lib::*;
use winnow::error::{ContextError, ErrMode, ParseError};

/// Error struct to represent parsing errors in a more user-friendly way.
#[derive(Debug)]
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
    pub fn from_context(error: ErrMode<ContextError>, input: &str, err_pos: usize) -> Self {
        let (labels, message) = error
            .map(|ctx_err| {
                let mut labels = String::new();
                let mut msg = "expected ".to_string();

                for ctx in ctx_err.context() {
                    match ctx {
                        winnow::error::StrContext::Label(label) => {
                            labels += label;
                        }
                        winnow::error::StrContext::Expected(expected) => {
                            msg += &expected.to_string();
                        }
                        _ => (),
                    }
                }
                (labels, msg)
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
