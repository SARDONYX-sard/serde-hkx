//! Convert Havok HKX animation to Gamebryo KF animation.
//!
//! This is the inverse of `to_hkx`: reads `hkaSplineCompressedAnimation` from
//! a class map and writes one `.kf` file per animation binding found.

mod bridge;
pub(crate) mod de_spline;
mod info_ver;
mod sampling;

use std::path::{Path, PathBuf};

use havok_classes::{Classes, hkaAnimationBinding, hkaSkeleton, hkaSplineCompressedAnimation};
use snafu::ResultExt as _;

use self::info_ver::NifVersion;
use crate::ClassMap;

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// Conversion settings – mirrors the CLI switches of the original `ExportKF` command.
///
/// | C++ switch | Field                 | Default    |
/// |------------|-----------------------|------------|
/// | `-l`       | `export_float_tracks` | `false`    |
/// | `-v x.x`   | `nif_version`         | 20.2.0.7   |
/// | `-u x`     | `user_version`        | `11`       |
/// | `-u2 x`    | `user_version2`       | `83`       |
/// | `-n`       | `recursive`           | `true`     |
#[derive(Debug, Clone)]
pub struct Config {
    /// Export float tracks in addition to transform tracks (`-l`).
    pub export_float_tracks: bool,
    /// NIF format version to write (`-v`).
    pub nif_version: NifVersion,
    /// NIF user version (`-u`).  Skyrim = 11, Skyrim SE = 12.
    pub user_version: u32,
    /// NIF user version 2 (`-u2`).  Skyrim / SE = 83.
    pub user_version2: u32,
    /// Recurse into subdirectories when a folder is given as input (`-n` disables).
    pub recursive: bool,
    /// Frame rate used when sampling the spline animation.
    pub frames_per_second: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            export_float_tracks: false,
            nif_version: NifVersion::Ver20_2_0_7,
            user_version: 11,
            user_version2: 83,
            recursive: true,
            frames_per_second: 30.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Public entry point  (mirrors `ExportAnimations`)
// ---------------------------------------------------------------------------

/// Convert one or more HKX animation files to KF, given an explicit skeleton.
///
/// Mirrors `ExportAnimations(rootdir, skelfile, animlist, outdir, nifver)`.
///
/// - `skeleton_path` – Havok skeleton HKX.
/// - `anim_paths`    – Havok animation HKX files to convert.
/// - `output`        – Explicit `.kf` path (only when `anim_paths.len() == 1`) **or** a directory. When it is a directory the output
/// - `root_dir`      – Root used to compute relative paths
/// - `config`        – Conversion settings.
///
/// Returns every `.kf` path that was written successfully.
fn export_animations(
    skeleton_path: &Path,
    anim_paths: &[PathBuf],
    output: &Path,
    root_dir: &Path,
    config: &Config,
) -> Result<Vec<PathBuf>, KfDeError> {
    if anim_paths.is_empty() {
        return Err(KfDeError::NoAnimations);
    }

    let output_is_kf = output
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("kf"));

    if output_is_kf && anim_paths.len() > 1 {
        return Err(KfDeError::ExplicitKfWithMultipleInputs);
    }

    // Load skeleton once and reuse for every animation.
    let skel_bytes = std::fs::read(skeleton_path).with_context(|_| IoSnafu {
        path: skeleton_path.to_owned(),
    })?;
    let mut skel_text = String::new();
    let skel_class_map = crate::serde::de::deserialize(&skel_bytes, &mut skel_text, skeleton_path)
        .context(SerdeHkxFeatureSnafu)?;
    let skeleton = find_skeleton(&skel_class_map)?;

    let mut written = Vec::new();

    for anim_path in anim_paths {
        let out_path = if output_is_kf {
            // Explicit single output path.
            output.to_owned()
        } else {
            // Mirror C++ PathRelativePathTo + PathCombine logic:
            // place the .kf under `output`, preserving the path relative to `root_dir`.
            let rel = anim_path
                .strip_prefix(root_dir)
                .unwrap_or_else(|_| anim_path.file_name().map_or(anim_path, Path::new));
            output.join(rel).with_extension("kf")
        };

        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).with_context(|_| IoSnafu {
                    path: parent.to_owned(),
                })?;
            }
        }

        match export_one_file(skeleton, anim_path, &out_path, config) {
            Ok(paths) => {
                for p in &paths {
                    tracing::info!("Exported '{}'", p.display());
                }
                written.extend(paths);
            }
            Err(e) => {
                tracing::error!("Export failed for '{}': {e}", anim_path.display());
            }
        }
    }

    Ok(written)
}

// ---------------------------------------------------------------------------
// Project-level helper  (mirrors `ExportProject`)
// ---------------------------------------------------------------------------

/// Locate the skeleton and animation files for a project directory or
/// `skeleton.hkx` path, then call [`export_animations`].
///
/// Mirrors `ExportProject(projfile, rootPath, outdir, nifver, recursion)`.
///
/// # Errors
/// - `KfDeError::NoSkeleton` if no skeleton file is found.
/// - `KfDeError::MultipleSkeletons` if multiple skeleton files are found.
/// - `KfDeError::NoAnimations` if no animation files are found.
pub fn export_project(
    proj_path: &Path,
    root_path: &Path,
    output_dir: &Path,
    config: &Config,
) -> Result<Vec<PathBuf>, KfDeError> {
    use std::ffi::OsStr;

    let is_skeleton = proj_path
        .file_name()
        .and_then(OsStr::to_str)
        .is_some_and(|n| n.eq_ignore_ascii_case("skeleton.hkx"));

    let (skel_files, anim_files) = if is_skeleton {
        // proj_path IS the skeleton; look for animations in `../animations/`.
        let anim_dir = proj_path
            .parent()
            .unwrap_or(proj_path)
            .parent()
            .unwrap_or(proj_path)
            .join("animations");
        let anims = collect_hkx_files(&anim_dir, config.recursive);
        (vec![proj_path.to_owned()], anims)
    } else {
        // proj_path is a project file; look for skeleton in `character assets/`
        // and animations in `animations/`.
        let base = proj_path.parent().unwrap_or(proj_path);
        let skel_dir = base.join("character assets");
        let anim_dir = base.join("animations");

        let skels = collect_hkx_files(&skel_dir, config.recursive)
            .into_iter()
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.to_ascii_lowercase().contains("skeleton"))
            })
            .collect::<Vec<_>>();

        let anims = collect_hkx_files(&anim_dir, config.recursive);
        (skels, anims)
    };

    match skel_files.len() {
        0 => {
            tracing::warn!("No skeletons found. Skipping '{}'", proj_path.display());
            return Ok(vec![]);
        }
        n if n > 1 => {
            tracing::warn!(
                "Multiple skeletons found. Skipping '{}'",
                proj_path.display()
            );
            return Ok(vec![]);
        }
        _ => {}
    }

    if anim_files.is_empty() {
        tracing::warn!("No animations found. Skipping '{}'", proj_path.display());
        return Ok(vec![]);
    }

    export_animations(&skel_files[0], &anim_files, output_dir, root_path, config)
}

// ---------------------------------------------------------------------------
// Per-file helper
// ---------------------------------------------------------------------------

/// Read one HKX animation and write the corresponding `.kf` file(s).
fn export_one_file(
    skeleton: &hkaSkeleton,
    anim_path: &Path,
    out_path: &Path,
    config: &Config,
) -> Result<Vec<PathBuf>, KfDeError> {
    let anim_bytes = std::fs::read(anim_path).with_context(|_| IoSnafu {
        path: anim_path.to_owned(),
    })?;
    let mut anim_text = String::new();
    let anim_class_map = crate::serde::de::deserialize(&anim_bytes, &mut anim_text, anim_path)
        .context(SerdeHkxFeatureSnafu)?;

    to_kf(&anim_class_map, skeleton, anim_path, out_path, config)
}

// ---------------------------------------------------------------------------
// Core conversion  (mirrors `AnimationExport::doExport` + `exportController`)
// ---------------------------------------------------------------------------

/// Convert a parsed HKX class map to one `.kf` file.
///
/// This is the innermost function that all public entry points eventually call.
/// The NIF write step is a placeholder until the bridge FFI is complete.
///
/// # Errors
/// - `KfDeError::NoSkeleton` if no `hkaSkeleton` is found in the class map.
pub fn to_kf(
    class_map: &ClassMap,
    skeleton: &hkaSkeleton,
    anim_path: &Path,
    out_path: &Path,
    config: &Config,
) -> Result<Vec<PathBuf>, KfDeError> {
    let pairs = collect_binding_anim_pairs(class_map)?;

    if pairs.is_empty() {
        return Err(KfDeError::NoAnimationBindings);
    }

    // The C++ code rejects files with ≠ 1 binding.
    if pairs.len() != 1 {
        return Err(KfDeError::UnexpectedBindingCount { count: pairs.len() });
    }

    let (binding, anim) = &pairs[0];

    let num_tracks = anim.parent.m_numberOfTransformTracks as usize;
    if num_tracks == 0 {
        return Err(KfDeError::ZeroTransformTracks);
    }
    if num_tracks > skeleton.m_bones.len() {
        return Err(KfDeError::TrackCountExceedsBones {
            tracks: num_tracks,
            bones: skeleton.m_bones.len(),
        });
    }

    // Derive the sequence name from the animation file stem
    // (mirrors `_splitpath(animfile, …, fname, …)`).
    let seq_name = anim_path.file_stem().unwrap_or_default().to_string_lossy();

    // Skeleton root name for NiControllerSequence::SetTargetName
    let target_name = binding
        .m_originalSkeletonName
        .get_ref()
        .as_deref()
        .unwrap_or("");

    // Sample spline animation → per-track key lists.
    let sampled = sampling::sample_animation(skeleton, binding, anim, config.export_float_tracks)
        .context(SamplingSnafu)?;

    bridge::write_kf(out_path, &seq_name, target_name, &sampled, config).map_err(|e| {
        KfDeError::WriteError {
            path: out_path.to_owned(),
            source: e,
        }
    })?;

    Ok(vec![out_path.to_owned()])
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn find_skeleton<'a>(class_map: &'a ClassMap) -> Result<&'a hkaSkeleton<'a>, KfDeError> {
    let mut found: Vec<_> = class_map
        .values()
        .filter_map(|c| {
            if let Classes::hkaSkeleton(s) = c {
                Some(s)
            } else {
                None
            }
        })
        .collect();

    match found.len() {
        1 => Ok(found.swap_remove(0)),
        0 => Err(KfDeError::NoSkeleton),
        n => Err(KfDeError::MultipleSkeletons { count: n }),
    }
}

fn collect_binding_anim_pairs<'a>(
    class_map: &'a ClassMap,
) -> Result<
    Vec<(
        &'a hkaAnimationBinding<'a>,
        &'a hkaSplineCompressedAnimation<'a>,
    )>,
    KfDeError,
> {
    // Index spline animations by pointer value for O(1) lookup.
    let anim_index: std::collections::HashMap<usize, &hkaSplineCompressedAnimation> = class_map
        .iter()
        .filter_map(|(&ptr, c)| {
            if let Classes::hkaSplineCompressedAnimation(a) = c {
                Some((ptr, a))
            } else {
                None
            }
        })
        .collect();

    let pairs = class_map
        .values()
        .filter_map(|c| {
            if let Classes::hkaAnimationBinding(b) = c {
                let ptr = b.m_animation.get();
                anim_index.get(&ptr).map(|a| (b, *a))
            } else {
                None
            }
        })
        .collect();

    Ok(pairs)
}

/// Recursively (or not) collect all `.hkx` files under `dir`.
fn collect_hkx_files(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };

    let mut files = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && recursive {
            files.extend(collect_hkx_files(&path, recursive));
        } else if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("hkx"))
        {
            files.push(path);
        }
    }
    files
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, snafu::Snafu)]
pub enum KfDeError {
    #[snafu(display("internal serde_hkx error: {source}"))]
    SerdeHkxFeatureError { source: crate::error::Error },

    #[snafu(display("sampling error: {source}"))]
    SamplingError { source: sampling::SamplingError },

    #[snafu(display("I/O error for '{}': {source}", path.display()))]
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },

    /// No `hkaSkeleton` found in the HKX file.
    NoSkeleton,

    /// Multiple skeletons found; expected exactly one.
    #[snafu(display("expected one hkaSkeleton, found {count}"))]
    MultipleSkeletons { count: usize },

    /// No `hkaAnimationBinding` entries found.
    NoAnimationBindings,

    /// Expected exactly one binding per file (matches C++ behaviour).
    #[snafu(display("expected 1 hkaAnimationBinding, found {count}"))]
    UnexpectedBindingCount { count: usize },

    /// The animation has no transform tracks.
    ZeroTransformTracks,

    /// Track count exceeds skeleton bone count.
    #[snafu(display(
        "number of transform tracks ({tracks}) exceeds skeleton bone count ({bones})"
    ))]
    TrackCountExceedsBones { tracks: usize, bones: usize },

    /// No animation files were provided.
    NoAnimations,

    /// An explicit `.kf` output path was given with multiple input animations.
    ExplicitKfWithMultipleInputs,

    /// Error writing the `.kf` file.
    #[snafu(display("error writing '{}': {source}", path.display()))]
    WriteError {
        path: PathBuf,
        source: cxx::Exception,
    },
}
