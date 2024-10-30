use crate::Vector4;
use parse_display::Display;

/// Rotation (same as [`Matrix3`])
///
/// # C++ Info
/// - name: `hkRotation`
/// - type_size: ` 48`(x86)/` 48`(x86_64)
/// - align: ` 16`(x86)/` 16`(x86_64)
///
/// # Examples
/// ```
/// use havok_types::{Rotation, Vector4};
///
/// assert_eq!(Rotation::new(
///  Vector4::new(0.0, 0.0, 0.0, 0.0),
///  Vector4::new(-0.0, 0.0, 1.0, 0.0),
///  Vector4::new(1.0, 1.0, 0.0, 0.0),
/// ).to_string(), "(0.000000 0.000000 0.000000)(-0.000000 0.000000 1.000000)(1.000000 1.000000 0.000000)");
/// ```
///
/// # Note
/// - [`Vector4::w`] (4th) isn't used.
#[repr(C, align(16))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display)]
#[display("{x}{y}{z}")]
pub struct Rotation {
    /// # C++ Info
    /// - name: `x`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    ///
    /// # NOTE
    /// - `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub x: Vector4,
    /// # C++ Info
    /// - name: `y`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    ///
    /// # NOTE
    /// - `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub y: Vector4,
    /// # C++ Info
    /// - name: `z`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    ///
    /// # NOTE
    /// - `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub z: Vector4,
}

static_assertions::assert_eq_size!(Rotation, [u8; 48]);

impl Rotation {
    /// Creates a new `Rotation`
    #[inline]
    pub const fn new(x: Vector4, y: Vector4, z: Vector4) -> Self {
        Self { x, y, z }
    }

    pub fn to_le_bytes(&self) -> [u8; 48] {
        let mut bytes = [0_u8; 48];
        bytes[0..16].copy_from_slice(&self.x.to_le_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_le_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_le_bytes());
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 48] {
        let mut bytes = [0_u8; 48];
        bytes[0..16].copy_from_slice(&self.x.to_be_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_be_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_be_bytes());
        bytes
    }
}
