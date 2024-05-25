use derive_new::new;
use parse_display::Display;

/// # XML
/// ```xml
/// <hkparam>(0.000000 0.000000 0.000000 1.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display, new)]
#[display("({x:.06} {y:.06} {z:.06} {scaler:.06})")]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    scaler: f32,
}

impl Quaternion {
    #[inline]
    pub fn to_le_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&self.x.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.scaler.to_le_bytes());
        bytes
    }

    #[inline]
    pub fn to_be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.scaler.to_be_bytes());
        bytes
    }

    #[inline]
    pub fn from_le_bytes(bytes: &[u8; 16]) -> Self {
        let x = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let y = f32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let z = f32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let scaler = f32::from_le_bytes(bytes[12..16].try_into().unwrap());
        Self { x, y, z, scaler }
    }

    #[inline]
    pub fn from_be_bytes(bytes: &[u8; 16]) -> Self {
        let x = f32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let y = f32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let z = f32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let scaler = f32::from_be_bytes(bytes[12..16].try_into().unwrap());
        Self { x, y, z, scaler }
    }
}
