use crate::{Quaternion, Vector4};

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
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct QsTransform {
    /// # C++ Info
    /// - name: `transition`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    ///
    /// # NOTE
    /// - `Vector4::w`(4th) isn't used(always 0.0).
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
    pub scale: Vector4,
}

impl core::fmt::Display for QsTransform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "({:.06} {:.06} {:.06}){}({:.06} {:.06} {:.06})",
            self.transition.x,
            self.transition.y,
            self.transition.z,
            self.quaternion,
            self.scale.x,
            self.scale.y,
            self.scale.z,
        )
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_representation() {
        let transform = QsTransform {
            transition: Vector4::new(0.0, 0.0, 0.0, 0.0),
            quaternion: Quaternion::new(-0.0, 0.0, -0.0, 1.0),
            scale: Vector4::new(1.0, 1.0, 1.0, 0.0),
        };

        assert_eq!(
            transform.to_string(),
            "(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000)"
        );
    }
}
