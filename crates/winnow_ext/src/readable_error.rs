use core::{fmt, ops::Range};

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
    #[inline]
    pub fn from_parse(error: ParseError<&str, ContextError>) -> Self {
        let message = error.inner().to_string();
        let input = (*error.input()).to_string();
        let span = error.char_span();
        Self {
            title: "Parse error".to_string(),
            message,
            span,
            input,
        }
    }

    /// Constructs [`Self`] from parse error & input.
    #[inline]
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
        let span = char_boundary(input.as_bytes(), err_pos);

        Self {
            title: labels,
            message,
            span,
            input,
        }
    }

    #[inline]
    pub fn from_display<T, U>(message: T, input: U, err_pos: usize) -> Self
    where
        T: fmt::Display,
        U: fmt::Display,
    {
        let input = input.to_string();
        let span = char_boundary(input.as_bytes(), err_pos);

        Self {
            title: "Validation Error".to_string(),
            message: message.to_string(),
            span,
            input,
        }
    }
}

impl fmt::Display for ReadableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let report = &[annotate_snippets::Level::ERROR
            .primary_title(&self.title)
            .element(
                annotate_snippets::Snippet::source(&self.input)
                    .fold(true)
                    .annotation(
                        annotate_snippets::AnnotationKind::Context
                            .span(self.span.clone())
                            .label(&self.message),
                    ),
            )];

        let rendered = annotate_snippets::Renderer::plain().render(report);
        rendered.fmt(f)
    }
}

impl std::error::Error for ReadableError {}

/// winnow method
fn char_boundary(input: &[u8], offset: usize) -> core::ops::Range<usize> {
    let len = input.len();
    if offset == len {
        return offset..offset;
    }

    /// Taken from `core::num`
    const fn is_utf8_char_boundary(b: u8) -> bool {
        // This is bit magic equivalent to: b < 128 || b >= 192
        (b as i8) >= -0x40
    }

    let start = (0..(offset + 1).min(len))
        .rev()
        .find(|i| input.get(*i).copied().is_some_and(is_utf8_char_boundary))
        .unwrap_or(0);
    let end = (offset + 1..len)
        .find(|i| input.get(*i).copied().is_some_and(is_utf8_char_boundary))
        .unwrap_or(len);
    start..end
}
