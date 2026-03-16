//! Spline-compressed animation block deserializer.
//!
//! # Binary layout (one block)
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │  TransformMask × num_tracks  (4 bytes each)                 │
//! │  pad to 4                                                   │
//! │  ┌──────────────────────────────────────────────────────┐   │
//! │  │ Track i                                              │   │
//! │  │  [Position section]  pad to 4 after                  │   │
//! │  │  [Rotation section]  pad to 4 after                  │   │
//! │  │  [Scale   section]   pad to 4 after                  │   │
//! │  └──────────────────────────────────────────────────────┘   │
//! │  pad to 16                                                  │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # TransformMask (4 bytes)
//!
//! ```text
//! Byte 0 – quantization types
//!   bits [1:0]  PositionQuantizationType   (0=BITS8, 1=BITS16)
//!   bits [5:2]  RotationQuantizationType   (0=POLAR32 … 5=UNCOMPRESSED)
//!   bits [7:6]  ScaleQuantizationType      (0=BITS8, 1=BITS16)
//! Byte 1 – position  FlagOffset bitmask
//! Byte 2 – rotation  FlagOffset bitmask
//! Byte 3 – scale     FlagOffset bitmask
//! ```
//!
//! # FlagOffset bitmask
//!
//! ```text
//! bit 0  STATIC_X    bit 4  SPLINE_X
//! bit 1  STATIC_Y    bit 5  SPLINE_Y
//! bit 2  STATIC_Z    bit 6  SPLINE_Z
//! bit 3  STATIC_W    bit 7  SPLINE_W
//! ```
//!
//! # Position / Scale section
//!
//! **Spline path** (any SPLINE_* bit set):
//! ```text
//! u16   num_items          (= control-point count − 1)
//! u8    degree             (typically 3 = cubic B-spline)
//! u8[]  knots              (num_items + degree + 2 bytes)
//! pad to 4
//! for each active SPLINE axis (X, Y, Z in order):
//!     f32  bounds_min
//!     f32  bounds_max
//! quantized values interleaved per frame:
//!   for i in 0..=num_items:
//!     for each active SPLINE axis: u8 (BITS8) or u16-LE (BITS16)
//! ```
//!
//! **Static path** (no SPLINE_* bits, read directly in the caller):
//! ```text
//! f32  x  (if STATIC_X)
//! f32  y  (if STATIC_Y)
//! f32  z  (if STATIC_Z)
//! pad to 4
//! ```
//!
//! # Rotation section
//!
//! **Spline path** (any SPLINE_* bit set):
//! ```text
//! u16   num_items
//! u8    degree
//! u8[]  knots  (num_items + degree + 2 bytes)
//! pad to rotation_align (type-dependent, see table below)
//! Quaternion × (num_items + 1)  (encoding = RotationQuantizationType)
//! ```
//!
//! **Static path** (any STATIC_* bit set, no SPLINE_*):
//! ```text
//! pad to rotation_align
//! Quaternion × 1
//! ```
//!
//! # RotationQuantizationType sizes / alignments
//!
//! | Variant      | bytes | align |
//! |--------------|-------|-------|
//! | POLAR32      |   4   |   4   |
//! | THREECOMP40  |   5   |   1   |
//! | THREECOMP48  |   6   |   2   |
//! | THREECOMP24  |   3   |   1   | ← not implemented
//! | STRAIGHT16   |   2   |   2   | ← not implemented
//! | UNCOMPRESSED |  16   |   4   |
#![allow(clippy::needless_range_loop)]
use bitflags::bitflags;
use havok_types::Quaternion;
use serde_hkx::errors::readable::ReadableError;
use winnow::binary::{le_f32, le_i16, le_u16, le_u32, u8};
use winnow::combinator::fail;
use winnow::error::{StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::token::take;

type Input<'a> = &'a [u8];

// ---------------------------------------------------------------------------
// Quantization types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum ScalarQuantizationType {
    Bits8 = 0,
    Bits16 = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum RotationQuantizationType {
    /// 4 bytes, align 4
    Polar32 = 0,
    /// 5 bytes, align 1
    ThreeComp40 = 1,
    /// 6 bytes, align 2
    ThreeComp48 = 2,
    /// 3 bytes, align 1 – not implemented
    ThreeComp24 = 3,
    /// 2 bytes, align 2 – not implemented
    Straight16 = 4,
    /// 16 bytes, align 4
    Uncompressed = 5,
}

impl RotationQuantizationType {
    /// Returns the byte-alignment required before reading quantized quaternions,
    /// or `None` for unsupported variants.
    ///
    /// # Unimplemented variants (ThreeComp24 / Straight16).
    #[inline]
    pub(crate) const fn rotation_align(self) -> Option<usize> {
        Some(match self {
            Self::ThreeComp40 => 1,
            Self::ThreeComp48 => 2,
            Self::Polar32 | Self::Uncompressed => 4,
            _ => return None, // Unsupported
        })
    }
}

// ---------------------------------------------------------------------------
// FlagOffset bitmask
// ---------------------------------------------------------------------------

bitflags! {
    /// Per-axis channel flags packed into one byte.
    ///
    /// Lower nibble = static (constant) channels.
    /// Upper nibble = spline (animated) channels.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FlagOffset: u8 {
        const STATIC_X = 0b0000_0001;
        const STATIC_Y = 0b0000_0010;
        const STATIC_Z = 0b0000_0100;
        const STATIC_W = 0b0000_1000;
        const SPLINE_X = 0b0001_0000;
        const SPLINE_Y = 0b0010_0000;
        const SPLINE_Z = 0b0100_0000;
        const SPLINE_W = 0b1000_0000;
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Advance `input` forward until its address is a multiple of `align`.
///
/// Uses the *virtual address* of the slice pointer, which matches what
/// `BinaryReaderEx.Pad()` does in the C# reference implementation.
#[inline]
fn pad_to_abs(input: &mut Input, align: usize) {
    let addr = input.as_ptr() as usize;
    let mis = addr % align;
    if mis != 0 {
        *input = &input[align - mis..];
    }
}

fn read_quantized_float(
    input: &mut Input,
    min: f32,
    max: f32,
    qt: ScalarQuantizationType,
) -> ModalResult<f32> {
    let ratio = match qt {
        ScalarQuantizationType::Bits8 => u8.parse_next(input)? as f32 / 255.0,
        ScalarQuantizationType::Bits16 => le_u16.parse_next(input)? as f32 / 65535.0,
    };
    Ok((max - min).mul_add(ratio, min))
}

// ---------------------------------------------------------------------------
// TransformMask
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct TransformMask {
    pub pos_q: ScalarQuantizationType,
    pub rot_q: RotationQuantizationType,
    pub scale_q: ScalarQuantizationType,
    pub pos_flags: FlagOffset,
    pub rot_flags: FlagOffset,
    pub scale_flags: FlagOffset,
}

fn parse_transform_mask(input: &mut Input) -> ModalResult<TransformMask> {
    let qt = u8
        .context(StrContext::Expected(StrContextValue::Description(
            "TransformMask byte 0: quantization types (pos[1:0] rot[5:2] scale[7:6])",
        )))
        .parse_next(input)?;
    let pos_byte = u8
        .context(StrContext::Expected(StrContextValue::Description(
            "TransformMask byte 1: position FlagOffset bitmask",
        )))
        .parse_next(input)?;
    let rot_byte = u8
        .context(StrContext::Expected(StrContextValue::Description(
            "TransformMask byte 2: rotation FlagOffset bitmask",
        )))
        .parse_next(input)?;
    let scale_byte = u8
        .context(StrContext::Expected(StrContextValue::Description(
            "TransformMask byte 3: scale FlagOffset bitmask",
        )))
        .parse_next(input)?;

    Ok(TransformMask {
        pos_q: match qt & 0x3 {
            0 => ScalarQuantizationType::Bits8,
            _ => ScalarQuantizationType::Bits16,
        },
        rot_q: match (qt >> 2) & 0xF {
            0 => RotationQuantizationType::Polar32,
            1 => RotationQuantizationType::ThreeComp40,
            2 => RotationQuantizationType::ThreeComp48,
            3 => RotationQuantizationType::ThreeComp24,
            4 => RotationQuantizationType::Straight16,
            _ => RotationQuantizationType::Uncompressed,
        },
        scale_q: match (qt >> 6) & 0x3 {
            0 => ScalarQuantizationType::Bits8,
            _ => ScalarQuantizationType::Bits16,
        },
        pos_flags: FlagOffset::from_bits_truncate(pos_byte),
        rot_flags: FlagOffset::from_bits_truncate(rot_byte),
        scale_flags: FlagOffset::from_bits_truncate(scale_byte),
    })
}

// ---------------------------------------------------------------------------
// Quaternion decoders
// ---------------------------------------------------------------------------

/// Decode a POLAR32-packed quaternion.
///
/// ```text
/// Bit layout of the 32-bit value (LSB first):
///   [17: 0]  phiTheta  – encodes polar angle pair
///   [27:18]  R         – reinterpreted as IEEE-754 float bits, then scaled
///   [28]     sign X
///   [29]     sign Y
///   [30]     sign Z
///   [31]     sign W
/// ```
fn read_quat_polar32(input: &mut Input) -> ModalResult<Quaternion> {
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_4};

    let c = le_u32
        .context(StrContext::Expected(StrContextValue::Description(
            "POLAR32 quaternion: u32 packed value",
        )))
        .parse_next(input)?;

    // The 10-bit R field is reinterpreted as IEEE-754 float bits before scaling.
    // Matches C#: `CastToFloat((cVal >> 18) & rMask) * rFrac`
    const R_MASK: u32 = (1 << 10) - 1; // 0x3FF
    const R_FRAC: f32 = 1.0 / R_MASK as f32;
    let r_raw = f32::from_bits((c >> 18) & R_MASK) * R_FRAC;
    let mut w = 1.0 - r_raw * r_raw;

    let phi_theta = (c & 0x0003_FFFF) as f32;
    // phi_int is the integer floor of sqrt(phiTheta); used in theta before
    // being converted to radians.
    let phi_int = phi_theta.sqrt().floor();
    let (theta, phi_rad) = if phi_int > 0.0 {
        let t = FRAC_PI_4 * (phi_theta - phi_int * phi_int) / phi_int;
        (t, phi_int * (FRAC_PI_2 / 511.0))
    } else {
        (0.0_f32, 0.0_f32)
    };

    let mag = (1.0 - w * w).sqrt();
    let mut x = phi_rad.sin() * theta.cos() * mag;
    let mut y = phi_rad.sin() * theta.sin() * mag;
    let mut z = phi_rad.cos() * mag;

    if c & 0x1000_0000 != 0 {
        x = -x;
    }
    if c & 0x2000_0000 != 0 {
        y = -y;
    }
    if c & 0x4000_0000 != 0 {
        z = -z;
    }
    if c & 0x8000_0000 != 0 {
        w = -w;
    }

    Ok(Quaternion::new(x, y, z, w))
}

/// Decode a THREECOMP48-packed quaternion (3 × i16).
///
/// ```text
/// x[15]      – bit 0 of result_shift
/// y[15:14]   – bits 1:0 of result_shift (bit 1 from y[14])
/// z[15]      – sign of the reconstructed component
/// x[14:0], y[14:0], z[14:0] – three packed components
///   bias = 0b0011_1111_1111_1111 >> 1  (= 0x3FFF)
/// The largest-magnitude component is omitted; its slot index = result_shift.
/// ```
fn read_quat_three_comp48(input: &mut Input) -> ModalResult<Quaternion> {
    // (1 << 15) - 1 overflows i16, so write the bit pattern directly.
    const MASK: i16 = 0b0111_1111_1111_1111; // 0x7FFF
    const FRACTAL: f32 = 0.000_043_161;

    let raw_x = le_i16
        .context(StrContext::Expected(StrContextValue::Description(
            "THREECOMP48 i16 x",
        )))
        .parse_next(input)?;
    let raw_y = le_i16
        .context(StrContext::Expected(StrContextValue::Description(
            "THREECOMP48 i16 y",
        )))
        .parse_next(input)?;
    let raw_z = le_i16
        .context(StrContext::Expected(StrContextValue::Description(
            "THREECOMP48 i16 z",
        )))
        .parse_next(input)?;

    let result_shift = (((raw_y >> 14) & 0x2) | ((raw_x >> 15) & 0x1)) as usize;
    let r_sign = (raw_z >> 15) != 0;

    let fx = ((raw_x & MASK) - (MASK >> 1)) as f32 * FRACTAL;
    let fy = ((raw_y & MASK) - (MASK >> 1)) as f32 * FRACTAL;
    let fz = ((raw_z & MASK) - (MASK >> 1)) as f32 * FRACTAL;

    let mut out = scatter3_to4(fx, fy, fz, result_shift);
    let mut w = (1.0 - (fx * fx + fy * fy + fz * fz)).max(0.0).sqrt();
    if r_sign {
        w = -w;
    }
    out[result_shift] = w;

    Ok(Quaternion::new(out[0], out[1], out[2], out[3]))
}

/// Decode a THREECOMP40-packed quaternion (5 bytes).
///
/// ```text
/// bits [11: 0]  x component (12-bit unsigned, bias = 0x7FF)
/// bits [23:12]  y component
/// bits [35:24]  z component
/// bits [37:36]  result_shift (index of the reconstructed component)
/// bit  [38]     sign of the reconstructed component
/// ```
fn read_quat_three_comp40(input: &mut Input) -> ModalResult<Quaternion> {
    const MASK: u64 = (1 << 12) - 1; // 0xFFF
    const BIAS: i32 = (MASK >> 1) as i32; // 2047
    const FRACTAL: f32 = 0.000_345_436;

    let bytes = take(5_usize)
        .context(StrContext::Expected(StrContextValue::Description(
            "THREECOMP40 quaternion: 5 packed bytes",
        )))
        .parse_next(input)?;
    let mut buf = [0_u8; 8];
    buf[..5].copy_from_slice(bytes);
    let c = u64::from_le_bytes(buf);

    let fx = ((c & MASK) as i32 - BIAS) as f32 * FRACTAL;
    let fy = (((c >> 12) & MASK) as i32 - BIAS) as f32 * FRACTAL;
    let fz = (((c >> 24) & MASK) as i32 - BIAS) as f32 * FRACTAL;
    let result_shift = ((c >> 36) & 0x3) as usize;
    let r_sign = ((c >> 38) & 1) != 0;

    let mut out = scatter3_to4(fx, fy, fz, result_shift);
    let mut w = (1.0 - (fx.mul_add(fx, fy * fy) + fz * fz)).max(0.0).sqrt();
    if r_sign {
        w = -w;
    }
    out[result_shift] = w;

    Ok(Quaternion::new(out[0], out[1], out[2], out[3]))
}

/// Scatter three float values into a 4-element array, skipping index `skip`.
///
/// Used by THREECOMP40 / THREECOMP48: the three decoded values are placed into
/// their correct quaternion slots; the omitted component slot (= `skip`) is
/// left at `0.0` and filled in by the caller after reconstruction.
#[inline]
fn scatter3_to4(a: f32, b: f32, c: f32, skip: usize) -> [f32; 4] {
    let src = [a, b, c];
    let mut out = [0.0_f32; 4];
    let mut si = 0;
    for i in 0..4 {
        if i != skip {
            out[i] = src[si];
            si += 1;
        }
    }
    out
}

fn read_quantized_quat(input: &mut Input, qt: RotationQuantizationType) -> ModalResult<Quaternion> {
    match qt {
        RotationQuantizationType::Polar32 => read_quat_polar32(input),
        RotationQuantizationType::ThreeComp40 => read_quat_three_comp40(input),
        RotationQuantizationType::ThreeComp48 => read_quat_three_comp48(input),
        RotationQuantizationType::Uncompressed => {
            let x = le_f32
                .context(StrContext::Expected(StrContextValue::Description(
                    "Uncompressed quat x",
                )))
                .parse_next(input)?;
            let y = le_f32
                .context(StrContext::Expected(StrContextValue::Description(
                    "Uncompressed quat y",
                )))
                .parse_next(input)?;
            let z = le_f32
                .context(StrContext::Expected(StrContextValue::Description(
                    "Uncompressed quat z",
                )))
                .parse_next(input)?;
            let w = le_f32
                .context(StrContext::Expected(StrContextValue::Description(
                    "Uncompressed quat w",
                )))
                .parse_next(input)?;
            Ok(Quaternion::new(x, y, z, w))
        }
        RotationQuantizationType::ThreeComp24 | RotationQuantizationType::Straight16 => fail
            .context(StrContext::Expected(StrContextValue::Description(
                "unsupported RotationQuantizationType (ThreeComp24 / Straight16)",
            )))
            .parse_next(input),
    }
}

// ---------------------------------------------------------------------------
// Spline Vec3 track
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct SplineChannel<T> {
    pub values: Vec<T>,
}

/// Decoded position or scale channel for one track.
///
/// - **Spline**: `degree` and `knots` are populated; each axis channel holds
///   `num_items + 1` control-point values.
/// - **Static**: `degree == 0`, `knots` is empty, each present axis channel
///   holds exactly one value.
#[derive(Debug)]
pub struct Vec3Track<'a> {
    pub degree: u8,
    pub knots: &'a [u8],
    pub x: Option<SplineChannel<f32>>,
    pub y: Option<SplineChannel<f32>>,
    pub z: Option<SplineChannel<f32>>,
}

/// Parse a spline Vec3 section.
///
/// Only called when at least one SPLINE_* axis is active.
/// Static axes do **not** appear inside this section in the binary format;
/// they are handled by the caller via the static path.
fn parse_spline_vec3<'a>(
    input: &mut Input<'a>,
    flags: FlagOffset,
    qt: ScalarQuantizationType,
) -> ModalResult<Vec3Track<'a>> {
    let num = le_u16
        .context(StrContext::Expected(StrContextValue::Description(
            "SplineVec3 num_items (u16, = control_points − 1)",
        )))
        .parse_next(input)? as usize;
    let degree = u8
        .context(StrContext::Expected(StrContextValue::Description(
            "SplineVec3 degree (u8, typically 3 for cubic B-spline)",
        )))
        .parse_next(input)?;
    let knot_count = num + degree as usize + 2;
    let knots = take(knot_count)
        .context(StrContext::Expected(StrContextValue::Description(
            "SplineVec3 knot vector (num_items + degree + 2 bytes)",
        )))
        .parse_next(input)?;

    // Bounds and quantized values follow after 4-byte padding.
    pad_to_abs(input, 4);

    let mut bx = (0.0_f32, 0.0_f32);
    let mut by = (0.0_f32, 0.0_f32);
    let mut bz = (0.0_f32, 0.0_f32);
    let mut cx: Option<SplineChannel<f32>> = None;
    let mut cy: Option<SplineChannel<f32>> = None;
    let mut cz: Option<SplineChannel<f32>> = None;

    if flags.contains(FlagOffset::SPLINE_X) {
        bx.0 = le_f32
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineX bounds_min",
            )))
            .parse_next(input)?;
        bx.1 = le_f32
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineX bounds_max",
            )))
            .parse_next(input)?;
        cx = Some(SplineChannel {
            values: Vec::with_capacity(num + 1),
        });
    }
    if flags.contains(FlagOffset::SPLINE_Y) {
        by.0 = le_f32
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineY bounds_min",
            )))
            .parse_next(input)?;
        by.1 = le_f32
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineY bounds_max",
            )))
            .parse_next(input)?;
        cy = Some(SplineChannel {
            values: Vec::with_capacity(num + 1),
        });
    }
    if flags.contains(FlagOffset::SPLINE_Z) {
        bz.0 = le_f32
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineZ bounds_min",
            )))
            .parse_next(input)?;
        bz.1 = le_f32
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineZ bounds_max",
            )))
            .parse_next(input)?;
        cz = Some(SplineChannel {
            values: Vec::with_capacity(num + 1),
        });
    }

    for _ in 0..=num {
        if let Some(c) = cx.as_mut() {
            c.values.push(read_quantized_float(input, bx.0, bx.1, qt)?);
        }
        if let Some(c) = cy.as_mut() {
            c.values.push(read_quantized_float(input, by.0, by.1, qt)?);
        }
        if let Some(c) = cz.as_mut() {
            c.values.push(read_quantized_float(input, bz.0, bz.1, qt)?);
        }
    }

    Ok(Vec3Track {
        degree,
        knots,
        x: cx,
        y: cy,
        z: cz,
    })
}

#[derive(Debug)]
pub struct QuatTrack<'a> {
    pub degree: u8,
    pub knots: &'a [u8],
    /// Spline: `num_items + 1` quaternions.  Static: exactly 1 quaternion.
    pub values: Vec<Quaternion>,
}

fn parse_quat_section<'a>(
    input: &mut Input<'a>,
    flags: FlagOffset,
    qt: RotationQuantizationType,
) -> ModalResult<Option<QuatTrack<'a>>> {
    if flags.intersects(
        FlagOffset::SPLINE_X | FlagOffset::SPLINE_Y | FlagOffset::SPLINE_Z | FlagOffset::SPLINE_W,
    ) {
        let num_items = le_u16
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineRotation num_items (u16)",
            )))
            .parse_next(input)? as usize;
        let degree = u8
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineRotation degree (u8)",
            )))
            .parse_next(input)?;
        let knot_count = num_items + degree as usize + 2;
        let knots = take(knot_count)
            .context(StrContext::Expected(StrContextValue::Description(
                "SplineRotation knot vector (num_items + degree + 2 bytes)",
            )))
            .parse_next(input)?;

        let Some(align) = qt.rotation_align() else {
            return fail
                .context(StrContext::Expected(StrContextValue::Description(
                    "SplineRotation: unsupported RotationQuantizationType \
                     (ThreeComp24 / Straight16 not implemented)",
                )))
                .parse_next(input);
        };
        pad_to_abs(input, align);

        let mut values = Vec::with_capacity(num_items + 1);
        for _ in 0..=num_items {
            values.push(read_quantized_quat(input, qt)?);
        }

        Ok(Some(QuatTrack {
            degree,
            knots,
            values,
        }))
    } else if flags.intersects(
        FlagOffset::STATIC_X | FlagOffset::STATIC_Y | FlagOffset::STATIC_Z | FlagOffset::STATIC_W,
    ) {
        let Some(align) = qt.rotation_align() else {
            return fail
                .context(StrContext::Expected(StrContextValue::Description(
                    "StaticRotation: unsupported RotationQuantizationType \
                     (ThreeComp24 / Straight16 not implemented)",
                )))
                .parse_next(input);
        };
        pad_to_abs(input, align);

        Ok(Some(QuatTrack {
            degree: 0,
            knots: &[],
            values: vec![read_quantized_quat(input, qt)?],
        }))
    } else {
        Ok(None)
    }
}

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct TransformTrack<'a> {
    pub position: Option<Vec3Track<'a>>,
    pub rotation: Option<QuatTrack<'a>>,
    pub scale: Option<Vec3Track<'a>>,
}

// ---------------------------------------------------------------------------
// SplineParser
// ---------------------------------------------------------------------------

/// Stateful parser that keeps the current byte offset for error reporting.
struct SplineParser<'a> {
    /// Pointer to the very first byte of the data buffer (never advanced).
    full: &'a [u8],
    /// Remaining input (advanced as bytes are consumed).
    input: &'a [u8],
    /// Most recently started block index (for diagnostics).
    current_block: usize,
    /// Most recently started track index within the block (for diagnostics).
    current_track: usize,
}

impl<'a> SplineParser<'a> {
    const fn new(data: &'a [u8]) -> Self {
        Self {
            full: data,
            input: data,
            current_block: 0,
            current_track: 0,
        }
    }

    fn current_position(&self) -> usize {
        self.input.as_ptr() as usize - self.full.as_ptr() as usize
    }

    fn pad_to(&mut self, align: usize) {
        let addr = self.input.as_ptr() as usize;
        let mis = addr % align;
        if mis != 0 {
            let skip = align - mis;
            // Guard against reading past the end of the buffer.
            // This can happen at the very end of the last block.
            if skip <= self.input.len() {
                self.input = &self.input[skip..];
            } else {
                self.input = &self.input[self.input.len()..];
            }
        }
    }

    /// Parse one position or scale section.
    ///
    /// - Any SPLINE_* bit set → full spline header + control points.
    /// - No SPLINE_* but any STATIC_* set → zero to three raw `f32` values.
    /// - No bits set → `None`.
    fn parse_vec3_section(
        &mut self,
        flags: FlagOffset,
        qt: ScalarQuantizationType,
    ) -> ModalResult<Option<Vec3Track<'a>>> {
        if flags.intersects(FlagOffset::SPLINE_X | FlagOffset::SPLINE_Y | FlagOffset::SPLINE_Z) {
            return Ok(Some(parse_spline_vec3(&mut self.input, flags, qt)?));
        }

        if !flags.intersects(FlagOffset::STATIC_X | FlagOffset::STATIC_Y | FlagOffset::STATIC_Z) {
            return Ok(None);
        }

        // Static path: individual f32 per active axis, no spline header.
        let x = if flags.contains(FlagOffset::STATIC_X) {
            Some(SplineChannel {
                values: vec![
                    le_f32
                        .context(StrContext::Expected(StrContextValue::Description(
                            "Static axis x (f32)",
                        )))
                        .parse_next(&mut self.input)?,
                ],
            })
        } else {
            None
        };
        let y = if flags.contains(FlagOffset::STATIC_Y) {
            Some(SplineChannel {
                values: vec![
                    le_f32
                        .context(StrContext::Expected(StrContextValue::Description(
                            "Static axis y (f32)",
                        )))
                        .parse_next(&mut self.input)?,
                ],
            })
        } else {
            None
        };
        let z = if flags.contains(FlagOffset::STATIC_Z) {
            Some(SplineChannel {
                values: vec![
                    le_f32
                        .context(StrContext::Expected(StrContextValue::Description(
                            "Static axis z (f32)",
                        )))
                        .parse_next(&mut self.input)?,
                ],
            })
        } else {
            None
        };

        Ok(Some(Vec3Track {
            degree: 0,
            knots: &[],
            x,
            y,
            z,
        }))
    }

    fn parse_block(&mut self, num_tracks: usize) -> ModalResult<Vec<TransformTrack<'a>>> {
        // Read all TransformMasks upfront (4 bytes each), then pad to 4.
        let mut masks = Vec::with_capacity(num_tracks);
        for _ in 0..num_tracks {
            masks.push(parse_transform_mask(&mut self.input)?);
        }
        self.pad_to(4);

        let mut tracks = Vec::with_capacity(num_tracks);

        for (track_idx, m) in masks.into_iter().enumerate() {
            self.current_track = track_idx;

            let position = self.parse_vec3_section(m.pos_flags, m.pos_q)?;
            self.pad_to(4);

            let rotation = parse_quat_section(&mut self.input, m.rot_flags, m.rot_q)?;
            self.pad_to(4);

            let scale = self.parse_vec3_section(m.scale_flags, m.scale_q)?;
            self.pad_to(4);

            tracks.push(TransformTrack {
                position,
                rotation,
                scale,
            });
        }

        self.pad_to(16);
        Ok(tracks)
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn read_spline_compressed_blocks(
    data: &[u8],
    num_tracks: usize,
    num_blocks: usize,
) -> Result<Vec<Vec<TransformTrack<'_>>>, ReadableError> {
    let mut parser = SplineParser::new(data);
    let mut blocks = Vec::with_capacity(num_blocks);

    for block_idx in 0..num_blocks {
        parser.current_block = block_idx;
        let block = parser.parse_block(num_tracks).map_err(|e| {
            let input_hex = serde_hkx::bytes::hexdump::to_string(parser.full);
            let err_pos = serde_hkx::bytes::hexdump::to_hexdump_pos(parser.current_position());
            ReadableError::from_context(e, input_hex, err_pos)
        })?;
        blocks.push(block);
    }

    Ok(blocks)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke-test against a captured 8 384-byte spline block from
    /// `MCO_Dodge-B-1.hkx` (97 transform tracks, 1 block).
    #[test]
    #[ignore = "requires external test data file"]
    fn decode_single_block() {
        let bytes =
            std::fs::read("../../tests/data_8384bytes.bin").expect("failed to read test data");
        let re = read_spline_compressed_blocks(&bytes, 97, 1).unwrap_or_else(|e| panic!("{e}"));

        assert_eq!(re.len(), 1);
        assert_eq!(re[0].len(), 97);
        std::fs::write("./logs/data.debug.log", format!("{re:#?}")).unwrap();
    }
}
