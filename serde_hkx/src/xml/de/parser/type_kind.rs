//! TypeKind XML parsers

use crate::{lib::*, tri};

use super::delimited_with_multispace0;
use havok_types::*;
use std::borrow::Cow;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{alt, preceded, seq};
use winnow::error::{ContextError, StrContext, StrContextValue};
use winnow::token::take_until;
use winnow::Parser;

/// Parses [`bool`]. `true` or `false``
/// - The corresponding type kind: `Bool`
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
            "Real(e.g. `0.100000`)",
        )))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Math

/// Parse as [`Vector4`]
///
/// ```
/// use havok_types::Vector4;
/// use serde_hkx::xml::de::parser::type_kind::vector4;
/// use winnow::Parser as _;
///
/// assert_eq!(vector4().parse("(1.000000 1.000000 1.000000 0.000000)"), Ok(Vector4::new(1.0, 1.0, 1.0, 0.0)));
/// assert_eq!(vector4().parse("(-0.000000 0.000000 -0.000000 1.000000)"), Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0)));
/// assert_eq!(vector4().parse("   (   -0.000000 0.000000 -0.000000 1.000000  ) "), Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0)));
/// ```
pub fn vector4<'a>() -> impl Parser<&'a str, Vector4, ContextError> {
    seq!(Vector4 {
        _: delimited_with_multispace0("("),
        x: real().context(StrContext::Label("x")),
        _: multispace0,
        y: real().context(StrContext::Label("y")),
        _: multispace0,
        z: real().context(StrContext::Label("z")),
        _: multispace0,
        w: real().context(StrContext::Label("w")),
        _: delimited_with_multispace0(")"),
    })
    .context(StrContext::Label("Vector4"))
}

/// Attempt to extract from the string representation in the tag on XML.
///
/// # Examples
/// ```
/// use havok_types::Quaternion;
/// use serde_hkx::xml::de::parser::type_kind::quaternion;
/// use winnow::Parser as _;
/// assert_eq!(quaternion().parse("   (   -0.000000 0.000000 -0.000000 1.000000  ) "), Ok(Quaternion::new(-0.0, 0.0, -0.0, 1.0)));
/// ```
#[inline]
pub fn quaternion<'a>() -> impl Parser<&'a str, Quaternion, ContextError> {
    move |input: &mut &'a str| {
        let Vector4 { x, y, z, w } = tri!(vector4().parse_next(input));
        Ok(Quaternion { x, y, z, scaler: w })
    }
}

pub fn matrix3<'a>() -> impl Parser<&'a str, Matrix3, ContextError> {
    seq!(Matrix3 {
        x: vector3().context(StrContext::Label("x")),
        y: vector3().context(StrContext::Label("y")),
        z: vector3().context(StrContext::Label("z")),
    })
    .context(StrContext::Label("Matrix3"))
}

pub fn rotation<'a>() -> impl Parser<&'a str, Rotation, ContextError> {
    seq!(Rotation {
        x: vector3().context(StrContext::Label("x")),
        y: vector3().context(StrContext::Label("y")),
        z: vector3().context(StrContext::Label("z")),
    })
    .context(StrContext::Label("Rotation"))
}

pub fn qstransform<'a>() -> impl Parser<&'a str, QsTransform, ContextError> {
    seq!(QsTransform {
        transition: vector4().context(StrContext::Label("transition")),
        quaternion: quaternion().context(StrContext::Label("quaternion")),
        scale: vector4().context(StrContext::Label("scale")),
    })
    .context(StrContext::Label("QsTransform"))
}

pub fn matrix4<'a>() -> impl Parser<&'a str, Matrix4, ContextError> {
    seq!(Matrix4 {
        x: vector4().context(StrContext::Label("x")),
        y: vector4().context(StrContext::Label("y")),
        z: vector4().context(StrContext::Label("z")),
        w: vector4().context(StrContext::Label("w")),
    })
    .context(StrContext::Label("Matrix4"))
}

pub fn transform<'a>() -> impl Parser<&'a str, Transform, ContextError> {
    seq!(Transform {
        rotation: rotation().context(StrContext::Label("rotation")),
        transition: vector4().context(StrContext::Label("transition")),
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
                "Pointer(e.g. `#0001`)"
            )))
            .parse_next(input));
        Ok(Pointer::new(digit))
    }
}

/// Parses a string literal until `</`, e.g., `example` in (`example</`).
/// - The corresponding type kind: `CString`, `StringPtr`
pub fn string<'a>() -> impl Parser<&'a str, Cow<'a, str>, ContextError> {
    take_until(0.., "</")
        .map(|s| html_escape::decode_html_entities(s))
        .context(StrContext::Label(
            "end of string tag(`</hkparam>`, `</hkcstring>` in Array)",
        ))
        .context(StrContext::Expected(StrContextValue::Description(
            "e.g. Hello</hkparam>",
        )))
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
            _: delimited_with_multispace0("("),
            x: real().context(StrContext::Label("x")),
            _: multispace0,
            y: real().context(StrContext::Label("y")),
            _: multispace0,
            z: real().context(StrContext::Label("z")),
            _: delimited_with_multispace0(")"),
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
    fn test_vector4() {
        assert_eq!(
            vector4().parse("(-0.000000 0.000000 -0.000000 1.000000)"),
            Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0))
        )
    }

    #[test]
    fn test_matrix3() {
        assert_eq!(
            matrix3().parse("(0.000000 0.000000 0.000000)(-0.000000 0.000000 1.000000)(1.000000 1.000000 0.000000)"),
            Ok(Matrix3 {
                x: Vector4::default(),
                y: Vector4 { x: -0.0, y: 0.0, z: 1.0, w: 0.0 },
                z: Vector4 { x: 1.0, y: 1.0, z: 0.0, w: 0.0 }
            })
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(string().parse_next(&mut "example</"), Ok("example".into()));
        assert_eq!(
            string().parse_next(&mut "example</not_hkparam>"),
            Ok("example".into())
        );

        assert_eq!(string().parse_next(&mut "&#9216;</"), Ok("\u{2400}".into()));
        let mut escaped =
            "This is a &lt;test&gt; &amp; &quot;example&quot; &apos; &#9216; &#x2400;</";
        assert_eq!(
            string().parse_next(&mut escaped),
            Ok(Cow::Borrowed(
                "This is a <test> & \"example\" ' \u{2400} \u{2400}"
            ))
        );
    }
}
