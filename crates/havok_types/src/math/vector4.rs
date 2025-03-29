use parse_display::Display;

/// # Vector4
///
/// # C++ Info
/// - name: `hkVector4`
/// - type_size: ` 16`(x86)/` 16`(x86_64)
///
/// # Examples
/// ```
/// use havok_types::Vector4;
///
/// assert_eq!(Vector4::new(1.0, 1.0, 1.0, 0.0).to_string(), "(1.000000 1.000000 1.000000 0.000000)");
/// assert_eq!(Vector4::new(-0.0, 0.0, -0.0, 1.0).to_string(), "(-0.000000 0.000000 -0.000000 1.000000)");
/// ```
///
/// # XML representation
/// ```xml
/// <hkparam name="">
///   <!-- x         y        z         w -->
///   (-0.000000 0.000000 -0.000000 1.000000)
/// </hkparam>
/// ```
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C, align(16))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display)]
#[display("({x:.06} {y:.06} {z:.06} {w:.06})")]
pub struct Vector4 {
    /// # C++ Info
    /// - name: `x`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 4`(x86)/` 4`(x86_64)
    pub x: f32,
    /// # C++ Info
    /// - name: `y`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: ` 4`(x86)/` 4`(x86_64)
    pub y: f32,
    /// # C++ Info
    /// - name: `z`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: ` 4`(x86)/` 4`(x86_64)
    pub z: f32,
    /// # C++ Info
    /// - name: `w`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: ` 4`(x86)/` 4`(x86_64)
    pub w: f32,
}

static_assertions::assert_eq_size!(Vector4, [u8; 16]);

impl Vector4 {
    /// Creates a new `Vector4`
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// As a byte array in little endian.
    #[inline]
    pub fn to_le_bytes(&self) -> [u8; 16] {
        let mut bytes = [0_u8; 16];
        bytes[0..4].copy_from_slice(&self.x.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.w.to_le_bytes());
        bytes
    }

    /// As a byte array in big endian.
    #[inline]
    pub fn to_be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0_u8; 16];
        bytes[0..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.w.to_be_bytes());
        bytes
    }

    /// Create a [`Vector4`] value from its representation as a byte array in little endian.
    #[inline]
    pub const fn from_le_bytes(bytes: &[u8; 16]) -> Self {
        Self {
            x: f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            y: f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            z: f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            w: f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }

    /// Create a [`Vector4`] value from its representation as a byte array in big endian.
    #[inline]
    pub const fn from_be_bytes(bytes: &[u8; 16]) -> Self {
        Self {
            x: f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            y: f32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            z: f32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            w: f32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }
}

static_assertions::assert_eq_size!(Vector4, [u8; 16]); // Vector4 must be 16bytes size.
static_assertions::assert_eq_align!(Vector4, u128); // Vector4 must be 16bytes(16 * 8 = 128bit) align.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[rustfmt::skip]
        let expected_bytes: &[u8; 16] = &[
            0, 0, 128, 63, // 1.0 in bytes (little-endian): 0x3F800000 → [0x00, 0x00, 0x80, 0x3F]
            0, 0, 128, 63, // 1.0 in bytes (little-endian): 0x3F800000 → [0x00, 0x00, 0x80, 0x3F]
            0, 0, 128, 63, // 1.0 in bytes (little-endian): 0x3F800000 → [0x00, 0x00, 0x80, 0x3F]
            0, 0, 0, 0,    // 0.0 in bytes (little-endian): 0x00000000 → [0x00, 0x00, 0x00, 0x00]
        ];

        assert_eq!(
            &Vector4::new(1.0, 1.0, 1.0, 0.0).to_le_bytes(),
            expected_bytes,
        );
    }
}
