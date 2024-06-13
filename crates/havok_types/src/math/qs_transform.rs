use crate::{Quaternion, Vector4};
use derive_new::new;
use parse_display::Display;

/// # XML representation
/// - [`Vector4::w`] (4th) of `transition` & `scale` isn't used.
/// ```xml
/// <!--  transition: Vector4 --><!--     rotation: Quaternion      --><!--   scale: Vector4    -->
/// (0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000)
/// ```
///
/// [`Vector4::w`](Vector4)
#[repr(C, align(16))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display, new)]
#[display("{transition}{quaternion}{scale}")]
pub struct QsTransform {
    /// # NOTE
    /// `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub transition: Vector4,
    pub quaternion: Quaternion,
    /// # NOTE
    /// `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub scale: Vector4,
}

impl QsTransform {
    pub fn to_le_bytes(&self) -> [u8; 48] {
        let mut bytes = [0; 48];
        bytes[0..16].copy_from_slice(&self.transition.to_le_bytes());
        bytes[16..32].copy_from_slice(&self.quaternion.to_le_bytes());
        bytes[32..48].copy_from_slice(&self.scale.to_le_bytes());
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 48] {
        let mut bytes = [0; 48];
        bytes[0..16].copy_from_slice(&self.transition.to_le_bytes());
        bytes[16..32].copy_from_slice(&self.quaternion.to_le_bytes());
        bytes[32..48].copy_from_slice(&self.scale.to_le_bytes());
        bytes
    }

    pub fn from_str(s: &str) -> crate::error::Result<(&str, Self)> {
        let res = crate::tri!(parse_qstransform(s));
        Ok(res)
    }
}

fn parse_qstransform(input: &str) -> winnow::PResult<(&str, QsTransform)> {
    use crate::{parse_quaternion, parse_vector3};

    let (input, transition) = parse_vector3(input)?;
    let (input, quaternion) = parse_quaternion(input)?;
    let (remain, scale) = parse_vector3(input)?;
    Ok((
        remain,
        QsTransform {
            transition,
            quaternion,
            scale,
        },
    ))
}
