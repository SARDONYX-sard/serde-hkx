use super::vector4::Vector4;
use derive_new::new;
use parse_display::Display;

/// Matrix4x4 for havok.
///
/// # XML representation
/// ```xml
/// <hkparam>(0.000000 0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000 0.000000)(1.000000 1.000000 1.000000 0.000000)</hkparam>
/// ```
#[repr(C, align(16))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Display, new)]
#[display("{x}{y}{z}{w}")]
pub struct Matrix4 {
    /// The first column of the matrix.
    pub x: Vector4,
    /// The second column of the matrix.
    pub y: Vector4,
    /// The third column of the matrix.
    pub z: Vector4,
    /// The fourth column of the matrix.
    pub w: Vector4,
}

static_assertions::assert_eq_size!(Matrix4, [u8; 64]);

impl Matrix4 {
    pub fn to_le_bytes(&self) -> [u8; 64] {
        let mut bytes = [0; 64];
        bytes[0..16].copy_from_slice(&self.x.to_le_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_le_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_le_bytes());
        bytes[48..64].copy_from_slice(&self.w.to_le_bytes());
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[0..16].copy_from_slice(&self.x.to_be_bytes());
        bytes[16..32].copy_from_slice(&self.y.to_be_bytes());
        bytes[32..48].copy_from_slice(&self.z.to_be_bytes());
        bytes[48..64].copy_from_slice(&self.w.to_be_bytes());
        bytes
    }

    #[inline]
    pub fn from_str(s: &str) -> crate::error::Result<(&str, Self)> {
        let res = crate::tri!(parse_matrix4(s));
        Ok(res)
    }
}

fn parse_matrix4(input: &str) -> winnow::PResult<(&str, Matrix4)> {
    use crate::parse_vector4;

    let (input, x) = parse_vector4(input)?;
    let (input, y) = parse_vector4(input)?;
    let (input, z) = parse_vector4(input)?;
    let (remain, w) = parse_vector4(input)?;
    Ok((remain, Matrix4 { x, y, z, w }))
}
