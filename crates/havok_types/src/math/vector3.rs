use crate::Vector4;

pub fn parse_vector3(input: &str) -> winnow::PResult<(&str, Vector4)> {
    use winnow::ascii::{float, space0};
    use winnow::combinator::{cut_err, seq};
    use winnow::error::{StrContext, StrContextValue};
    use winnow::Parser;

    struct Vector3 {
        x: f32,
        y: f32,
        z: f32,
    }

    let (remain,  Vector3{ x, y, z }) = seq!(Vector3 {
            _: space0,
            _: cut_err('(').context(StrContext::Expected(StrContextValue::CharLiteral('('))),
            _: space0,
            x: float.context(StrContext::Expected(StrContextValue::Description("float for x component"))),
            _: space0,
            y: float.context(StrContext::Expected(StrContextValue::Description("float for y component"))),
            _: space0,
            z: float.context(StrContext::Expected(StrContextValue::Description("float for z component"))),
            _:space0,
            _:  cut_err(')').context(StrContext::Expected(StrContextValue::CharLiteral(')'))),
            _: space0,
        })
    .parse_peek(input)?;

    Ok((remain, Vector4 { x, y, z, w: 0.0 }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            parse_vector3("(-0.000000 0.000000 1.000000)").unwrap(),
            ("", Vector4::new(-0.0, 0.0, 1.0, 0.0))
        );
    }
}
