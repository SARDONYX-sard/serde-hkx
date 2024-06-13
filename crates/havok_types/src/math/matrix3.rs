use crate::error::Result;
use crate::Vector4;
use derive_new::new;
use parse_display::Display;

/// Matrix3x3
///
/// # XML representation
/// - [`Vector4::w`] (4th) isn't used.
/// ```xml
/// <hkparam>(0.000000 0.000000 0.000000)(-0.000000 0.000000 1.000000)(1.000000 1.000000 0.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display, new)]
#[display("{x}{y}{z}")]
pub struct Matrix3 {
    /// # NOTE
    /// `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub x: Vector4,
    /// # NOTE
    /// `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub y: Vector4,
    /// # NOTE
    /// `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub z: Vector4,
}

static_assertions::assert_eq_size!(Matrix3, [u8; 48]);

impl Matrix3 {
    pub fn to_le_bytes(&self) -> [u8; 48] {
        let mut bytes = [0; 48];
        bytes[0..16].copy_from_slice(&self.x.to_le_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_le_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_le_bytes());
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 48] {
        let mut bytes = [0; 48];
        bytes[0..16].copy_from_slice(&self.x.to_be_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_be_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_be_bytes());
        bytes
    }

    #[inline]
    pub fn from_str(s: &str) -> Result<(&str, Self)> {
        let res = crate::tri!(parse_matrix3(s));
        Ok(res)
    }
}

pub fn parse_matrix3(input: &str) -> winnow::PResult<(&str, Matrix3)> {
    use crate::parse_vector3;

    let (input, x) = parse_vector3(input)?;
    let (input, y) = parse_vector3(input)?;
    let (remain, z) = parse_vector3(input)?;
    Ok((remain, Matrix3 { x, y, z }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_str() -> Result<()> {
        let (remain, matrix3) = Matrix3::from_str(
            "(0.000000 0.000000 0.000000)(-0.000000 0.000000 1.000000)(1.000000 1.000000 0.000000)",
        )?;
        assert_eq!(remain, "");
        assert_eq!(
            matrix3,
            Matrix3 {
                x: Vector4::default(),
                y: Vector4 {
                    x: -0.0,
                    y: 0.0,
                    z: 1.0,
                    w: 0.0
                },
                z: Vector4 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                    w: 0.0
                }
            }
        );
        Ok(())
    }
}
