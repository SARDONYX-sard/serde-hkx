use parse_display::Display;

/// # Quaternion
///
/// # C++ Info
/// - name: `hkQuaternion`
/// - type_size: ` 16`(x86)/` 16`(x86_64)
///
/// # XML representation
/// ```xml
/// <hkparam>(0.000000 0.000000 0.000000 1.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display)]
#[display("({x:.06} {y:.06} {z:.06} {scaler:.06})")]
pub struct Quaternion {
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
    /// - name: `scaler`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: ` 4`(x86)/` 4`(x86_64)
    pub scaler: f32,
}

static_assertions::assert_eq_size!(Quaternion, [u8; 16]);

impl Quaternion {
    /// Creates a new `Quaternion`
    pub const fn new(x: f32, y: f32, z: f32, scaler: f32) -> Self {
        Self { x, y, z, scaler }
    }

    #[inline]
    pub fn to_le_bytes(&self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[0..4].copy_from_slice(&self.x.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.scaler.to_le_bytes());
        bytes
    }

    #[inline]
    pub fn to_be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[0..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.scaler.to_be_bytes());
        bytes
    }

    /// Create a [`Quaternion`] value from its representation as a byte array in little endian.
    #[inline]
    pub const fn from_le_bytes(bytes: &[u8; 16]) -> Self {
        Self {
            x: f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            y: f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            z: f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            scaler: f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }

    /// Create a [`Quaternion`] value from its representation as a byte array in big endian.
    #[inline]
    pub const fn from_be_bytes(bytes: &[u8; 16]) -> Self {
        Self {
            x: f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            y: f32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            z: f32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            scaler: f32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }
}
