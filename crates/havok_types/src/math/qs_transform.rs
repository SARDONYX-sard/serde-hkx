use crate::{Quaternion, Vector4};
use parse_display::Display;

/// # QsTransform
///
/// # C++ Info
/// - name: `hkQsTransform`
/// - type_size: ` 48`(x86)/` 48`(x86_64)
/// - align: ` 16`(x86)/` 16`(x86_64)
///
/// # XML representation
/// - [`Vector4::w`] (4th) of `transition` & `scale` isn't used.
/// ```xml
/// <!--  transition: Vector4 --><!--     rotation: Quaternion      --><!--   scale: Vector4    -->
/// (0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000)
/// ```
///
/// [`Vector4::w`](Vector4)
#[repr(C, align(16))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display)]
#[display("{transition}{quaternion}{scale}")]
pub struct QsTransform {
    /// # C++ Info
    /// - name: `transition`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    ///
    /// # NOTE
    /// - `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub transition: Vector4,
    /// # C++ Info
    /// - name: `quaternion`(ctype: `hkQuaternion`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub quaternion: Quaternion,
    /// # C++ Info
    /// - name: `scale`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - `scale`: `Vector4`
    ///
    /// # NOTE
    /// - `Vector4::w`(4th) isn't used(always 0.0).
    #[display("({x:.06} {y:.06} {z:.06})")]
    pub scale: Vector4,
}

impl QsTransform {
    /// Creates a new `QsTransform`
    #[inline]
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

#[test]
fn should_write_bytes() {
    assert_eq!(
        QsTransform {
            transition: Vector4::default(),
            quaternion: Quaternion {
                x: 0.0,
                y: -0.0,
                z: -0.0,
                scaler: -1.0
            },
            scale: Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 0.0
            }
        }
        .to_le_bytes(),
        [
            0, 0, 0, 0, //
            0, 0, 0, 0, //
            0, 0, 0, 0, //
            0, 0, 0, 0, //
            //
            0, 0, 0, 0, //
            0, 0, 0, 128, //
            0, 0, 0, 128, //
            0, 0, 128, 191, //
            //
            0, 0, 128, 63, //
            0, 0, 128, 63, //
            0, 0, 128, 63, //
            0, 0, 0, 0, //
        ]
    );
}
