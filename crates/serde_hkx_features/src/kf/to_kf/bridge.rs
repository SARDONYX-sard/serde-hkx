//! FFI bridge for NIF write operations (niflib via C++).
//!
//! Read operations reuse the existing `to_hkx::bridge`; this module adds
//! the write side needed by `from_hkx`.

use super::{Config, sampling::SampledAnimation};

#[cxx::bridge(namespace = "nifbridge_write")]
pub(crate) mod ffi {
    /// One translate/rotate/scale key set for a single transform track.
    #[derive(Debug, Clone)]
    pub struct TransformTrackFfi {
        pub bone_name: String,
        pub translate_keys: Vec<Vector3KeyFfi>,
        pub rotate_keys: Vec<QuaternionKeyFfi>,
        pub scale_keys: Vec<FloatKeyFfi>,
    }

    #[derive(Debug, Clone)]
    pub struct FloatTrackFfi {
        pub slot_name: String,
        pub keys: Vec<FloatKeyFfi>,
    }

    #[derive(Debug, Clone)]
    pub struct Vector3KeyFfi {
        pub time: f32,
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Debug, Clone)]
    pub struct QuaternionKeyFfi {
        pub time: f32,
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }

    #[derive(Debug, Clone)]
    pub struct FloatKeyFfi {
        pub time: f32,
        pub data: f32,
    }

    unsafe extern "C++" {
        include!("wrapper_write.h");

        /// Write a `.kf` file.
        ///
        /// - `out_path`        – destination file path
        /// - `seq_name`        – `NiControllerSequence::m_name`
        /// - `target_name`     – skeleton root name (`binding->m_originalSkeletonName`)
        /// - `duration`        – animation duration in seconds
        /// - `transform_tracks`– one entry per transform track
        /// - `float_tracks`    – one entry per float track (may be empty)
        /// - `nif_version`     – NIF format version (u32 from `NifVersion`)
        /// - `user_version`    – e.g. 11 for Skyrim
        /// - `user_version2`   – e.g. 83 for Skyrim/SE
        #[allow(clippy::too_many_arguments)]
        fn write_kf(
            out_path: &str,
            seq_name: &str,
            target_name: &str,
            duration: f32,
            transform_tracks: Vec<TransformTrackFfi>,
            float_tracks: Vec<FloatTrackFfi>,
            nif_version: u32,
            user_version: u32,
            user_version2: u32,
        ) -> Result<()>;
    }
}

// ---------------------------------------------------------------------------
// Rust-side adapter
// ---------------------------------------------------------------------------

/// Convert [`SampledAnimation`] to FFI types and call the C++ writer.
pub fn write_kf(
    out_path: &std::path::Path,
    seq_name: &str,
    target_name: &str,
    sampled: &SampledAnimation,
    config: &Config,
) -> Result<(), cxx::Exception> {
    use ffi::{FloatKeyFfi, FloatTrackFfi, QuaternionKeyFfi, TransformTrackFfi, Vector3KeyFfi};
    use rayon::prelude::*;

    let transform_tracks: Vec<TransformTrackFfi> = sampled
        .transform_tracks
        .par_iter()
        .map(|t| TransformTrackFfi {
            bone_name: t.bone_name.clone(),
            translate_keys: t
                .translate
                .iter()
                .map(|k| Vector3KeyFfi {
                    time: k.time,
                    x: k.x,
                    y: k.y,
                    z: k.z,
                })
                .collect(),
            rotate_keys: t
                .rotate
                .iter()
                .map(|k| QuaternionKeyFfi {
                    time: k.time,
                    x: k.x,
                    y: k.y,
                    z: k.z,
                    w: k.w,
                })
                .collect(),
            scale_keys: t
                .scale
                .iter()
                .map(|k| FloatKeyFfi {
                    time: k.time,
                    data: k.data,
                })
                .collect(),
        })
        .collect();

    let float_tracks: Vec<FloatTrackFfi> = sampled
        .float_tracks
        .iter()
        .map(|ft| FloatTrackFfi {
            slot_name: ft.slot_name.clone(),
            keys: ft
                .keys
                .iter()
                .map(|k| FloatKeyFfi {
                    time: k.time,
                    data: k.data,
                })
                .collect(),
        })
        .collect();

    // cxx does not allow constructing CxxVector on the Rust side directly;
    // pass slices and let C++ consume them.
    // Until cxx supports Vec<T> → CxxVector<T> automatically for non-trivial types,
    // we use the generated UniquePtr path.  The actual call is forwarded to C++
    // via a helper that accepts raw slices.
    ffi::write_kf(
        out_path.to_str().unwrap_or_default(),
        seq_name,
        target_name,
        sampled.duration,
        transform_tracks,
        float_tracks,
        config.nif_version as u32,
        config.user_version,
        config.user_version2,
    )
}
