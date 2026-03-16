//! Apply Gamebryo KF animation to Havok HKX behavior.
use serde_hkx_features::{convert::Format, error::Result, kf, kf::from_kf::Config};
use std::path::PathBuf;

pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>Apply kf to hkx</blue!>
  <cyan!>hkxc from-kf -i</cyan!> ./defaultmale.hkx <cyan!>-a</cyan!> ./idle.kf <cyan!>-o</cyan!> ./defaultmale_patched.hkx
- <blue!>Additive blend</blue!>
  <cyan!>hkxc from-kf -i</cyan!> ./defaultmale.hkx <cyan!>-a</cyan!> ./additive.kf <cyan!>--additive</cyan!>
- <blue!>No root siblings, convert float tracks</blue!>
  <cyan!>hkxc from-kf -i</cyan!> ./defaultmale.hkx <cyan!>-a</cyan!> ./idle.kf <cyan!>--no-root-siblings --float-tracks</cyan!>
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// Input HKX behavior file to patch.
    #[clap(short, long)]
    pub input: PathBuf,

    /// One or more KF animation files to apply.
    #[clap(short = 'a', long, value_name = "KF", num_args = 1..)]
    pub anim: Vec<PathBuf>,

    /// Output HKX path (default: overwrite input).
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Output format for the HKX file.
    #[clap(short = 'v', long, ignore_case = true, default_value = "amd64")]
    pub format: Format,

    /// Exclude root sibling bones.
    #[clap(long)]
    pub no_root_siblings: bool,

    /// Apply as additive blend animation.
    #[clap(long)]
    pub additive: bool,

    /// Convert float tracks.
    #[clap(long)]
    pub float_tracks: bool,

    /// Frames per second for sampling.
    #[clap(long, default_value = "30.0")]
    pub fps: f32,
}

pub async fn from_kf(args: &Args) -> Result<()> {
    let config = Config {
        no_root_siblings: args.no_root_siblings,
        additive_blend: args.additive,
        convert_float_tracks: args.float_tracks,
        frames_per_second: args.fps,
        frames_increment: 1.0 / args.fps,
    };

    let output = args.output.clone().unwrap_or_else(|| args.input.clone());

    let bytes = std::fs::read(&args.input)
        .map_err(|e| serde_hkx_features::error::Error::IoError { source: e })?;
    let mut text = String::new();

    let result = kf::from_kf::from_kf(
        &bytes,
        &mut text,
        &args.input,
        args.format,
        args.anim.clone(),
        &config,
    )?;

    std::fs::write(&output, result)
        .map_err(|e| serde_hkx_features::error::Error::IoError { source: e })?;
    tracing::info!("Written '{}'", output.display());

    Ok(())
}
