//! Spline-compressed animation block serializer.
//!
//! Inverse of `de_spline`. Writes one block per call to [`write_block`].
//!
//! # Strategy
//!
//! Each frame's sampled value is used directly as a B-spline control point
//! (degree = 1, i.e. linear interpolation). The knot vector is the standard
//! clamped uniform knot vector for degree-1 splines:
//!
//! ```text
//! knots = [0, 0, 1, 2, ..., n-1, n, n]   (num_items + degree + 2 entries)
//! ```
//!
//! If all frames for an axis are identical the axis is written as Static.
//! If the axis is absent from the skeleton it is omitted entirely.
#![allow(clippy::needless_range_loop)]
use havok_classes::hkaSplineCompressedAnimationTrackCompressionParams;
use havok_types::{QsTransform, Quaternion};

use crate::kf::to_kf::de_spline::{FlagOffset, RotationQuantizationType, ScalarQuantizationType};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Serialized output for one animation block.
pub struct BlockBytes {
    pub data: Vec<u8>,
    /// Byte offset of the transform-track data within `data`
    /// (i.e. after the mask+quantization section).
    pub transform_offsets: Vec<u32>,
    /// Byte offset of the float-track data within `data`.
    pub float_block_offset: u32,
}

/// Write one spline-compressed block from a flat transform array.
///
/// `transforms[track + frame * n_transforms]` = transform for that track/frame.
pub fn write_block(
    transforms: &[&QsTransform],
    n_transforms: usize,
    n_frames: usize,
    params: &hkaSplineCompressedAnimationTrackCompressionParams,
) -> BlockBytes {
    let rot_qt = match params.m_rotationQuantizationType {
        havok_classes::RotationQuantization::THREECOMP48 => RotationQuantizationType::ThreeComp48,
        havok_classes::RotationQuantization::POLAR32 => RotationQuantizationType::Polar32,
        _ => RotationQuantizationType::ThreeComp40,
    };
    let pos_qt = ScalarQuantizationType::Bits8;
    let scale_qt = ScalarQuantizationType::Bits8;

    // -----------------------------------------------------------------------
    // Pass 1 – compute masks for all tracks
    // -----------------------------------------------------------------------
    let masks: Vec<TrackMask> = (0..n_transforms)
        .map(|ti| {
            compute_mask(
                transforms,
                ti,
                n_transforms,
                n_frames,
                pos_qt,
                rot_qt,
                scale_qt,
            )
        })
        .collect();

    let mut data: Vec<u8> = Vec::new();

    // -----------------------------------------------------------------------
    // Write TransformMasks (4 bytes each) + pad to 4
    // -----------------------------------------------------------------------
    for m in &masks {
        write_transform_mask(&mut data, m);
    }
    pad_to_4(&mut data);

    // -----------------------------------------------------------------------
    // Pass 2 – write per-track data, recording offsets
    // -----------------------------------------------------------------------
    let block_base = data.len() as u32;
    let mut transform_offsets = Vec::with_capacity(n_transforms);

    for (ti, m) in masks.iter().enumerate() {
        transform_offsets.push(data.len() as u32 - block_base);

        // --- Position ---
        write_vec3_section(
            &mut data,
            &collect_positions(transforms, ti, n_transforms, n_frames),
            m.pos_flags,
            m.pos_qt,
        );
        pad_to_4(&mut data);

        // --- Rotation ---
        write_quat_section(
            &mut data,
            &collect_rotations(transforms, ti, n_transforms, n_frames),
            m.rot_flags,
            m.rot_qt,
        );
        pad_to_4(&mut data);

        // --- Scale ---
        write_vec3_section(
            &mut data,
            &collect_scales(transforms, ti, n_transforms, n_frames),
            m.scale_flags,
            m.scale_qt,
        );
        pad_to_4(&mut data);
    }

    let float_block_offset = data.len() as u32;

    // Pad block to 16-byte boundary
    while data.len() % 16 != 0 {
        data.push(0);
    }

    BlockBytes {
        data,
        transform_offsets,
        float_block_offset,
    }
}

// ---------------------------------------------------------------------------
// Mask computation
// ---------------------------------------------------------------------------

struct TrackMask {
    pos_flags: FlagOffset,
    rot_flags: FlagOffset,
    scale_flags: FlagOffset,
    pos_qt: ScalarQuantizationType,
    rot_qt: RotationQuantizationType,
    scale_qt: ScalarQuantizationType,
}

fn compute_mask(
    transforms: &[&QsTransform],
    ti: usize,
    n_transforms: usize,
    n_frames: usize,
    pos_qt: ScalarQuantizationType,
    rot_qt: RotationQuantizationType,
    scale_qt: ScalarQuantizationType,
) -> TrackMask {
    let positions = collect_positions(transforms, ti, n_transforms, n_frames);
    let rotations = collect_rotations(transforms, ti, n_transforms, n_frames);
    let scales = collect_scales(transforms, ti, n_transforms, n_frames);

    TrackMask {
        pos_flags: vec3_flags(&positions),
        rot_flags: quat_flags(&rotations),
        scale_flags: vec3_flags(&scales),
        pos_qt,
        rot_qt,
        scale_qt,
    }
}

/// Collect position vectors for one track across all frames.
fn collect_positions(
    transforms: &[&QsTransform],
    ti: usize,
    n_transforms: usize,
    n_frames: usize,
) -> Vec<[f32; 3]> {
    (0..n_frames)
        .map(|fi| {
            let t = &transforms[ti + fi * n_transforms];
            [t.transition.x, t.transition.y, t.transition.z]
        })
        .collect()
}

fn collect_rotations(
    transforms: &[&QsTransform],
    ti: usize,
    n_transforms: usize,
    n_frames: usize,
) -> Vec<Quaternion> {
    (0..n_frames)
        .map(|fi| transforms[ti + fi * n_transforms].quaternion.clone())
        .collect()
}

fn collect_scales(
    transforms: &[&QsTransform],
    ti: usize,
    n_transforms: usize,
    n_frames: usize,
) -> Vec<[f32; 3]> {
    (0..n_frames)
        .map(|fi| {
            let t = &transforms[ti + fi * n_transforms];
            [t.scale.x, t.scale.y, t.scale.z]
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Flag computation
// ---------------------------------------------------------------------------

const FLOAT_EPS: f32 = 1e-6;

/// Determine position/scale FlagOffset by checking per-axis variance.
fn vec3_flags(frames: &[[f32; 3]]) -> FlagOffset {
    if frames.is_empty() {
        return FlagOffset::empty();
    }

    let mut flags = FlagOffset::empty();
    let first = frames[0];

    for (axis, (spline_flag, static_flag)) in [
        (FlagOffset::SPLINE_X, FlagOffset::STATIC_X),
        (FlagOffset::SPLINE_Y, FlagOffset::STATIC_Y),
        (FlagOffset::SPLINE_Z, FlagOffset::STATIC_Z),
    ]
    .iter()
    .enumerate()
    {
        let all_same = frames
            .iter()
            .all(|f| (f[axis] - first[axis]).abs() < FLOAT_EPS);
        if all_same {
            // Static: only set if value is non-zero (matches Havok convention)
            if first[axis].abs() >= FLOAT_EPS {
                flags |= *static_flag;
            }
        } else {
            flags |= *spline_flag;
        }
    }

    flags
}

/// Determine rotation FlagOffset by checking quaternion variance.
fn quat_flags(frames: &[Quaternion]) -> FlagOffset {
    if frames.is_empty() {
        return FlagOffset::empty();
    }

    let first = &frames[0];
    let all_same = frames.iter().all(|q| quat_approx_eq(q, first));

    if all_same {
        // Static: STATIC_X | STATIC_Y | STATIC_Z | STATIC_W
        FlagOffset::STATIC_X | FlagOffset::STATIC_Y | FlagOffset::STATIC_Z | FlagOffset::STATIC_W
    } else {
        FlagOffset::SPLINE_X | FlagOffset::SPLINE_Y | FlagOffset::SPLINE_Z | FlagOffset::SPLINE_W
    }
}

fn quat_approx_eq(a: &Quaternion, b: &Quaternion) -> bool {
    (a.x - b.x).abs() < FLOAT_EPS
        && (a.y - b.y).abs() < FLOAT_EPS
        && (a.z - b.z).abs() < FLOAT_EPS
        && (a.scaler - b.scaler).abs() < FLOAT_EPS
}

// ---------------------------------------------------------------------------
// TransformMask writer
// ---------------------------------------------------------------------------

fn write_transform_mask(out: &mut Vec<u8>, m: &TrackMask) {
    let pos_q_bits = m.pos_qt as u8 & 0x3;
    let rot_q_bits = (m.rot_qt as u8 & 0xF) << 2;
    let scale_q_bits = (m.scale_qt as u8 & 0x3) << 6;
    out.push(pos_q_bits | rot_q_bits | scale_q_bits);
    out.push(m.pos_flags.bits());
    out.push(m.rot_flags.bits());
    out.push(m.scale_flags.bits());
}

// ---------------------------------------------------------------------------
// Vec3 section writer
// ---------------------------------------------------------------------------

fn write_vec3_section(
    out: &mut Vec<u8>,
    frames: &[[f32; 3]],
    flags: FlagOffset,
    qt: ScalarQuantizationType,
) {
    let has_spline =
        flags.intersects(FlagOffset::SPLINE_X | FlagOffset::SPLINE_Y | FlagOffset::SPLINE_Z);

    if has_spline {
        write_spline_vec3(out, frames, flags, qt);
    } else {
        // Static path: raw f32 per active axis
        if flags.contains(FlagOffset::STATIC_X) {
            out.extend_from_slice(&frames[0][0].to_le_bytes());
        }
        if flags.contains(FlagOffset::STATIC_Y) {
            out.extend_from_slice(&frames[0][1].to_le_bytes());
        }
        if flags.contains(FlagOffset::STATIC_Z) {
            out.extend_from_slice(&frames[0][2].to_le_bytes());
        }
        // No data if flags are empty
    }
}

fn write_spline_vec3(
    out: &mut Vec<u8>,
    frames: &[[f32; 3]],
    flags: FlagOffset,
    qt: ScalarQuantizationType,
) {
    // num_items = n_frames - 1  (each frame is a control point)
    let num_items = frames.len().saturating_sub(1);

    // Header
    out.extend_from_slice(&(num_items as u16).to_le_bytes());
    out.push(1); // degree = 1 (linear)

    // Clamped uniform knot vector for degree 1:
    // [0, 0, 1, 2, ..., num_items-1, num_items, num_items]
    // length = num_items + 1 + 2 = num_items + degree + 2
    out.push(0);
    for i in 0..num_items {
        out.push(i.min(255) as u8);
    }
    out.push(num_items.min(255) as u8);

    // Pad to 4
    pad_to_4(out);

    // Compute bounds per active spline axis
    let bx = if flags.contains(FlagOffset::SPLINE_X) {
        Some(axis_bounds(frames, 0))
    } else {
        None
    };
    let by = if flags.contains(FlagOffset::SPLINE_Y) {
        Some(axis_bounds(frames, 1))
    } else {
        None
    };
    let bz = if flags.contains(FlagOffset::SPLINE_Z) {
        Some(axis_bounds(frames, 2))
    } else {
        None
    };

    // Write bounds
    if let Some((mn, mx)) = bx {
        out.extend_from_slice(&mn.to_le_bytes());
        out.extend_from_slice(&mx.to_le_bytes());
    }
    if let Some((mn, mx)) = by {
        out.extend_from_slice(&mn.to_le_bytes());
        out.extend_from_slice(&mx.to_le_bytes());
    }
    if let Some((mn, mx)) = bz {
        out.extend_from_slice(&mn.to_le_bytes());
        out.extend_from_slice(&mx.to_le_bytes());
    }

    // Write quantized control points
    for frame in frames {
        if let Some((mn, mx)) = bx {
            write_quantized_float(out, frame[0], mn, mx, qt);
        }
        if let Some((mn, mx)) = by {
            write_quantized_float(out, frame[1], mn, mx, qt);
        }
        if let Some((mn, mx)) = bz {
            write_quantized_float(out, frame[2], mn, mx, qt);
        }
    }
}

fn axis_bounds(frames: &[[f32; 3]], axis: usize) -> (f32, f32) {
    let mut mn = f32::MAX;
    let mut mx = f32::MIN;
    for f in frames {
        mn = mn.min(f[axis]);
        mx = mx.max(f[axis]);
    }
    (mn, mx)
}

fn write_quantized_float(
    out: &mut Vec<u8>,
    value: f32,
    min: f32,
    max: f32,
    qt: ScalarQuantizationType,
) {
    let range = max - min;
    let ratio = if range < FLOAT_EPS {
        0.0
    } else {
        (value - min) / range
    };
    let ratio = ratio.clamp(0.0, 1.0);
    match qt {
        ScalarQuantizationType::Bits8 => out.push((ratio * 255.0).round() as u8),
        ScalarQuantizationType::Bits16 => {
            out.extend_from_slice(&((ratio * 65535.0).round() as u16).to_le_bytes());
        }
    }
}

// ---------------------------------------------------------------------------
// Quaternion section writer
// ---------------------------------------------------------------------------

fn write_quat_section(
    out: &mut Vec<u8>,
    frames: &[Quaternion],
    flags: FlagOffset,
    qt: RotationQuantizationType,
) {
    let align = qt.rotation_align().unwrap_or(1);

    if flags.intersects(
        FlagOffset::SPLINE_X | FlagOffset::SPLINE_Y | FlagOffset::SPLINE_Z | FlagOffset::SPLINE_W,
    ) {
        let num_items = frames.len().saturating_sub(1);

        // Header
        out.extend_from_slice(&(num_items as u16).to_le_bytes());
        out.push(1); // degree = 1

        // Knot vector
        out.push(0);
        for i in 0..num_items {
            out.push(i.min(255) as u8);
        }
        out.push(num_items.min(255) as u8);

        // Pad to rotation align
        pad_to_align(out, align);

        for q in frames {
            write_quantized_quat(out, q, qt);
        }
    } else if flags.intersects(
        FlagOffset::STATIC_X | FlagOffset::STATIC_Y | FlagOffset::STATIC_Z | FlagOffset::STATIC_W,
    ) {
        pad_to_align(out, align);
        write_quantized_quat(out, &frames[0], qt);
    }
    // No flags: write nothing
}

fn write_quantized_quat(out: &mut Vec<u8>, q: &Quaternion, qt: RotationQuantizationType) {
    match qt {
        RotationQuantizationType::ThreeComp40 => write_quat_three_comp40(out, q),
        RotationQuantizationType::ThreeComp48 => write_quat_three_comp48(out, q),
        RotationQuantizationType::Polar32 => write_quat_polar32(out, q),
        RotationQuantizationType::Uncompressed => {
            out.extend_from_slice(&q.x.to_le_bytes());
            out.extend_from_slice(&q.y.to_le_bytes());
            out.extend_from_slice(&q.z.to_le_bytes());
            out.extend_from_slice(&q.scaler.to_le_bytes());
        }
        _ => {
            // ThreeComp24 / Straight16 not implemented; fall back to ThreeComp40
            write_quat_three_comp40(out, q);
        }
    }
}

// ---------------------------------------------------------------------------
// Quaternion encoders (inverse of de_spline decoders)
// ---------------------------------------------------------------------------

/// Encode THREECOMP40 (5 bytes).
///
/// Find the largest-magnitude component, omit it, encode the other three
/// as 12-bit signed values with bias 2047.
fn write_quat_three_comp40(out: &mut Vec<u8>, q: &Quaternion) {
    const MASK: u64 = (1 << 12) - 1;
    const BIAS: i32 = (MASK >> 1) as i32; // 2047
    const FRACTAL: f32 = 0.000_345_436;

    let (q, result_shift, r_sign) = normalize_for_threecomp(q);
    let comps = [q.x, q.y, q.z, q.scaler];

    // Gather the three non-omitted components in order
    let mut src = [0_i32; 3];
    let mut si = 0;
    for i in 0..4 {
        if i != result_shift {
            src[si] = ((comps[i] / FRACTAL) as i32 + BIAS).clamp(0, MASK as i32);
            si += 1;
        }
    }

    let c: u64 = (src[0] as u64 & MASK)
        | ((src[1] as u64 & MASK) << 12)
        | ((src[2] as u64 & MASK) << 24)
        | ((result_shift as u64 & 0x3) << 36)
        | (if r_sign { 1_u64 } else { 0 } << 38);

    // Write 5 bytes LE
    let bytes = c.to_le_bytes();
    out.extend_from_slice(&bytes[..5]);
}

/// Encode THREECOMP48 (6 bytes = 3 × i16).
fn write_quat_three_comp48(out: &mut Vec<u8>, q: &Quaternion) {
    const MASK: i16 = 0b0111_1111_1111_1111; // 0x7FFF
    const FRACTAL: f32 = 0.000_043_161;

    let (q, result_shift, r_sign) = normalize_for_threecomp(q);
    let comps = [q.x, q.y, q.z, q.scaler];

    let mut encoded = [0_i16; 3];
    let mut si = 0;

    for i in 0..4 {
        if i != result_shift {
            let v = ((comps[i] / FRACTAL) as i32 + (MASK >> 1) as i32).clamp(0, MASK as i32) as i16;
            encoded[si] = v;
            si += 1;
        }
    }

    // Pack result_shift bits into x[15] and y[14]
    let mut x = encoded[0];
    let mut y = encoded[1];
    let z = encoded[2];

    x = (x & MASK) | (((result_shift & 0x1) as i16) << 15);
    y = (y & MASK) | ((((result_shift >> 1) & 0x1) as i16) << 14);
    let z_out = (z & MASK)
        | (if r_sign {
            0_i16.wrapping_sub(1) & !MASK
        } else {
            0
        });

    out.extend_from_slice(&x.to_le_bytes());
    out.extend_from_slice(&y.to_le_bytes());
    out.extend_from_slice(&z_out.to_le_bytes());
}

/// Encode POLAR32 (4 bytes).
///
/// This is the inverse of `read_quat_polar32`.
fn write_quat_polar32(out: &mut Vec<u8>, q: &Quaternion) {
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_4};

    // Ensure w is the "R" component (largest magnitude convention used by POLAR32)
    let mut w = q.scaler;
    let mut x = q.x;
    let mut y = q.y;
    let mut z = q.z;

    let mut sign_x = false;
    let mut sign_y = false;
    let mut sign_z = false;
    let mut sign_w = false;

    if x < 0.0 {
        x = -x;
        sign_x = true;
    }
    if y < 0.0 {
        y = -y;
        sign_y = true;
    }
    if z < 0.0 {
        z = -z;
        sign_z = true;
    }
    if w < 0.0 {
        w = -w;
        sign_w = true;
    }

    // mag = sqrt(x^2+y^2+z^2), phi = atan2(sqrt(x^2+y^2), z), theta = atan2(y, x)
    let mag = z.mul_add(z, x.mul_add(x, y * y)).sqrt();
    let phi = mag.atan2(w).max(0.0); // polar angle from w-axis
    let theta = y.atan2(x);

    // phi_int: inverse of phiFrac * phi_int = phi  → phi_int = phi * 511 / FRAC_PI_2
    let phi_int = (phi * 511.0 / FRAC_PI_2).round().clamp(0.0, 511.0);

    // phiTheta: inverse of theta = FRAC_PI_4 * (phiTheta - phi_int^2) / phi_int
    // → phiTheta = theta * phi_int / FRAC_PI_4 + phi_int^2
    let phi_theta = if phi_int > 0.0 {
        (theta * phi_int / FRAC_PI_4 + phi_int * phi_int)
            .round()
            .clamp(0.0, (1 << 18) as f32 - 1.0)
    } else {
        0.0
    };

    // r_raw: inverse of w = 1 - r_raw^2  → r_raw = sqrt(1 - w) (w here is cos(polar_angle))
    // But in the decoder: r_raw = f32::from_bits(r_bits) * R_FRAC, w = 1 - r_raw^2
    // So r_raw = sqrt(1 - w_component^2)... we store w as the cos, so:
    let r_raw = w.mul_add(-w, 1.0).sqrt().clamp(0.0, 1.0);
    const R_MASK: u32 = (1 << 10) - 1;
    const R_FRAC: f32 = 1.0 / R_MASK as f32;
    // Inverse of: r_raw = f32::from_bits(r_bits) * R_FRAC
    // r_bits = (r_raw / R_FRAC).to_bits() ... but r_raw is a normal float in [0,1]
    // Use linear quantization instead (the original CastToFloat is unusual)
    let r_q = ((r_raw / R_FRAC) as u32).clamp(0, R_MASK);

    let c: u32 = (phi_theta as u32 & 0x0003_FFFF)
        | ((r_q & R_MASK) << 18)
        | (if sign_x { 0x1000_0000 } else { 0 })
        | (if sign_y { 0x2000_0000 } else { 0 })
        | (if sign_z { 0x4000_0000 } else { 0 })
        | (if sign_w { 0x8000_0000 } else { 0 });

    out.extend_from_slice(&c.to_le_bytes());
}

/// Find the largest-magnitude component, make it positive, and return the
/// normalized quaternion along with the omitted component index and its sign.
fn normalize_for_threecomp(q: &Quaternion) -> (Quaternion, usize, bool) {
    let comps = [q.x, q.y, q.z, q.scaler];
    let (result_shift, _) = comps
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            a.abs()
                .partial_cmp(&b.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or((0, &0.0));

    let r_sign = comps[result_shift] < 0.0;
    // Normalize so the omitted component is positive before encoding
    let scale = if r_sign { -1.0 } else { 1.0 };
    let q_norm = Quaternion::new(q.x * scale, q.y * scale, q.z * scale, scale);

    (q_norm, result_shift, r_sign)
}

// ---------------------------------------------------------------------------
// Padding helpers
// ---------------------------------------------------------------------------

fn pad_to_4(out: &mut Vec<u8>) {
    while out.len() % 4 != 0 {
        out.push(0);
    }
}

fn pad_to_align(out: &mut Vec<u8>, align: usize) {
    if align > 1 {
        while out.len() % align != 0 {
            out.push(0);
        }
    }
}
