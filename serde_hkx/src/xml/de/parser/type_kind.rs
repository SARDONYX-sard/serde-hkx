use crate::lib::*;
use crate::tri;

use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{alt, cut_err, preceded, seq};
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::token::take_until;
use winnow::Parser;

use super::tag::end_tag;
use havok_types::*;

/// Parses [`bool`]. `true` or `false``
/// - The corresponding type for this XML: `Bool`
pub fn boolean<'a>() -> impl Parser<&'a str, bool, ContextError> {
    alt(("true".value(true), "false".value(false)))
        .context(StrContext::Label("bool"))
        .context(StrContext::Expected(StrContextValue::Description(
            "`true` or `false`",
        )))
}

// Unsigned integers -> use `dec_unit`
//   Signed integers -> use `dec_nit`

/// Parses [`f32`](`Real`)
pub fn real<'a>() -> impl Parser<&'a str, f32, ContextError> {
    // Need to support special representation of decimal points in C++
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

    alt((nan, pos_inf, neg_inf, winnow::ascii::float))
        .context(StrContext::Label("real(f32)"))
        .context(StrContext::Expected(StrContextValue::Description(
            "e.g. 0.100000",
        )))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Math

/// Parse as [`Vector4`]
///
/// # XML Examples
/// ```xml
/// (-0.000000 0.000000 -0.000000 1.000000)
/// ```
pub fn vector4<'a>() -> impl Parser<&'a str, Vector4, ContextError> {
    seq!(Vector4 {
        _: multispace0,
        _: cut_err('(').context(StrContext::Expected(StrContextValue::CharLiteral('('))),
        _: multispace0,
        x: real(),
        _: multispace0,
        y: real(),
        _: multispace0,
        z: real(),
        _: multispace0,
        w: real(),
        _: multispace0,
        _:  cut_err(')').context(StrContext::Expected(StrContextValue::CharLiteral(')'))),
        _: multispace0,
    })
    .context(StrContext::Label("Vector4"))
}

#[inline]
pub fn quaternion<'a>() -> impl Parser<&'a str, Quaternion, ContextError> {
    move |input: &mut &'a str| {
        let Vector4 { x, y, z, w } = tri!(vector4().parse_next(input));
        Ok(Quaternion { x, y, z, scaler: w })
    }
}

pub fn matrix3<'a>() -> impl Parser<&'a str, Matrix3, ContextError> {
    seq!(Matrix3 {
        x: vector3(),
        y: vector3(),
        z: vector3(),
    })
    .context(StrContext::Label("Matrix3"))
}

pub fn rotation<'a>() -> impl Parser<&'a str, Rotation, ContextError> {
    seq!(Rotation {
        x: vector3(),
        y: vector3(),
        z: vector3(),
    })
    .context(StrContext::Label("Rotation"))
}

pub fn qstransform<'a>() -> impl Parser<&'a str, QsTransform, ContextError> {
    seq!(QsTransform {
        transition: vector4(),
        quaternion: quaternion(),
        scale: vector4(),
    })
    .context(StrContext::Label("QsTransform"))
}

pub fn matrix4<'a>() -> impl Parser<&'a str, Matrix4, ContextError> {
    seq!(Matrix4 {
        x: vector4(),
        y: vector4(),
        z: vector4(),
        w: vector4(),
    })
    .context(StrContext::Label("Matrix4"))
}

pub fn transform<'a>() -> impl Parser<&'a str, Transform, ContextError> {
    seq!(Transform {
        rotation: rotation(),
        transition: vector4(),
    })
    .context(StrContext::Label("Transform"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// - Class pointer. (e.g. `#0050`)
pub fn pointer<'a>() -> impl Parser<&'a str, Pointer, ContextError> {
    move |input: &mut &'a str| {
        let digit = tri!(preceded("#", digit1)
            .parse_to()
            .context(StrContext::Label("Pointer"))
            .context(StrContext::Expected(StrContextValue::Description(
                "Pointer e.g. #0001"
            )))
            .parse_next(input));
        Ok(Pointer::new(digit))
    }
}

/// Parses a string literal until `</hkparam>`, e.g., `example` in (`example</hkparam>`).
/// - The corresponding type for this XML: `CString`, `StringPtr`
///
/// # Attention during unit testing
/// This will get any string until the end tag `</hkparam>` exists, so testing with [`Parser::parse`] will fail
/// because it leaves an unparsed region of the string. (This is because `parse` is intended for use with the root analyzer.)
///
/// Therefore, use the [`Parser::parse_next`] method for testing.
pub fn string<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    move |input: &mut &'a str| {
        let s = take_until(0.., '<')
            .context(StrContext::Label("end-of-string marker tag(`</hkparam>`)"))
            .context(StrContext::Expected(StrContextValue::Description(
                "e.g. Hello</hkparam>",
            )))
            .parse_next(input)?;

        let _ = tri!(end_tag("hkparam").parse_peek(input));
        Ok(s)
    }
}

/// Parses a string in `Array` e.g. `Hello` in `<hkcstring>Hello</hkcstring>`
/// - The corresponding type for this XML: `Array<CString>`, `Array<StringPtr>`
pub fn string_in_array<'a>() -> impl Parser<&'a str, &'a str, ContextError> {
    move |input: &mut &'a str| {
        let s = take_until(0.., '<')
            .context(StrContext::Label(
                "string in Array marker tag(`</hkcstring>`)",
            ))
            .context(StrContext::Expected(StrContextValue::Description(
                "e.g. Hello</hkcstring>",
            )))
            .parse_next(input)?;
        let _ = tri!(end_tag("hkcstring").parse_peek(input));
        Ok(s)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Support functions(These are not used as Havok types)

///After reading off whitespace including line breaks and interpreting it as `Vector3`, put `0.0`
/// in the w element to make `Vector4`.
fn vector3<'a>() -> impl Parser<&'a str, Vector4, ContextError> {
    struct Vector3 {
        x: f32,
        y: f32,
        z: f32,
    }

    move |input: &mut &'a str| {
        let Vector3 { x, y, z } = tri!(seq!(Vector3 {
            _: multispace0,
            _: cut_err('(').context(StrContext::Expected(StrContextValue::CharLiteral('('))),
            _: multispace0,
            x: real(),
            _: multispace0,
            y: real(),
            _: multispace0,
            z: real(),
            _: multispace0,
            _:  cut_err(')').context(StrContext::Expected(StrContextValue::CharLiteral(')'))),
            _: multispace0,
        })
        .context(StrContext::Label("Vector3"))
        .parse_next(input));

        Ok(Vector4 { x, y, z, w: 0.0 })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_boolean() {
        assert_eq!(boolean().parse("true"), Ok(true));
        assert_eq!(boolean().parse("false"), Ok(false));
        assert!(boolean().parse("yes").is_err());
    }

    #[test]
    fn test_string() {
        let mut input = "example</hkparam>";
        assert_eq!(string().parse_next(&mut input), Ok("example"));

        let invalid_input = "example</not_hkparam>";
        assert!(string().parse_peek(invalid_input).is_err());
    }

    #[test]
    fn test_string_in_array() {
        let input = "Hello</hkcstring>";
        assert_eq!(string_in_array().parse(input), Ok("Hello"));

        let invalid_input = "Hello<hkcstring>";
        assert!(string_in_array().parse(invalid_input).is_err());
    }
}
