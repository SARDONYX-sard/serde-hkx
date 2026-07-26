use crate::{Rotation, Vector4};

/// # Transform
///
/// # C++ Info
/// - name: `hkTransform`
/// - type_size: ` 64`(x86)/` 64`(x86_64)
/// - align: ` 16`(x86)/` 16`(x86_64)
///
/// # XML representation
/// - [`Vector4::w`] (4th) isn't used.
/// ```xml
///          <!--                             Matrix3 rotation                                --><!--   Vector4 transition  -->
/// <hkparam>(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Transform {
    /// # C++ Info
    /// - name: `rotation`(ctype: `hkRotation`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub rotation: Rotation,
    /// # C++ Info
    /// - name: `transition`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    ///
    /// # NOTE
    /// - `Vector4::w`(4th) isn't used(always 0.0).
    pub transition: Vector4,
}

const _: () = assert!(core::mem::size_of::<Transform>() == 64);

impl core::fmt::Display for Transform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}({:.06} {:.06} {:.06})",
            self.rotation, self.transition.x, self.transition.y, self.transition.z,
        )
    }
}

impl Transform {
    /// Create a new `Transform`
    #[inline]
    pub const fn new(rotation: Rotation, transition: Vector4) -> Self {
        Self {
            rotation,
            transition,
        }
    }

    pub fn to_le_bytes(&self) -> [u8; 64] {
        let mut bytes = [0_u8; 64];
        bytes[0..48].copy_from_slice(&self.rotation.to_le_bytes());
        bytes[48..64].copy_from_slice(&self.transition.to_le_bytes());
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 64] {
        let mut bytes = [0_u8; 64];
        bytes[0..48].copy_from_slice(&self.rotation.to_be_bytes());
        bytes[48..64].copy_from_slice(&self.transition.to_be_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_representation() {
        let transform = Transform {
            rotation: Rotation {
                x: Vector4::new(0.0, 0.0, 0.0, 0.0),
                y: Vector4::new(0.0, 0.0, 0.0, 0.0),
                z: Vector4::new(0.0, 0.0, 0.0, 0.0),
            },
            transition: Vector4::new(-0.0, 0.0, -0.0, 0.0),
        };

        assert_eq!(
            transform.to_string(),
            "(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)"
        );
    }
}
