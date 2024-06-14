use crate::{error::Result, tri};
use winnow::{combinator::alt, Parser as _};

/// parse [`str`] to [`bool`]
pub fn parse_bool(input: &str) -> Result<(&str, bool)> {
    #[inline]
    fn boolean(input: &str) -> winnow::PResult<(&str, bool)> {
        let parse_true = "true".value(true);
        let parse_false = "false".value(false);
        alt((parse_true, parse_false)).parse_peek(input)
    }

    let (remain, boolean) = tri!(boolean(input));
    Ok((remain, boolean))
}

/// Parse [`str`] to [`f32`]
pub fn parse_float(input: &str) -> Result<(&str, f32)> {
    #[inline]
    fn float(input: &str) -> winnow::PResult<(&str, f32)> {
        let nan = alt((
            "1.#IND",
            "1.#IND0",
            "1.#IND00",
            "-1.#IND",
            "-1.#IND0",
            "-1.#IND00",
        ))
        .map(|_| f32::NAN);
        let pos_inf = alt(("1.#INF", "1.#INF0", "1.#INF00")).map(|_| f32::INFINITY);
        let neg_inf = alt(("-1.#INF", "-1.#INF0", "-1.#INF00")).map(|_| f32::NEG_INFINITY);

        alt((nan, pos_inf, neg_inf, winnow::ascii::float)).parse_peek(input)
    }

    let (remain, float) = tri!(float(input));
    Ok((remain, float))
}
