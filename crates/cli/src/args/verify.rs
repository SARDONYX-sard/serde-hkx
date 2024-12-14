use crate::args::progress_handler::CliProgressHandler;
use serde_hkx_features::error::Result;
use std::path::{Path, PathBuf};

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"In case of error, dir and file behave differently
    - dir: All error paths are displayed
    - file: Error path + diff displayed

<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>Test the reproducibility of hkx</blue!>
  <cyan!>hkxc verify</cyan!> ./defaultmale.hkx

- <blue!>Test the reproducibility of hkx files</blue!>
  <cyan!>./hkxc verify</cyan!> ./input --log-level info --log-file "./verify_inputs.log"
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// hkx file path or dir for which you want to verify reproducibility. It is recommended to use log in INFO mode.
    pub path: PathBuf,
    /// Option to make the diff display colorful in case of error when file is specified.
    #[clap(long)]
    pub color: bool,
}

pub fn verify<P>(path: P, color: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    println!("Verifying...");
    serde_hkx_features::verify::verify(path, color, CliProgressHandler::new()).map(|_| {
        color_print::cprintln!(
            "<green>Complete hkx reproduction: {}</green>",
            path.display()
        );
    })
}

// NOTE: We don't try it except local PC because MIRI makes an error. Therefore, leave it commented out.
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "need templates"]
    #[test]
    fn test_verify_dir() {
        let input = "../../tests/input";
        verify(input, false).unwrap_or_else(|err| panic!("{err}"));
    }
}
