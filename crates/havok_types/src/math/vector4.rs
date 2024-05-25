use std::str::FromStr;

use derive_new::new;
use parse_display::Display;
#[cfg(feature = "bytes")]
use zerocopy::{AsBytes, FromBytes, FromZeroes};

/// Vector4 for Havok C++ Class.
///
/// # Examples
/// ```
/// use havok_types::Vector4;
///
/// assert_eq!(Vector4::new(1.0, 1.0, 1.0, 0.0).to_string(), "(1.000000 1.000000 1.000000 0.000000)");
/// assert_eq!(Vector4::new(-0.0, 0.0, -0.0, 1.0).to_string(), "(-0.000000 0.000000 -0.000000 1.000000)");
///
/// assert_eq!("(1.000000 1.000000 1.000000 0.000000)".parse(), Ok(Vector4::new(1.0, 1.0, 1.0, 0.0)));
/// assert_eq!("(-0.000000 0.000000 -0.000000 1.000000)".parse(), Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0)));
/// ```
///
/// # XML representation
/// ```xml
/// <hkparam name="">
///   <!-- x         y        z         w -->
///   (-0.000000 0.000000 -0.000000 1.000000)
/// </hkparam>
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytes", derive(AsBytes, FromBytes, FromZeroes))]
#[repr(C, align(16))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display, new)]
#[display("({x:.06} {y:.06} {z:.06} {w:.06})")]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    #[inline]
    pub fn to_le_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&self.x.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.w.to_le_bytes());
        bytes
    }

    #[inline]
    pub fn to_be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.w.to_be_bytes());
        bytes
    }

    #[inline]
    pub fn from_le_bytes(bytes: &[u8; 16]) -> Self {
        let x = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let y = f32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let z = f32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let w = f32::from_le_bytes(bytes[12..16].try_into().unwrap());
        Self { x, y, z, w }
    }

    #[inline]
    pub fn from_be_bytes(bytes: &[u8; 16]) -> Self {
        let x = f32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let y = f32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let z = f32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let w = f32::from_be_bytes(bytes[12..16].try_into().unwrap());
        Self { x, y, z, w }
    }
}

static_assertions::assert_eq_size!(Vector4, [u8; 16]); // Vector4 must be 16bytes size.
static_assertions::assert_eq_align!(Vector4, u128); // Vector4 must be 16bytes(16 * 8 = 128bit) align.

impl FromStr for Vector4 {
    type Err = String;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use winnow::error::ErrMode;
        match parse_vector4(s) {
            Ok((_, vec)) => Ok(vec),
            Err(e) => Err(match e {
                ErrMode::Incomplete(e) => format!("{e:?}"),
                ErrMode::Backtrack(e) | ErrMode::Cut(e) => e.to_string(),
            }),
        }
    }
}

pub fn parse_vector4(input: &str) -> winnow::PResult<(&str, Vector4)> {
    use winnow::ascii::{float, space0};
    use winnow::combinator::{cut_err, delimited, seq};
    use winnow::error::{StrContext, StrContextValue};
    use winnow::Parser;

    delimited(
        cut_err('(').context(StrContext::Expected(StrContextValue::CharLiteral('('))),
        seq!(Vector4 {
                    _: space0,
                    x: float.context(StrContext::Expected(StrContextValue::Description("float for x component"))),
                    _: space0,
                    y: float.context(StrContext::Expected(StrContextValue::Description("float for y component"))),
                    _: space0,
                    z: float.context(StrContext::Expected(StrContextValue::Description("float for z component"))),
                    _:space0,
                    w: float.context(StrContext::Expected(StrContextValue::Description("float for w component"))),
                    _:space0,
                }),
        cut_err(')').context(StrContext::Expected(StrContextValue::CharLiteral(')'))),
    )
    .parse_peek(input)
}

#[allow(dead_code)]
pub fn parse_vector3(input: &str) -> winnow::PResult<(&str, Vector4)> {
    use winnow::ascii::{float, space0};
    use winnow::combinator::{cut_err, delimited, seq};
    use winnow::error::{StrContext, StrContextValue};
    use winnow::Parser;

    struct Vector3 {
        x: f32,
        y: f32,
        z: f32,
    }

    let v = delimited(
        cut_err('(').context(StrContext::Expected(StrContextValue::CharLiteral('('))),
        seq!(Vector3 {
                    _: space0,
                    x: float.context(StrContext::Expected(StrContextValue::Description("float for x component"))),
                    _: space0,
                    y: float.context(StrContext::Expected(StrContextValue::Description("float for y component"))),
                    _: space0,
                    z: float.context(StrContext::Expected(StrContextValue::Description("float for z component"))),
                    _:space0,
                }),
        cut_err(')').context(StrContext::Expected(StrContextValue::CharLiteral(')'))),
    )
    .parse_peek(input)?;

    Ok((
        v.0,
        Vector4 {
            x: v.1.x,
            y: v.1.y,
            z: v.1.z,
            w: 0.0,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        #[rustfmt::skip]
        let expected_bytes: &[u8; 16] = &[
            0, 0, 128, 63, // 1.0 in bytes (little-endian): 0x3F800000 → [0x00, 0x00, 0x80, 0x3F]
            0, 0, 128, 63, // 1.0 in bytes (little-endian): 0x3F800000 → [0x00, 0x00, 0x80, 0x3F]
            0, 0, 128, 63, // 1.0 in bytes (little-endian): 0x3F800000 → [0x00, 0x00, 0x80, 0x3F]
            0, 0, 0, 0,    // 0.0 in bytes (little-endian): 0x00000000 → [0x00, 0x00, 0x00, 0x00]
        ];

        assert_eq!(
            Vector4::read_from_prefix(expected_bytes),
            Some(Vector4::new(1.0, 1.0, 1.0, 0.0))
        );
        assert_eq!(
            &Vector4::new(1.0, 1.0, 1.0, 0.0).to_le_bytes(),
            expected_bytes,
        );
        assert_eq!(Vector4::new(1.0, 1.0, 1.0, 0.0).as_bytes(), expected_bytes);

        assert_eq!(
            "(-0.000000 0.000000 -0.000000 1.000000) others".parse(),
            Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0))
        );
        assert_eq!(
            "(-0.000000 0.000000j -0.000000 1.000000)".parse(),
            Ok(Vector4::new(-0.0, 0.0, -0.0, 1.0))
        );
    }
}
