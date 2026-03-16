//! Convert Havok HKX animation to Gamebryo KF animation.
use serde_hkx_features::{
    error::Result,
    kf::to_kf::info_ver::NifVersion,
    kf::to_kf::{Config, export_animations, export_project},
};
use std::path::PathBuf;

pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>skeleton + animation -> kf</blue!>
  <cyan!>hkxc to-kf -s</cyan!> ./skeleton.hkx <cyan!>-a</cyan!> ./animations/idle.hkx <cyan!>-o</cyan!> ./out/
- <blue!>project directory -> kf (auto-discover skeleton + animations)</blue!>
  <cyan!>hkxc to-kf</cyan!> ./characters/defaultmale/
- <blue!>export float tracks, Skyrim SE versions</blue!>
  <cyan!>hkxc to-kf -s</cyan!> ./skeleton.hkx <cyan!>-a</cyan!> ./idle.hkx <cyan!>-o</cyan!> ./out/ <cyan!>--float-tracks --user-version</cyan!> 12
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// Project directory (auto-discovers skeleton.hkx + animations/*.hkx),
    /// OR explicit skeleton path when used with --anim.
    #[clap(value_name = "DIR_OR_SKEL")]
    pub input: Option<PathBuf>,

    /// Skeleton HKX path (required when --anim is specified).
    #[clap(short = 's', long, value_name = "SKELETON")]
    pub skeleton: Option<PathBuf>,

    /// One or more animation HKX files to convert.
    #[clap(short = 'a', long, value_name = "ANIM", num_args = 1..)]
    pub anim: Vec<PathBuf>,

    /// Output directory (or explicit .kf path when a single animation is given).
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Export float tracks in addition to transform tracks.
    #[clap(long)]
    pub float_tracks: bool,

    /// Do not recurse into subdirectories.
    #[clap(short = 'n', long)]
    pub no_recursive: bool,

    /// NIF version string to write (default: 20.2.0.7).
    #[clap(long, default_value = "20.2.0.7")]
    pub nif_version: NifVersion,

    /// NIF user version (default: 11 = Skyrim LE).
    #[clap(short = 'u', long, default_value = "11")]
    pub user_version: u32,

    /// NIF user version 2 (default: 83).
    #[clap(long = "user-version2", default_value = "83")]
    pub user_version2: u32,
}

pub fn to_kf(args: &Args) -> Result<()> {
    let config = Config {
        export_float_tracks: args.float_tracks,
        nif_version: args.nif_version,
        user_version: args.user_version,
        user_version2: args.user_version2,
        recursive: !args.no_recursive,
        frames_per_second: 30.0,
    };

    // Explicit skeleton + animation(s) mode.
    if let Some(skel) = &args.skeleton {
        if args.anim.is_empty() {
            return Err(serde_hkx_features::error::Error::IoError {
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--skeleton requires at least one --anim path",
                ),
            });
        }

        let output = args
            .output
            .clone()
            .unwrap_or_else(|| skel.parent().unwrap_or(skel).to_owned());

        // root_dir = directory of the first anim (for relative-path calculation)
        let root_dir = args.anim[0]
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_owned();

        let written = export_animations(skel, &args.anim, &output, &root_dir, &config)?;

        for p in &written {
            tracing::info!("Exported '{}'", p.display());
        }
        return Ok(());
    }

    // Project directory / skeleton.hkx auto-discovery mode.
    let input = match &args.input {
        Some(p) => p.clone(),
        None => {
            return Err(serde_hkx_features::error::Error::IoError {
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Provide a project directory or use --skeleton / --anim",
                ),
            });
        }
    };

    let output = args.output.clone().unwrap_or_else(|| input.clone());

    let root = input.canonicalize().unwrap_or_else(|_| input.clone());

    let written = export_project(&input, &root, &output, &config)?;

    for p in &written {
        tracing::info!("Exported '{}'", p.display());
    }

    Ok(())
}
