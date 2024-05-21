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
