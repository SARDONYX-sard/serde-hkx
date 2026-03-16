use cxx::UniquePtr;
use havok_classes::{hkaAnimationBinding, hkaSkeleton};
use havok_types::QsTransform;
use rayon::prelude::*;

use crate::kf::from_kf::{
    bridge,
    math::{
        fill_transforms, normalize_rotations, sample_float, sample_rotation, sample_scale,
        sample_transition,
    },
};

pub struct AnimationLayout {
    pub duration: f32,
    pub n_frames: usize,
    pub n_transforms: usize,
    pub n_floats: usize,
}

pub fn compute_animation_layout(
    seq: &bridge::ffi::NiControllerSequenceRef,
    binding: &hkaAnimationBinding,
    bones_len: usize,
    track_to_bone_len: usize,
    float_tracks_len: usize,
    convert_float_tracks: bool,
    frames_increment: f32,
) -> AnimationLayout {
    let duration = {
        let start = bridge::ffi::get_start_time(seq);
        let stop = bridge::ffi::get_stop_time(seq);
        stop - start
    };

    let n_frames = (duration / frames_increment).round() as usize + 1;

    let n_transforms = if binding.m_blendHint == havok_classes::BlendHint::ADDITIVE {
        track_to_bone_len
    } else {
        bones_len
    };

    let n_floats = if convert_float_tracks {
        float_tracks_len
    } else {
        0
    };

    AnimationLayout {
        duration,
        n_frames,
        n_transforms,
        n_floats,
    }
}

pub fn sample_transform_animation(
    skeleton: &hkaSkeleton,
    binding: &hkaAnimationBinding,
    layout: &AnimationLayout,
    track_to_bone: &[usize],
    transform_data: &[UniquePtr<bridge::ffi::NiTransformDataRef>],
    frames_increment: f32,
    transforms: &mut [QsTransform],
) {
    // NORMAL → init bind pose
    if binding.m_blendHint == havok_classes::BlendHint::NORMAL {
        for index in 0..layout.n_transforms {
            let local = &skeleton.m_referencePose[index];
            fill_transforms(transforms, index, layout.n_transforms, local);
        }
    }

    for (i, data) in transform_data.iter().enumerate() {
        let track_idx = if binding.m_blendHint == havok_classes::BlendHint::ADDITIVE {
            i
        } else {
            track_to_bone[i]
        };

        let keys = bridge::ffi::get_transform_keys(data);

        // --- translation ---
        let t_keys = keys.get_translate_keys();
        let t_type = bridge::ffi::get_translate_key_type(data);
        let mut t_hint = 0_usize;

        // --- rotation ---
        let r_keys = keys.get_rotate_keys();
        let r_type = bridge::ffi::get_rotate_key_type(data);
        let mut r_hint = 0_usize;

        // --- scale ---
        let s_keys = keys.get_scale_keys();
        let s_type = bridge::ffi::get_scale_key_type(data);
        let mut s_hint = 0_usize;

        for frame in 0..layout.n_frames {
            let time = frame as f32 * frames_increment;
            let idx = frame * layout.n_transforms + track_idx;
            let transform = &mut transforms[idx];

            if !t_keys.is_empty() {
                transform.transition =
                    sample_transition(time, t_keys.as_slice(), t_type, &mut t_hint);
            }

            if !r_keys.is_empty() {
                transform.quaternion =
                    sample_rotation(time, r_keys.as_slice(), r_type, &mut r_hint);
            }

            if !s_keys.is_empty() {
                transform.scale = sample_scale(time, s_keys.as_slice(), s_type, &mut s_hint);
            }
        }
    }

    normalize_rotations(transforms);
}

pub fn sample_float_animation(
    layout: &AnimationLayout,
    float_data: &[UniquePtr<bridge::ffi::NiFloatDataRef>],
    frames_increment: f32,
    floats: &mut [f32],
) {
    for (i, data) in float_data.iter().enumerate() {
        let keys = bridge::ffi::get_float_keys(data);
        if keys.is_empty() {
            continue;
        }

        let key_type = bridge::ffi::get_float_key_type(data);
        let mut key_hint = 0;

        for frame in 0..layout.n_frames {
            let time = frame as f32 * frames_increment;
            let idx = frame * layout.n_floats + i;

            floats[idx] = sample_float(time, keys.as_slice(), key_type, &mut key_hint);
        }
    }
}

pub fn finalize_binding(
    binding: &mut hkaAnimationBinding,
    track_to_bone: &[usize],
    track_to_float: &[usize],
    convert_float_tracks: bool,
) {
    // bind float track indices
    if convert_float_tracks {
        binding.m_floatTrackToFloatSlotIndices =
            track_to_float.par_iter().map(|&x| x as i16).collect();
    }

    // If additive, record transform->bone mapping(NOTE: Not hkxcmd code)
    if binding.m_blendHint == havok_classes::BlendHint::ADDITIVE {
        binding.m_transformTrackToBoneIndices = track_to_bone.iter().map(|&x| x as i16).collect();
    }
}
