//! Apply kf to hkx
mod bridge;
mod math;
mod sampling;
mod ser_spline;

use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
};

use cxx::UniquePtr;
use havok_classes::{
    Classes, hkaAnimation, hkaAnimationBinding, hkaInterleavedUncompressedAnimation, hkaSkeleton,
    hkaSplineCompressedAnimation, hkaSplineCompressedAnimationTrackCompressionParams,
};
use havok_types::{Pointer, QsTransform};
use rayon::prelude::*;
use snafu::ResultExt as _;

use crate::kf::to_hkx::sampling::AnimationLayout;
use crate::{ClassMap, Format};

#[derive(Debug, Clone)]
pub struct Config {
    /// keep behavior configurable
    pub no_root_siblings: bool,
    pub additive_blend: bool,
    pub convert_float_tracks: bool,

    pub frames_per_second: f32,
    pub frames_increment: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            no_root_siblings: false,
            additive_blend: false,
            convert_float_tracks: false,
            frames_per_second: 30.0,
            frames_increment: 0.033333,
        }
    }
}

fn build_bone_map<'a>(skeleton: &'a hkaSkeleton) -> HashMap<&'a str, usize> {
    skeleton
        .m_bones
        .par_iter()
        .enumerate()
        .map(|(i, b)| (b.m_name.as_str(), i))
        .collect()
}

fn build_float_map<'a>(skeleton: &'a hkaSkeleton) -> HashMap<&'a str, usize> {
    skeleton
        .m_floatSlots
        .par_iter()
        .enumerate()
        .map(|(i, n)| (n.as_str(), i))
        .collect()
}

struct TrackVectors {
    track_to_bone: Vec<usize>,
    transform_data: Vec<UniquePtr<bridge::ffi::NiTransformDataRef>>,
    track_to_float: Vec<usize>,
    float_data: Vec<UniquePtr<bridge::ffi::NiFloatDataRef>>,
}

/// Collect transform & float interpolators that map to known bones/slots
fn collect_tracks(
    seq: &bridge::ffi::NiControllerSequenceRef,
    bone_map: &HashMap<&str, usize>,
    float_map: &HashMap<&str, usize>,
    convert_float_tracks: bool,
) -> TrackVectors {
    let mut track_to_bone = Vec::new();
    let mut transform_data = Vec::new();
    let mut track_to_float = Vec::new();
    let mut float_data = Vec::new();

    let blocks = bridge::ffi::get_controller_links(seq);

    for block in blocks.iter() {
        if bridge::ffi::is_transform_type(block) {
            let name = bridge::ffi::get_node_name(block).to_string_lossy();
            if let Some(&idx) = bone_map.get(name.as_ref()) {
                track_to_bone.push(idx);
                transform_data.push(bridge::ffi::get_transform_ref(block));
            } else {
                tracing::warn!("Unknown bone '{name}' found in animation. Skipping.");
            }
        }

        if convert_float_tracks && bridge::ffi::is_float_type(block) {
            let name = bridge::ffi::get_node_name(block).to_string_lossy();
            if let Some(&idx) = float_map.get(name.as_ref()) {
                track_to_float.push(idx);
                float_data.push(bridge::ffi::get_float_ref(block));
            } else {
                tracing::warn!("Unknown float '{name}' found in animation. Skipping.");
            }
        }
    }

    TrackVectors {
        track_to_bone,
        transform_data,
        track_to_float,
        float_data,
    }
}

/// Compute number of transform/float tracks
fn compute_bones_len(skeleton: &hkaSkeleton, no_root_siblings: bool) -> usize {
    let mut len = skeleton.m_bones.len();

    if no_root_siblings {
        for i in 1..len {
            if skeleton.m_parentIndices.get(i).is_some() {
                len = i;
                break;
            }
        }
    }

    len
}

fn create_temp_animation<'a>(layout: &AnimationLayout) -> hkaInterleavedUncompressedAnimation<'a> {
    hkaInterleavedUncompressedAnimation {
        parent: hkaAnimation {
            // `HK_SPLINE_COMPRESSED_ANIMATION`: We'll put this in `hkaSplineCompressedAnimation` later, so specify this.
            m_type: havok_classes::AnimationType::HK_SPLINE_COMPRESSED_ANIMATION,
            m_duration: layout.duration,
            m_numberOfTransformTracks: layout.n_transforms as i32,
            m_numberOfFloatTracks: layout.n_floats as i32,
            m_annotationTracks: Vec::with_capacity(layout.n_transforms), // TODO: trackName: null str with transforms
            // m_extractedMotion: Pointer::new(0) // hkaDefaultAnimatedReferenceFrame*
            ..Default::default()
        },
        // transform[track][frame] = track + frame * n_transforms
        m_transforms: vec![QsTransform::identity(); layout.n_transforms * layout.n_frames],

        // float[track][frame] = track + frame * n_floats
        m_floats: vec![0.0; layout.n_floats * layout.n_frames],

        ..Default::default()
    }
}

fn export_controller<'a>(
    seq: &bridge::ffi::NiControllerSequenceRef,
    skeleton: &hkaSkeleton<'a>,
    binding: &mut hkaAnimationBinding<'a>,
    config: &Config,
) -> Result<hkaSplineCompressedAnimation<'a>, KfSerError> {
    let Config {
        no_root_siblings,
        additive_blend,
        convert_float_tracks,
        frames_per_second,
        frames_increment,
    } = config.clone();

    if additive_blend {
        binding.m_blendHint = havok_classes::BlendHint::ADDITIVE;
    }

    let bone_map = build_bone_map(skeleton);
    let float_map = build_float_map(skeleton);

    let TrackVectors {
        track_to_bone,
        transform_data,
        track_to_float,
        float_data,
    } = collect_tracks(seq, &bone_map, &float_map, convert_float_tracks);

    let layout = sampling::compute_animation_layout(
        seq,
        binding,
        compute_bones_len(skeleton, no_root_siblings),
        track_to_bone.len(),
        float_data.len(),
        convert_float_tracks,
        frames_increment,
    );

    let mut temp_anim = create_temp_animation(&layout);

    sampling::sample_transform_animation(
        skeleton,
        binding,
        &layout,
        &track_to_bone,
        &transform_data,
        frames_increment,
        &mut temp_anim.m_transforms,
    );

    if convert_float_tracks {
        sampling::sample_float_animation(
            &layout,
            &float_data,
            frames_increment,
            &mut temp_anim.m_floats,
        );
    }

    sampling::finalize_binding(
        binding,
        &track_to_bone,
        &track_to_float,
        convert_float_tracks,
    );
    let track_params = hkaSplineCompressedAnimationTrackCompressionParams {
        m_rotationTolerance: 0.001,
        m_rotationQuantizationType: havok_classes::RotationQuantization::THREECOMP40,
        ..Default::default()
    };

    Ok(new_animation(
        temp_anim,
        frames_per_second as u32,
        track_params,
    ))
}

/// Reproduces in Rust the calculations normally performed inside the
/// C++ constructor `hkaSplineCompressedAnimation(*raw, tcp, acp)`.
///
/// This version does not add any methods to existing structs – it only
/// computes values and stores them into the output struct.
fn new_animation<'a>(
    temp_anim: hkaInterleavedUncompressedAnimation<'a>,
    n_frames_per_second: u32,
    params: hkaSplineCompressedAnimationTrackCompressionParams,
) -> hkaSplineCompressedAnimation<'a> {
    use crate::kf::to_hkx::ser_spline::write_block;

    let duration = temp_anim.parent.m_duration;
    let n_transforms = temp_anim.parent.m_numberOfTransformTracks as usize;
    let n_floats = temp_anim.parent.m_numberOfFloatTracks as usize;

    const MAX_FRAMES_PER_BLOCK: usize = 255;

    let total_frames = ((duration * n_frames_per_second as f32).ceil() as usize).max(1);
    let frame_duration = if total_frames > 1 {
        duration / (total_frames - 1) as f32
    } else {
        duration
    };
    let n_blocks = total_frames.div_ceil(MAX_FRAMES_PER_BLOCK);
    let block_duration = duration / n_blocks as f32;
    let block_inverse_duration = 1.0 / block_duration;

    // maskAndQuantizationSize: n_transforms × 4 bytes, padded to 4
    let mask_and_quantization_size = (n_transforms * 4 + 3) & !3;

    let mut data: Vec<u8> = Vec::new();
    let mut block_offsets: Vec<u32> = Vec::with_capacity(n_blocks);
    let mut float_block_offsets: Vec<u32> = Vec::with_capacity(n_blocks);
    let mut transform_offsets: Vec<u32> = Vec::with_capacity(n_transforms * n_blocks);
    for block_idx in 0..n_blocks {
        let frame_start = block_idx * MAX_FRAMES_PER_BLOCK;
        let frame_end = (frame_start + MAX_FRAMES_PER_BLOCK).min(total_frames);
        let frames_in_block = frame_end - frame_start;

        // Slice transforms for this block: [track + frame * n_transforms]
        // Rearrange to contiguous block slice
        let block_transforms: Vec<_> = (frame_start..frame_end)
            .flat_map(|fi| (0..n_transforms).map(move |ti| ti + fi * n_transforms))
            .map(|idx| &temp_anim.m_transforms[idx])
            .collect();

        block_offsets.push(data.len() as u32);

        let block = write_block(&block_transforms, n_transforms, frames_in_block, &params);

        // Adjust transform offsets to be relative to data start
        let base = data.len() as u32;
        for off in &block.transform_offsets {
            transform_offsets.push(base + off);
        }

        float_block_offsets.push(base + block.float_block_offset);

        data.extend_from_slice(&block.data);
    }

    hkaSplineCompressedAnimation {
        __ptr: None,
        parent: temp_anim.parent,
        m_numFrames: total_frames as i32,
        m_numBlocks: n_blocks as i32,
        m_maxFramesPerBlock: MAX_FRAMES_PER_BLOCK as i32,
        m_maskAndQuantizationSize: mask_and_quantization_size as i32,
        m_blockDuration: block_duration,
        m_blockInverseDuration: block_inverse_duration,
        m_frameDuration: frame_duration,
        m_blockOffsets: block_offsets,
        m_floatBlockOffsets: float_block_offsets,
        m_transformOffsets: transform_offsets,
        m_floatOffsets: Vec::with_capacity(n_floats * n_blocks),
        m_data: data,
        m_endian: 0,
    }
}

fn export<'a>(
    seq: &bridge::ffi::NiControllerSequenceRef,
    block: &hkaSkeleton<'a>,
    binding: &mut hkaAnimationBinding<'a>,

    config: &Config,
) -> Result<hkaSplineCompressedAnimation<'a>, KfSerError> {
    let s = bridge::ffi::get_target_name(seq)
        .to_string_lossy()
        .to_string();

    binding.m_originalSkeletonName = Some(Cow::Owned(s)).into();

    export_controller(seq, block, binding, config)
}

/// Apply kf to hkx
///
/// # Arguments
/// * `bytes` - The input HKX file contents as bytes.
/// * `text`: new String(To avoid XML ownership error.)
/// * `input_path` - The file path of the input HKX file. This is used for error reporting.
/// * `output_format` - The desired output format for the HKX file (e.g., JSON, TOML, YAML).
/// * `animations` - A list of file paths to animation files (.kf) that should be merged into the output HKX.
///
/// # Errors
pub fn to_hkx<'a>(
    bytes: &'a Vec<u8>,
    text: &'a mut String,
    input_path: &Path,
    output_format: Format,

    animations: Vec<PathBuf>,
    config: &Config,
) -> Result<Vec<u8>, KfSerError> {
    let mut class_map: ClassMap<'a> =
        crate::serde::de::deserialize(bytes, text, input_path).context(SerdeHkxFeatureSnafu)?;
    export_animations(&mut class_map, animations, config)?;

    let output = crate::serde::ser::to_bytes(input_path, output_format, &mut class_map)
        .context(SerdeHkxFeatureSnafu)?;
    Ok(output)
}

fn export_animations<'a>(
    class_map: &mut ClassMap<'a>,
    anim_list: Vec<PathBuf>, // animations/.hkx paths
    config: &Config,
) -> Result<(), KfSerError> {
    let new_binding_ptr = 9999.into();

    {
        // Find one `hkRootLevelContainer`
        let (_ptr, root_container) = {
            let mut root_container: Vec<_> = class_map
                .into_par_iter()
                .filter(|(_, class)| matches!(class, Classes::hkRootLevelContainer(_)))
                .collect();

            match root_container.len() {
                1 => root_container.swap_remove(0),
                0 => return Err(KfSerError::MissingAnimationContainer),
                _ => {
                    return Err(KfSerError::MultipleAnimationContainerFound {
                        count: root_container.len(),
                    });
                }
            }
        };
        let Classes::hkRootLevelContainer(anim) = root_container else {
            return Err(KfSerError::MissingAnimationContainer);
        };
        anim.m_namedVariants
            .push(havok_classes::hkRootLevelContainerNamedVariant {
                __ptr: None,
                m_name: Some(Cow::Borrowed("Merged Animation Container")).into(),
                m_className: Some(Cow::Borrowed("hkaAnimationContainer")).into(),
                m_variant: new_binding_ptr,
            });
    }

    let (anim_container_ptr, skelton_a_ptr) = {
        // Find one `hkaAnimationContainer`
        let (&anim_container_ptr, anim_container) = {
            let mut anim_containers: Vec<_> = class_map
                .into_par_iter()
                .filter(|(_, class)| matches!(class, Classes::hkaAnimationContainer(_)))
                .collect();

            match anim_containers.len() {
                1 => anim_containers.swap_remove(0),
                0 => return Err(KfSerError::MissingAnimationContainer),
                _ => {
                    return Err(KfSerError::MultipleAnimationContainerFound {
                        count: anim_containers.len(),
                    });
                }
            }
        };
        let Classes::hkaAnimationContainer(anim) = anim_container else {
            return Err(KfSerError::MissingAnimationContainer);
        };

        anim.m_bindings.push(new_binding_ptr);

        (
            anim_container_ptr,
            anim.m_skeletons
                .first()
                .ok_or(KfSerError::EmptySkeletons)?
                .get(),
        )
    };
    let Some(Classes::hkaSkeleton(skelton_a)) = class_map.get_mut(&skelton_a_ptr) else {
        return Err(KfSerError::EmptySkeletons);
    };

    let mut new_binding = hkaAnimationBinding {
        __ptr: Some(new_binding_ptr),
        ..Default::default()
    };

    let mut anim_indexes = vec![];
    let mut animations = vec![];
    for anim_file in anim_list {
        cxx::let_cxx_string!(s = anim_file.to_string_lossy().to_string());
        let blocks = bridge::ffi::read_nif_list(&s)
            .with_context(|_| FailedToReadNifSnafu { path: anim_file })?;

        for (index, block) in blocks.iter().enumerate() {
            let new_index = 9998 - index;
            let new_ptr = Pointer::new(new_index);

            let mut new_anim = export(block, skelton_a, &mut new_binding, config)?;
            new_anim.__ptr = Some(new_ptr);
            animations.push((new_index, Classes::hkaSplineCompressedAnimation(new_anim)));
            anim_indexes.push(new_ptr);
        }
    }

    class_map.par_extend(animations);

    {
        let Some(Classes::hkaAnimationContainer(anim)) = class_map.get_mut(&anim_container_ptr)
        else {
            return Err(KfSerError::MissingAnimationContainer);
        };
        anim.m_animations.par_extend(anim_indexes);
    }

    Ok(())
}

#[derive(Debug, snafu::Snafu)]
pub enum KfSerError {
    /// Raised when the HKX data could not be parsed into a valid ClassMap.
    #[snafu(display("internal serde_hkx_features err: {source}"))]
    SerdeHkxFeatureError { source: crate::error::Error },

    /// Raised when the HKX data could not be parsed into a valid ClassMap.
    #[snafu(display("ReadNifList error occurred. path: `{}`, cpp_error: {source}", path.display()))]
    FailedToReadNif {
        path: PathBuf,
        source: cxx::Exception,
    },

    /// No `hkaAnimationContainer` class found
    MissingAnimationContainer,

    /// `hkAnimationContainer` was found, but `m_skeletons: Vec<Pointer>` was empty. (This requires at least one.)
    EmptySkeletons,

    /// Multiple hkaAnimationContainer classes found.
    #[snafu(display(
        "expected one `hkaAnimationContainer` per `hkx`, but multiple were obtained. Got count: {count}"
    ))]
    MultipleAnimationContainerFound { count: usize },

    /// Raised when an unsupported I32 variant was encountered.
    #[snafu(display("Unsupported i32 in animation field: {variant}"))]
    UnsupportedI32Variant { variant: String },

    /// Raised when file IO fails.
    #[snafu(display("Failed to read file: {source}"))]
    IoError { source: std::io::Error },

    /// Missing bone name: {missing_name}
    MissingBoneName { missing_name: String },
}
