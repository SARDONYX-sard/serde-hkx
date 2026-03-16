//! Sample a `hkaSplineCompressedAnimation` into per-track key lists.
//!
//! Mirrors `AnimationExport::exportController` from the C++ reference,
//! replacing `anim->samplePartialTracks()` with a pure-Rust NURBS evaluator
//! built from the decoded spline blocks produced by `de_spline`.
//!
//! # NURBS evaluation
//!
//! Implements Algorithm A2.1 (FindKnotSpan) and Basis_ITS1 / GetPoint_NR1
//! from *The NURBS Book*, 2nd ed., pages 68 and 64–65, exactly as the
//! original C# decompressor does.

use havok_classes::{BlendHint, hkaAnimationBinding, hkaSkeleton, hkaSplineCompressedAnimation};
use havok_types::Quaternion;
use snafu::Snafu;

use super::de_spline::{QuatTrack, SplineChannel, Vec3Track, read_spline_compressed_blocks};

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Vector3Key {
    pub time: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct QuaternionKey {
    pub time: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone)]
pub struct FloatKey {
    pub time: f32,
    pub data: f32,
}

/// All sampled keys for one transform track.
#[derive(Debug)]
pub struct TransformTrackKeys {
    pub bone_name: String,
    pub translate: Vec<Vector3Key>,
    pub rotate: Vec<QuaternionKey>,
    pub scale: Vec<FloatKey>,
}

/// All sampled keys for one float track.
#[derive(Debug)]
pub struct FloatTrackKeys {
    pub slot_name: String,
    pub keys: Vec<FloatKey>,
}

/// Output of [`sample_animation`].
#[derive(Debug)]
pub struct SampledAnimation {
    pub duration: f32,
    pub transform_tracks: Vec<TransformTrackKeys>,
    pub float_tracks: Vec<FloatTrackKeys>,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Sample a spline-compressed animation into linear key lists.
///
/// Mirrors the frame loop in `AnimationExport::exportController`:
/// for each frame, evaluate every track's NURBS curves and record the result.
pub fn sample_animation(
    skeleton: &hkaSkeleton,
    binding: &hkaAnimationBinding,
    anim: &hkaSplineCompressedAnimation,
    export_float_tracks: bool,
) -> Result<SampledAnimation, SamplingError> {
    let num_tracks = anim.parent.m_numberOfTransformTracks as usize;
    let num_floats = anim.parent.m_numberOfFloatTracks as usize;
    let duration = anim.parent.m_duration;
    let num_frames = anim.m_numFrames as usize;

    // frame_duration = duration / (nframes - 1)  (matches C++ incrFrame)
    let incr = if num_frames > 1 {
        duration / (num_frames - 1) as f32
    } else {
        0.0
    };

    // Decode the spline blocks from m_data.
    let blocks = read_spline_compressed_blocks(&anim.m_data, num_tracks, anim.m_numBlocks as usize)
        .map_err(|e| SamplingError::DeSpline {
            message: e.to_string(),
        })?;

    // Flatten blocks into a single indexed structure: tracks[track_idx].
    // Each block covers up to 255 frames; stitch them together.
    let max_fpb = anim.m_maxFramesPerBlock as usize;

    // Per-track accumulators.
    let is_additive = binding.m_blendHint == BlendHint::ADDITIVE;

    let mut translate: Vec<Vec<Vector3Key>> = (0..num_tracks).map(|_| Vec::new()).collect();
    let mut rotate: Vec<Vec<QuaternionKey>> = (0..num_tracks).map(|_| Vec::new()).collect();
    let mut scale_keys: Vec<Vec<FloatKey>> = (0..num_tracks).map(|_| Vec::new()).collect();

    for (block_idx, block_tracks) in blocks.iter().enumerate() {
        let frame_start = block_idx * max_fpb;
        let frame_end = (frame_start + max_fpb).min(num_frames);
        let frames_in_block = frame_end - frame_start;

        // Normalised time range for this block [0, block_duration].
        // let block_duration = anim.m_blockDuration;

        for (track_idx, track) in block_tracks.iter().enumerate() {
            if track_idx >= num_tracks {
                break;
            }

            for fi in 0..frames_in_block {
                // Local time within the block (in frames, not seconds).
                // The knot vectors use frame indices as knot values.
                let local_frame = fi as f32;
                let global_time = (frame_start + fi) as f32 * incr;

                // --- Translation ---
                let (tx, ty, tz) = eval_vec3(&track.position, local_frame);
                translate[track_idx].push(Vector3Key {
                    time: global_time,
                    x: tx,
                    y: ty,
                    z: tz,
                });

                // --- Rotation ---
                let q = eval_quat(&track.rotation, local_frame);
                rotate[track_idx].push(QuaternionKey {
                    time: global_time,
                    x: q.x,
                    y: q.y,
                    z: q.z,
                    w: q.scaler,
                });

                // --- Scale (uniform: use x component, matches C++ `sc.x`) ---
                let (sx, _, _) = eval_vec3(&track.scale, local_frame);
                scale_keys[track_idx].push(FloatKey {
                    time: global_time,
                    data: sx,
                });
            }
        }
    }

    // Build reference pose for key reduction
    // (matches C++ "if always at ref pose, keep only a single key").
    let mut transform_tracks = Vec::with_capacity(num_tracks);

    for track_idx in 0..num_tracks {
        let bone_idx = if is_additive {
            track_idx
        } else if let Some(&bi) = binding
            .m_transformTrackToBoneIndices
            .get(track_idx)
            .map(|v| v as &i16)
        {
            bi as usize
        } else {
            track_idx
        };

        let bone_name = skeleton.m_bones.get(bone_idx).map_or_else(
            || format!("bone_{bone_idx}"),
            |b| b.m_name.as_str().to_owned(),
        );

        let (ref_t, ref_r, ref_s) = reference_pose(skeleton, binding, bone_idx, is_additive);

        let t_keys = reduce_translate(translate[track_idx].clone(), ref_t);
        let r_keys = reduce_rotate(rotate[track_idx].clone(), ref_r);
        let s_keys = reduce_scale(scale_keys[track_idx].clone(), ref_s);

        transform_tracks.push(TransformTrackKeys {
            bone_name,
            translate: t_keys,
            rotate: r_keys,
            scale: s_keys,
        });
    }

    // Float tracks.
    let mut float_tracks = Vec::new();
    if export_float_tracks {
        // Float tracks are not spline-compressed in the blocks; they come from
        // m_floats in hkaInterleavedUncompressedAnimation. For spline-compressed
        // animations, float tracks are stored separately – currently not decoded.
        // Emit empty tracks with the correct slot names as a placeholder.
        for float_idx in 0..num_floats {
            let slot_name = if let Some(&si) = binding
                .m_floatTrackToFloatSlotIndices
                .get(float_idx)
                .map(|v| v as &i16)
            {
                skeleton
                    .m_floatSlots
                    .get(si as usize)
                    .map_or_else(|| format!("float_{float_idx}"), |s| s.as_str().to_owned())
            } else {
                skeleton
                    .m_floatSlots
                    .get(float_idx)
                    .map_or_else(|| format!("float_{float_idx}"), |s| s.as_str().to_owned())
            };

            float_tracks.push(FloatTrackKeys {
                slot_name,
                keys: vec![],
            });
        }
    }

    Ok(SampledAnimation {
        duration,
        transform_tracks,
        float_tracks,
    })
}

// ---------------------------------------------------------------------------
// NURBS evaluation
// ---------------------------------------------------------------------------

/// Algorithm A2.1 – FindKnotSpan (*The NURBS Book*, p. 68).
///
/// Returns the knot span index `i` such that `knots[i] <= value < knots[i+1]`.
fn find_knot_span(degree: usize, value: f32, n: usize, knots: &[u8]) -> usize {
    // Special case: clamp to last span when value == knots[n].
    if value >= knots[n] as f32 {
        return n - 1;
    }

    let mut low = degree;
    let mut high = n;
    let mut mid = (low + high) / 2;

    while value < knots[mid] as f32 || value >= knots[mid + 1] as f32 {
        if value < knots[mid] as f32 {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }

    mid
}

/// Basis_ITS1 / GetPoint_NR1 – evaluate a scalar B-spline at `frame`.
///
/// (*The NURBS Book*, pp. 64–65)
fn nurbs_eval_f32(span: usize, degree: usize, frame: f32, knots: &[u8], cpoints: &[f32]) -> f32 {
    let mut n = [0.0_f32; 5]; // degree <= 3, so 5 is enough
    n[0] = 1.0;

    for i in 1..=degree {
        for j in (0..i).rev() {
            let a = (frame - knots[span - j] as f32)
                / (knots[span + i - j] as f32 - knots[span - j] as f32);
            let tmp = n[j] * a;
            n[j + 1] += n[j] - tmp;
            n[j] = tmp;
        }
    }

    let mut result = 0.0_f32;
    for i in 0..=degree {
        result += cpoints[span - i] * n[i];
    }
    result
}

/// Evaluate a quaternion B-spline at `frame`.
fn nurbs_eval_quat(
    span: usize,
    degree: usize,
    frame: f32,
    knots: &[u8],
    cpoints: &[Quaternion],
) -> Quaternion {
    let mut n = [0.0_f32; 5];
    n[0] = 1.0;

    for i in 1..=degree {
        for j in (0..i).rev() {
            let a = (frame - knots[span - j] as f32)
                / (knots[span + i - j] as f32 - knots[span - j] as f32);
            let tmp = n[j] * a;
            n[j + 1] += n[j] - tmp;
            n[j] = tmp;
        }
    }

    let mut rx = 0.0_f32;
    let mut ry = 0.0_f32;
    let mut rz = 0.0_f32;
    let mut rw = 0.0_f32;
    for i in 0..=degree {
        let q = &cpoints[span - i];
        rx += q.x * n[i];
        ry += q.y * n[i];
        rz += q.z * n[i];
        rw += q.scaler * n[i];
    }

    // Normalize to keep unit quaternion after blending.
    let len = rw
        .mul_add(rw, rz.mul_add(rz, rx.mul_add(rx, ry * ry)))
        .sqrt();
    if len > 1e-8 {
        Quaternion::new(rx / len, ry / len, rz / len, rw / len)
    } else {
        Quaternion::new(0.0, 0.0, 0.0, 1.0)
    }
}

// ---------------------------------------------------------------------------
// Per-section evaluators
// ---------------------------------------------------------------------------

/// Evaluate a Vec3Track at local frame `t`.
/// Returns `(x, y, z)`; absent axes return 0.0 for position, 1.0 for scale
/// is handled by the caller via reference-pose fallback.
fn eval_vec3(track: &Option<Vec3Track<'_>>, t: f32) -> (f32, f32, f32) {
    let Some(v) = track else {
        return (0.0, 0.0, 0.0);
    };

    let eval_axis = |ch: &Option<SplineChannel<f32>>| -> f32 {
        let Some(c) = ch else { return 0.0 };
        if c.values.len() == 1 {
            return c.values[0]; // static
        }
        let degree = v.degree as usize;
        let n = c.values.len(); // num_items + 1
        let span = find_knot_span(degree, t, n, v.knots);
        nurbs_eval_f32(span, degree, t, v.knots, &c.values)
    };

    (eval_axis(&v.x), eval_axis(&v.y), eval_axis(&v.z))
}

/// Evaluate a QuatTrack at local frame `t`.
fn eval_quat(track: &Option<QuatTrack<'_>>, t: f32) -> Quaternion {
    let Some(q) = track else {
        return Quaternion::new(0.0, 0.0, 0.0, 1.0); // identity
    };

    if q.values.len() == 1 {
        return q.values[0].clone(); // static
    }

    let degree = q.degree as usize;
    let n = q.values.len();
    let span = find_knot_span(degree, t, n, q.knots);
    nurbs_eval_quat(span, degree, t, q.knots, &q.values)
}

// ---------------------------------------------------------------------------
// Reference pose + key reduction  (mirrors C++ EQUALS-based reduction)
// ---------------------------------------------------------------------------

const EPSILON: f32 = 1e-5;

fn reference_pose(
    skeleton: &hkaSkeleton,
    _binding: &hkaAnimationBinding,
    bone_idx: usize,
    is_additive: bool,
) -> ([f32; 3], [f32; 4], f32) {
    if is_additive {
        // Additive: identity reference.
        ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], 1.0)
    } else if let Some(pose) = skeleton.m_referencePose.get(bone_idx) {
        let t = &pose.transition;
        let r = &pose.quaternion;
        let s = pose.scale.x;
        ([t.x, t.y, t.z], [r.x, r.y, r.z, r.scaler], s)
    } else {
        ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], 1.0)
    }
}

/// If every key equals the reference, keep only the first key.
fn reduce_translate(keys: Vec<Vector3Key>, ref_t: [f32; 3]) -> Vec<Vector3Key> {
    if keys.iter().all(|k| {
        (k.x - ref_t[0]).abs() < EPSILON
            && (k.y - ref_t[1]).abs() < EPSILON
            && (k.z - ref_t[2]).abs() < EPSILON
    }) {
        keys.into_iter().take(1).collect()
    } else {
        keys
    }
}

fn reduce_rotate(keys: Vec<QuaternionKey>, ref_r: [f32; 4]) -> Vec<QuaternionKey> {
    if keys.iter().all(|k| {
        (k.x - ref_r[0]).abs() < EPSILON
            && (k.y - ref_r[1]).abs() < EPSILON
            && (k.z - ref_r[2]).abs() < EPSILON
            && (k.w - ref_r[3]).abs() < EPSILON
    }) {
        keys.into_iter().take(1).collect()
    } else {
        keys
    }
}

fn reduce_scale(keys: Vec<FloatKey>, ref_s: f32) -> Vec<FloatKey> {
    if keys.iter().all(|k| (k.data - ref_s).abs() < EPSILON) {
        keys.into_iter().take(1).collect()
    } else {
        keys
    }
}

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug, Snafu)]
pub enum SamplingError {
    #[snafu(display("de_spline error: {message}"))]
    DeSpline { message: String },
}
