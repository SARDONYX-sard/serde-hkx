//! TypeKind XML parsers

use crate::{lib::*, tri};

use super::delimited_with_multispace0;
use havok_types::*;
use std::borrow::Cow;
use winnow::ascii::{Caseless, digit1, float, multispace0};
use winnow::combinator::{alt, opt, preceded, seq};
use winnow::error::{StrContext, StrContextValue};
use winnow::token::take_until;
use winnow::{ModalResult, Parser};

/// Parses [`bool`]. `true` or `false`
/// - The corresponding type kind: `Bool`
///
/// # Examples
///
/// ```
/// use serde_hkx::xml::de::parser::type_kind::boolean;
/// use winnow::Parser as _;
///
/// assert_eq!(boolean.parse("true"), Ok(true));
/// assert_eq!(boolean.parse("false"), Ok(false));
/// assert!(boolean.parse("invalid").is_err());
/// ```
///
/// # Errors
/// When parse failed.
pub fn boolean(input: &mut &str) -> ModalResult<bool> {
    alt(("true".value(true), "false".value(false)))
        .context(StrContext::Label("bool"))
        .context(StrContext::Expected(StrContextValue::Description(
            "`true` or `false`",
        )))
        .parse_next(input)
}

// Unsigned integers -> use `dec_unit`
//   Signed integers -> use `dec_nit`

/// Parses [`f32`](`Real`)
///
/// # Examples
///
/// ```
/// use serde_hkx::xml::de::parser::type_kind::real;
/// use winnow::Parser as _;
///
/// assert_eq!(real.parse("1.0"), Ok(1.0_f32));
/// assert_eq!(real.parse("0.1"), Ok(0.1_f32));
/// assert_eq!(real.parse("0"), Ok(0.0_f32)); // Integer format
/// assert_eq!(real.parse("1"), Ok(1.0_f32)); // Integer format
/// assert_eq!(real.parse("-1"), Ok(-1.0_f32)); // Negative integer format
///
/// // C++ indeterminate
/// assert!(real.parse("1.#IND").unwrap().is_nan());
/// assert!(real.parse("1.#IND0").unwrap().is_nan());
/// assert!(real.parse("1.#IND00").unwrap().is_nan());
/// assert!(real.parse("-1.#IND0").unwrap().is_nan());
/// assert!(real.parse("-1.#IND00").unwrap().is_nan());
///
/// // C++ infinity
/// assert_eq!(real.parse("1.#INF"), Ok(f32::INFINITY));
/// // assert_eq!(real.parse("-1.#INF00"), Ok(f32::NEG_INFINITY));
///
/// assert!(real.parse("invalid").is_err());
/// ```
///
/// # Errors
/// When parse failed.
pub fn real(input: &mut &str) -> ModalResult<f32> {
    use winnow::ascii::digit1;
    use winnow::combinator::opt;

    // NOTE: For some reason, early errors occur if the display digits, such as 00, are not parsed first.

    // Need to support special representation of decimal points in C++
    let nan = alt((
        "1.#IND00",
        "1.#IND0",
        "1.#IND",
        "-1.#IND00",
        "-1.#IND0",
        "-1.#IND",
    ))
    .value(f32::NAN);
    let pos_inf = alt(("1.#INF00", "1.#INF0", "1.#INF")).value(f32::INFINITY);
    let neg_inf = alt(("-1.#INF00", "-1.#INF0", "-1.#INF")).value(f32::NEG_INFINITY);

    // Parse integers as floats (e.g., "0" -> 0.0, "1" -> 1.0, "-1" -> -1.0)
    let integer_as_float = seq!(opt("-"), digit1)
        .take()
        .try_map(|s: &str| s.parse::<f32>());

    alt((nan, pos_inf, neg_inf, float, integer_as_float))
        .context(StrContext::Label("real(f32)"))
        .context(StrContext::Expected(StrContextValue::Description(
            "Real(e.g. `0.100000` or `0`)",
        )))
        .parse_next(input)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Math

/// Parse as [`Vector4`]
///
/// # Examples
///
/// ```
/// use havok_types::Vector4;
/// use serde_hkx::xml::de::parser::type_kind::vector4;
/// use winnow::Parser as _;
///
/// assert_eq!(vector4.parse("(1.000000 1.000000 1.000000 0.000000)"), Ok(Vector4::new(1.0, 1.0, 1.0, 0.0)));
/// assert_eq!(vector4.parse("(-0.000000 0.000000 -0.000000 1.000000)"), Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0)));
/// assert_eq!(vector4.parse("   (   -0.000000 0.000000 -0.000000 1.000000  ) "), Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0)));
/// assert_eq!(vector4.parse("(0,0,0,0)"), Ok(Vector4::new(0.0, 0.0, 0.0, 0.0)));
/// assert_eq!(vector4.parse("(1,2,3,4)"), Ok(Vector4::new(1.0, 2.0, 3.0, 4.0)));
/// ```
///
/// # Errors
/// When parse failed.
pub fn vector4(input: &mut &str) -> ModalResult<Vector4> {
    use winnow::combinator::alt;

    // Helper function to parse separator (either comma or space)
    let mut separator = alt((
        delimited_with_multispace0(","), // comma-separated format: (0,1,2,3)
        multispace0,                     // space-separated format: (0 1 2 3)
    ));

    seq!(Vector4 {
        _: opt(delimited_with_multispace0("(")),
        x: real.context(StrContext::Label("x")),
        _: separator,
        y: real.context(StrContext::Label("y")),
        _: separator,
        z: real.context(StrContext::Label("z")),
        _: separator,
        w: real.context(StrContext::Label("w")),
        _: opt(delimited_with_multispace0(")")),
    })
    .context(StrContext::Label("Vector4"))
    .parse_next(input)
}

/// Parse as [`Quaternion`]
///
/// # Examples
/// ```
/// use havok_types::Quaternion;
/// use serde_hkx::xml::de::parser::type_kind::quaternion;
/// use winnow::Parser as _;
///
/// assert_eq!(
///     quaternion.parse("   (   -0.000000 0.000000 -0.000000 1.000000  ) "),
///     Ok(Quaternion::new(-0.0, 0.0, -0.0, 1.0))
/// );
/// ```
///
/// # Errors
/// When parse failed.
#[inline]
pub fn quaternion(input: &mut &str) -> ModalResult<Quaternion> {
    let Vector4 { x, y, z, w } = tri!(
        vector4
            .context(StrContext::Label("Quaternion"))
            .parse_next(input)
    );
    Ok(Quaternion { x, y, z, scaler: w })
}

/// Parse as [`Matrix3`]
///
/// # Examples
/// ```
/// use havok_types::{Matrix3, Vector4};
/// use serde_hkx::xml::de::parser::type_kind::matrix3;
/// use winnow::Parser as _;
///
/// assert_eq!(matrix3.parse("
///    (0.000000 0.000000 0.000000)
///    (-0.000000 0.000000 -0.000000)
///    (1.000000 1.000000 1.000000)
/// "), Ok(Matrix3::new(
///    Vector4::default(),
///    Vector4::new(-0.0, 0.0, -0.0, 0.0),
///    Vector4::new(1.0, 1.0, 1.0, 0.0),
/// )));
/// ```
///
/// # Errors
/// When parse failed.
pub fn matrix3(input: &mut &str) -> ModalResult<Matrix3> {
    seq!(Matrix3 {
        x: vector3.context(StrContext::Label("x")),
        y: vector3.context(StrContext::Label("y")),
        z: vector3.context(StrContext::Label("z")),
    })
    .context(StrContext::Label("Matrix3"))
    .parse_next(input)
}

/// Parse as [`Rotation`]
///
/// # Examples
/// ```
/// use havok_types::{Rotation, Vector4};
/// use serde_hkx::xml::de::parser::type_kind::rotation;
/// use winnow::Parser as _;
///
/// assert_eq!(rotation.parse("
///    (0.000000 0.000000 0.000000)
///    (-0.000000 0.000000 -0.000000)
///    (1.000000 1.000000 1.000000)
/// "), Ok(Rotation::new(
///    Vector4::default(),
///    Vector4::new(-0.0, 0.0, -0.0, 0.0),
///    Vector4::new(1.0, 1.0, 1.0, 0.0),
/// )));
/// ```
///
/// # Errors
/// When parse failed.
pub fn rotation(input: &mut &str) -> ModalResult<Rotation> {
    seq!(Rotation {
        x: vector3.context(StrContext::Label("x")),
        y: vector3.context(StrContext::Label("y")),
        z: vector3.context(StrContext::Label("z")),
    })
    .context(StrContext::Label("Rotation"))
    .parse_next(input)
}

/// Parse as [`QsTransform`]
///
/// # Examples
/// ```
/// use havok_types::{QsTransform, Quaternion,Vector4};
/// use serde_hkx::xml::de::parser::type_kind::qstransform;
/// use winnow::Parser as _;
///
/// assert_eq!(qstransform.parse("
///    (0.000000 0.000000 0.000000)
///    (-0.000000 0.000000 -0.000000 0.000000)
///    (1.000000 1.000000 1.000000)
/// "), Ok(QsTransform::new(
///    Vector4::default(),
///    Quaternion::new(-0.0, 0.0, -0.0, 0.0),
///    Vector4::new(1.0, 1.0, 1.0, 0.0),
/// )));
/// ```
///
/// # Errors
/// When parse failed.
pub fn qstransform(input: &mut &str) -> ModalResult<QsTransform> {
    seq!(QsTransform {
        transition: vector3.context(StrContext::Label("transition")),
        quaternion: quaternion.context(StrContext::Label("quaternion")),
        scale: vector3.context(StrContext::Label("scale")),
    })
    .context(StrContext::Label("QsTransform"))
    .parse_next(input)
}

/// Parse as [`Matrix4`]
///
/// # Examples
/// ```
/// use havok_types::{Matrix4, Vector4};
/// use serde_hkx::xml::de::parser::type_kind::matrix4;
/// use winnow::Parser as _;
///
/// assert_eq!(matrix4.parse("
///    (0.000000 0.000000 0.000000 0.000000)
///    (-0.000000 0.000000 -0.000000 0.000000)
///    (1.000000 1.000000 1.000000 0.000000)
///    (1.000000 1.000000 1.000000 0.000000)
/// "), Ok(Matrix4::new(
///    Vector4::default(),
///    Vector4::new(-0.0, 0.0, -0.0, 0.0),
///    Vector4::new(1.0, 1.0, 1.0, 0.0),
///    Vector4::new(1.0, 1.0, 1.0, 0.0),
/// )));
/// ```
///
/// # Errors
/// When parse failed.
pub fn matrix4(input: &mut &str) -> ModalResult<Matrix4> {
    seq!(Matrix4 {
        x: vector4.context(StrContext::Label("x")),
        y: vector4.context(StrContext::Label("y")),
        z: vector4.context(StrContext::Label("z")),
        w: vector4.context(StrContext::Label("w")),
    })
    .context(StrContext::Label("Matrix4"))
    .parse_next(input)
}

/// Parse as [`Transform`]
///
/// # Examples
/// ```
/// use havok_types::{Rotation, Transform, Vector4};
/// use serde_hkx::xml::de::parser::type_kind::transform;
/// use winnow::Parser as _;
///
/// assert_eq!(transform.parse("
///    (0.000000 0.000000 0.000000)
///    (-0.000000 0.000000 -0.000000)
///    (1.000000 1.000000 1.000000)
///
///    (0.000000 0.000000 1.000000)
/// "), Ok(Transform::new(
///    Rotation::new(
///         Vector4::default(),
///         Vector4::new(-0.0, 0.0, -0.0, 0.0),
///         Vector4::new(1.0, 1.0, 1.0, 0.0),
///    ),
///    Vector4::new(0.0, 0.0, 1.0, 1.0),
/// )));
/// ```
///
/// # Errors
/// When parse failed.
///
/// # Why is the w(of transition) in transform 1.0?
/// Must be 1.0 for affine conversion.
/// ```txt
/// [
///  // Rotation
///  [1, 0, 0, tx],
///  [0, 1, 0, ty],
///  [0, 0, 1, tz],
///
///  [0, 0, 0,  1], // transition
/// ]
/// ```
pub fn transform(input: &mut &str) -> ModalResult<Transform> {
    seq!(Transform {
        rotation: rotation.context(StrContext::Label("rotation")),
        transition: vector3
            .context(StrContext::Label("transition"))
            .map(|mut vec4| {
                vec4.w = 1.0; // To affine conversion.
                vec4
            }),
    })
    .context(StrContext::Label("Transform"))
    .parse_next(input)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///
/// Parse as [`Pointer`]
/// - Class pointer. (e.g. `#0050`, `null`, etc.)
///
/// # Examples
/// ```
/// use havok_types::Pointer;
/// use serde_hkx::xml::de::parser::type_kind::pointer;
/// use winnow::Parser as _;
///
/// assert_eq!(pointer.parse("null"), Ok(Pointer::new(0))); // null pointer
/// assert_eq!(pointer.parse("#0000"), Ok(Pointer::new(0))); // null pointer
/// assert_eq!(pointer.parse("#0100"), Ok(Pointer::new(100)));
/// assert!(pointer.parse("Non_Prefix#").is_err());
/// ```
///
/// # Errors
/// When parse failed.
pub fn pointer(input: &mut &str) -> ModalResult<Pointer> {
    let null_ptr = Caseless("null")
        .value(Pointer::new(0))
        .context(StrContext::Label("Pointer"))
        .context(StrContext::Expected(StrContextValue::Description(
            "Pointer(e.g. `null`)",
        )));

    alt((null_ptr, move |input: &mut &str| {
        let digit = tri!(
            preceded("#", digit1)
                .parse_to()
                .context(StrContext::Label("Pointer"))
                .context(StrContext::Expected(StrContextValue::Description(
                    "Pointer(e.g. `#0001`)"
                )))
                .parse_next(input)
        );
        Ok(Pointer::new(digit))
    }))
    .parse_next(input)
}

/// Parses a string literal until `</`, e.g., `example` in (`example</`).
/// - The corresponding type kind: `CString`, `StringPtr`
///
/// # Examples
/// ```
/// use serde_hkx::xml::de::parser::type_kind::string;
/// use std::borrow::Cow;
/// use winnow::Parser as _;
/// assert_eq!(string.parse_next(&mut "example</"), Ok("example".into()));
/// assert_eq!(
///     string.parse_next(&mut "example</not_hkparam>"),
///     Ok("example".into())
/// );
///
/// assert_eq!(string.parse_next(&mut "&#9216;</"), Ok("\u{2400}".into()));
/// let mut escaped =
///     "This is a &lt;test&gt; &amp; &quot;example&quot; &apos; &#9216; &#x2400;</";
/// assert_eq!(
///     string.parse_next(&mut escaped),
///     Ok(Cow::Borrowed(
///         "This is a <test> & \"example\" ' \u{2400} \u{2400}"
///     ))
/// );
/// ```
///
/// # Errors
/// When parse failed.
pub fn string<'a>(input: &mut &'a str) -> ModalResult<Cow<'a, str>> {
    take_until(0.., "</")
        .map(|s| html_escape::decode_html_entities(s))
        .context(StrContext::Label(
            "end of string tag(`</hkparam>`, `</hkcstring>` in Array)",
        ))
        .context(StrContext::Expected(StrContextValue::Description(
            "e.g. Hello</hkparam>",
        )))
        .parse_next(input)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Support functions(These are not used as Havok types)

/// Parse as `Vector3`
/// - After reading off whitespace including line breaks and interpreting it as `Vector3`, put `0.0`
///   in the w element to make `Vector4`.
///
/// # Examples
///
/// ```
/// use havok_types::Vector4;
/// use serde_hkx::xml::de::parser::type_kind::vector3;
/// use winnow::Parser as _;
///
/// assert_eq!(vector3.parse("(1.000000 1.000000 1.000000)"), Ok(Vector4::new(1.0, 1.0, 1.0, 0.0)));
/// assert_eq!(vector3.parse("(-0.000000 0.000000 -0.000000)"), Ok(Vector4::new(-0.0, 0.0, -0.0, 0.0)));
/// assert_eq!(vector3.parse("   (   -0.000000 0.000000 -0.000000) "), Ok(Vector4::new(-0.0, 0.0, -0.0, 0.0)));
/// assert_eq!(vector3.parse("(0,0,0)"), Ok(Vector4::new(0.0, 0.0, 0.0, 0.0)));
/// assert_eq!(vector3.parse("(1,2,3)"), Ok(Vector4::new(1.0, 2.0, 3.0, 0.0)));
/// ```
///
/// # Errors
/// When parse failed.
pub fn vector3(input: &mut &str) -> ModalResult<Vector4> {
    use winnow::combinator::alt;

    // Helper function to parse separator (either comma or space)
    let mut separator = alt((
        delimited_with_multispace0(","), // comma-separated format: (0,1,2)
        multispace0,                     // space-separated format: (0 1 2)
    ));

    struct Vector3 {
        x: f32,
        y: f32,
        z: f32,
    }

    let Vector3 { x, y, z } = tri!(
        seq!(Vector3 {
            _: opt(delimited_with_multispace0("(")),
            x: real.context(StrContext::Label("x")),
            _: separator,
            y: real.context(StrContext::Label("y")),
            _: separator,
            z: real.context(StrContext::Label("z")),
            _: opt(delimited_with_multispace0(")")),
        })
        .context(StrContext::Label("Vector3"))
        .parse_next(input)
    );

    Ok(Vector4 { x, y, z, w: 0.0 })
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_matrix3() {
        assert_eq!(
            matrix3.parse("(0.000000 0.000000 0.000000)(-0.000000 0.000000 1.000000)(1.000000 1.000000 0.000000)"),
            Ok(Matrix3 {
                x: Vector4::default(),
                y: Vector4 { x: -0.0, y: 0.0, z: 1.0, w: 0.0 },
                z: Vector4 { x: 1.0, y: 1.0, z: 0.0, w: 0.0 }
            })
        );
    }
}
