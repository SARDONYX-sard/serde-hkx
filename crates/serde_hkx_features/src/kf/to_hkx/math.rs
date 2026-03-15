use havok_types::{QsTransform, Quaternion, Vector4};
use rayon::prelude::*;

use crate::kf::to_hkx::bridge;

use bridge::{KeyType, Vector3};

#[inline]
fn float_equals(a: f32, b: f32) -> bool {
    /// Floating comparison epsilon taken from the original C++ implementation
    const MY_FLT_EPSILON: f32 = 1e-5;

    (a - b).abs() < MY_FLT_EPSILON
}

/// Set interpolated transition for the given frame.
pub(crate) fn sample_transition(
    time: f32,
    keys: &[bridge::ffi::Vector3Key],
    key_type: KeyType,
    i_ptr: &mut usize,
) -> Vector4 {
    // advance to the first key greater in time than current frame
    while *i_ptr < keys.len() && keys[*i_ptr].time <= time {
        *i_ptr += 1;
    }

    let vec = if *i_ptr == 0 {
        keys.first().map_or(Vector3::DEFAULT, |k| k.data.clone()) // clamp to first
    } else if *i_ptr >= keys.len() {
        keys.last().map_or(Vector3::DEFAULT, |k| k.data.clone())
    } else {
        let prev = &keys[*i_ptr - 1];
        let prev_time = prev.time;
        let next = &keys[*i_ptr];
        let next_time = next.time;
        if float_equals(prev_time, time) || key_type == KeyType::Const {
            prev.data.clone()
        } else if float_equals(next_time, time) {
            next.data.clone()
        } else {
            let prev_data = prev.data.clone();
            let next_data = next.data.clone();

            // linear interpolate
            let h = next_time - prev_time;
            let u = if h.abs() < 1e-12 {
                0.0
            } else {
                (time - prev_time) / h
            };

            Vector3 {
                x: (next_data.x - prev_data.x).mul_add(u, prev_data.x),
                y: (next_data.y - prev_data.y).mul_add(u, prev_data.y),
                z: (next_data.z - prev_data.z).mul_add(u, prev_data.z),
            }
        }
    };

    Vector4 {
        x: vec.x,
        y: vec.y,
        z: vec.z,
        w: 0.0,
    }
}

/// Set interpolated rotation for the given frame (linear on quaternion components,
/// matching the conservative behavior in the original C++ implementation).
pub(crate) fn sample_rotation(
    time: f32,
    keys: &[bridge::ffi::QuaternionKey],
    key_type: KeyType,
    i_ptr: &mut usize,
) -> Quaternion {
    while *i_ptr < keys.len() && keys[*i_ptr].time <= time {
        *i_ptr += 1;
    }

    let quat = if *i_ptr == 0 {
        keys.first()
            .map_or(bridge::Quaternion::DEFAULT, |k| k.data.clone())
    } else if *i_ptr >= keys.len() {
        keys.last()
            .map_or(bridge::Quaternion::DEFAULT, |k| k.data.clone())
    } else {
        let prev = &keys[*i_ptr - 1];
        let next = &keys[*i_ptr];

        let prev_time = prev.time;
        let next_time = next.time;

        if float_equals(prev_time, time) || key_type == KeyType::Const {
            prev.data.clone()
        } else if float_equals(next_time, time) {
            next.data.clone()
        } else {
            let prev_data = prev.data.clone();
            let next_data = next.data.clone();

            let h = next_time - prev_time;
            let u = if h.abs() < 1e-12 {
                0.0
            } else {
                (time - prev_time) / h
            };

            bridge::Quaternion {
                x: (next_data.x - prev_data.x).mul_add(u, prev_data.x),
                y: (next_data.y - prev_data.y).mul_add(u, prev_data.y),
                z: (next_data.z - prev_data.z).mul_add(u, prev_data.z),
                w: (next_data.w - prev_data.w).mul_add(u, prev_data.w),
            }
        }
    };

    Quaternion {
        x: quat.x,
        y: quat.y,
        z: quat.z,
        scaler: quat.w,
    }
}

pub(crate) fn sample_scale(
    time: f32,
    keys: &[bridge::ffi::FloatKey],
    key_type: KeyType,
    i_ptr: &mut usize,
) -> Vector4 {
    while *i_ptr < keys.len() && keys[*i_ptr].time <= time {
        *i_ptr += 1;
    }

    let s = if *i_ptr == 0 {
        keys.first().map_or(1.0, |k| k.data)
    } else if *i_ptr >= keys.len() {
        keys.last().map_or(1.0, |k| k.data)
    } else {
        let prev = &keys[*i_ptr - 1];
        let next = &keys[*i_ptr];

        let prev_time = prev.time;
        let next_time = next.time;

        if float_equals(prev_time, time) || key_type == KeyType::Const {
            prev.data
        } else if float_equals(next_time, time) {
            next.data
        } else {
            let h = next_time - prev_time;
            let u = if h.abs() < 1e-12 {
                0.0
            } else {
                (time - prev_time) / h
            };
            (next.data - prev.data).mul_add(u, prev.data)
        }
    };

    Vector4 {
        x: s,
        y: s,
        z: s,
        w: 0.0,
    }
}

pub(crate) fn sample_float(
    time: f32,
    keys: &[bridge::ffi::FloatKey],
    key_type: KeyType,
    i_ptr: &mut usize,
) -> f32 {
    while *i_ptr < keys.len() && keys[*i_ptr].time <= time {
        *i_ptr += 1;
    }

    match *i_ptr {
        0 => keys.first().map_or(0.0, |k| k.data),
        i_ptr if i_ptr >= keys.len() => keys.last().map_or(0.0, |k| k.data),
        i_ptr => {
            let prev = &keys[i_ptr - 1];
            let next = &keys[i_ptr];

            let prev_time = prev.time;
            let next_time = next.time;

            if float_equals(prev_time, time) || key_type == KeyType::Const {
                prev.data
            } else if float_equals(next_time, time) {
                next.data
            } else {
                let h = next_time - prev_time;
                let u = if h.abs() < 1e-12 {
                    0.0
                } else {
                    (time - prev_time) / h
                };
                (next.data - prev.data).mul_add(u, prev.data)
            }
        }
    }
}

/// Normalize quaternion rotations for all transforms to unit length.
pub(crate) fn normalize_rotations(transforms: &mut [QsTransform]) {
    for tr in transforms.iter_mut() {
        let q = &mut tr.quaternion;
        let len_sq = q
            .scaler
            .mul_add(q.scaler, q.z.mul_add(q.z, q.x.mul_add(q.x, q.y * q.y)));
        if len_sq > 0.0 {
            let inv = 1.0 / len_sq.sqrt();
            q.x *= inv;
            q.y *= inv;
            q.z *= inv;
            q.scaler *= inv;
        } else {
            // fallback to identity quaternion
            q.x = 0.0;
            q.y = 0.0;
            q.z = 0.0;
            q.scaler = 1.0;
        }
    }
}

pub(crate) fn fill_transforms(
    transforms: &mut [QsTransform],
    bone_idx: usize,
    num_tracks: usize,
    local: &QsTransform,
) {
    let frame_count = transforms.len() / num_tracks;
    if frame_count == 0 || bone_idx >= num_tracks {
        return;
    }

    transforms
        .par_chunks_exact_mut(num_tracks)
        .enumerate()
        .for_each(|(_, chunk)| {
            chunk[bone_idx] = local.clone();
        });
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn set_ipltd_translation_linear() {}
}
