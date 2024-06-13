use crate::{Rotation, Vector4};
use derive_new::new;
use parse_display::Display;

/// # XML representation
/// - [`Vector4::w`] (4th) isn't used.
/// ```xml
///          <!--                             Matrix3 rotation                                --><!--   Vector4 transition  -->
/// <hkparam>(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display, new)]
#[display("{rotation}{transition}")]
pub struct Transform {
    /// Rotation part
    pub rotation: Rotation,
    /// # NOTE
    /// `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub transition: Vector4,
}

static_assertions::assert_eq_size!(Transform, [u8; 64]);

impl Transform {
    pub fn to_le_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[0..48].copy_from_slice(&self.rotation.to_le_bytes());
        bytes[48..64].copy_from_slice(&self.transition.to_le_bytes());
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[0..48].copy_from_slice(&self.rotation.to_be_bytes());
        bytes[48..64].copy_from_slice(&self.transition.to_be_bytes());
        bytes
    }

    #[inline]
    pub fn from_str(s: &str) -> crate::error::Result<(&str, Self)> {
        let (remain, quaternion) = crate::tri!(parse_transform(s));
        Ok((remain, quaternion))
    }
}

pub fn parse_transform(input: &str) -> winnow::PResult<(&str, Transform)> {
    use crate::{parse_rotation, parse_vector4};

    let (input, rotation) = parse_rotation(input)?;
    let (remain, transition) = parse_vector4(input)?;
    Ok((
        remain,
        Transform {
            rotation,
            transition,
        },
    ))
}
