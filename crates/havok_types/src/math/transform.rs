use crate::{Rotation, Vector4};
use parse_display::Display;

/// # XML representation
/// - [`Vector4::w`] (4th) isn't used.
/// ```xml
///          <!--                             Matrix3 rotation                                --><!--   Vector4 transition  -->
/// <hkparam>(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display)]
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
    /// Create a new `Transform`
    pub const fn new(rotation: Rotation, transition: Vector4) -> Self {
        Self {
            rotation,
            transition,
        }
    }

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
}
