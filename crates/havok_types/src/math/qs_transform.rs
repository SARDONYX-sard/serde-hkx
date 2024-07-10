use crate::{Quaternion, Vector4};
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
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display)]
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
    /// Creates a new `QsTransform`
    pub const fn new(transition: Vector4, quaternion: Quaternion, scale: Vector4) -> Self {
        Self {
            transition,
            quaternion,
            scale,
        }
    }

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
}
