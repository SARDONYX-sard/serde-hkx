use crate::Vector4;

/// # Matrix4x4
///
/// # C++ Info
/// - name: `hkMatrix4`
/// - type_size: ` 64`(x86)/` 64`(x86_64)
/// - align: ` 16`(x86)/` 16`(x86_64)
///
/// # XML representation
/// ```xml
/// <hkparam>(0.000000 0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000 0.000000)(1.000000 1.000000 1.000000 0.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Matrix4 {
    /// # C++ Info
    /// - name: `x`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub x: Vector4,
    /// # C++ Info
    /// - name: `y`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub y: Vector4,
    /// # C++ Info
    /// - name: `z`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub z: Vector4,
    /// # C++ Info
    /// - name: `w`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub w: Vector4,
}

const _: () = assert!(core::mem::size_of::<Matrix4>() == 64);

impl core::fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}{}{}", self.x, self.y, self.z, self.w)
    }
}

impl Matrix4 {
    /// Creates a new `Matrix4`
    #[inline]
    pub const fn new(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self { x, y, z, w }
    }

    pub fn to_le_bytes(&self) -> [u8; 64] {
        let mut bytes = [0; 64];
        bytes[0..16].copy_from_slice(&self.x.to_le_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_le_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_le_bytes());
        bytes[48..64].copy_from_slice(&self.w.to_le_bytes());
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 64] {
        let mut bytes = [0_u8; 64];
        bytes[0..16].copy_from_slice(&self.x.to_be_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_be_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_be_bytes());
        bytes[48..64].copy_from_slice(&self.w.to_be_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_representation() {
        let matrix = Matrix4::new(
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            Vector4::new(-0.0, 0.0, -0.0, 1.0),
            Vector4::new(1.0, 1.0, 1.0, 0.0),
            Vector4::new(1.0, 1.0, 1.0, 0.0),
        );

        assert_eq!(
            matrix.to_string(),
            "(0.000000 0.000000 0.000000 0.000000)\
             (-0.000000 0.000000 -0.000000 1.000000)\
             (1.000000 1.000000 1.000000 0.000000)\
             (1.000000 1.000000 1.000000 0.000000)"
        );
    }
}
