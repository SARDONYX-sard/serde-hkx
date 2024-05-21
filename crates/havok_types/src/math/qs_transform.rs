use super::vector4::Vector4;
use crate::Quaternion;
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
