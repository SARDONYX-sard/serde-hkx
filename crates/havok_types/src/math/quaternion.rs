use crate::error::Result;
use crate::{parse_vector4, Vector4};
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
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub scaler: f32,
}

static_assertions::assert_eq_size!(Quaternion, [u8; 16]);

impl Quaternion {
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
    pub fn from_le_bytes(bytes: &[u8; 16]) -> Self {
        Self {
            x: f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            y: f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            z: f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            scaler: f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }

    /// Create a [`Quaternion`] value from its representation as a byte array in big endian.
    #[inline]
    pub fn from_be_bytes(bytes: &[u8; 16]) -> Self {
        Self {
            x: f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            y: f32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            z: f32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            scaler: f32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }

    /// Attempt to extract from the string representation in the tag on XML.
    ///
    /// # Examples
    /// ```
    /// use havok_types::Quaternion;
    /// let (remain, actual) = Quaternion::from_str("   (   -0.000000 0.000000 -0.000000 1.000000  ) ").unwrap();
    /// assert_eq!(remain, "");
    /// assert_eq!(actual, Quaternion::new(-0.0, 0.0, -0.0, 1.0));
    /// ```
    #[inline]
    pub fn from_str(s: &str) -> Result<(&str, Self)> {
        let (remain, quaternion) = crate::tri!(parse_quaternion(s));
        Ok((remain, quaternion))
    }
}

#[inline]
pub fn parse_quaternion(s: &str) -> winnow::PResult<(&str, Quaternion)> {
    let (remain, Vector4 { x, y, z, w }) = parse_vector4(s)?;
    Ok((remain, Quaternion { x, y, z, scaler: w }))
}
