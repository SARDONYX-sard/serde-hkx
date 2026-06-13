/// take_until implementation using only winnow
pub fn take_until_ext<Input, Output, Error, ParseNext>(
    occurrences: impl Into<winnow::stream::Range>,
    parser: ParseNext,
) -> impl winnow::Parser<Input, Input::Slice, Error>
where
    Input: winnow::stream::StreamIsPartial + winnow::stream::Stream,
    Error: winnow::error::ParserError<Input>,
    ParseNext: winnow::Parser<Input, Output, Error>,
{
    use winnow::{
        Parser as _,
        combinator::{not, peek, repeat, trace},
        token::any,
    };

    trace(
        "take_until_ext",
        repeat::<_, _, (), _, _>(occurrences, (peek(not(parser)), any)).take(),
    )
}
