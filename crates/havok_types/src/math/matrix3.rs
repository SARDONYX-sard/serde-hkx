use super::vector4::Vector4;
use derive_new::new;
use parse_display::Display;

/// Matrix3x3
///
/// # XML representation
/// - [`Vector4::w`] (4th) isn't used.
/// ```xml
/// <hkparam>(0.000000 0.000000 0.000000)(-0.000000 0.000000 1.000000)(1.000000 1.000000 0.000000)(1.000000 1.000000 0.000000)</hkparam>
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
}
