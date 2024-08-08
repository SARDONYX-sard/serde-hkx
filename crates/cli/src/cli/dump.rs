use crate::error::{Error, FailedReadFileSnafu, Result};
use serde_hkx::bytes::hexdump;
use snafu::ResultExt as _;
use std::path::Path;
use tokio::fs;

/// ANSI color representation command examples.
pub const EXAMPLES: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>hkx -> hexdump -> stdout</blue!>
  <cyan!>hkxc dump</cyan!> ./defaultmale.hkx

- <blue!>hkx -> hexdump -> a file</blue!>
  <cyan!>hkxc dump</cyan!> ./defaultmale.hkx <cyan!>-o</cyan!> hexdump.txt

 - <blue!>hexdump -> hkx -> a file</blue!><yellow>(NOTE: For reverse conversion, use `-o` to specify the output path.)</yellow>
  <cyan!>hkxc dump</cyan!> hexdump.txt <cyan!>-o</cyan!> ./defaultmale.hkx -r
"#
);

#[derive(Debug, clap::Args)]
#[clap(arg_required_else_help = true, after_long_help = EXAMPLES)]
pub(crate) struct Args {
    /// hkx file path/utf8 hexdump file path(when --reverse is enabled)
    pub input: String,
    /// If specified, write to a file (If not specified, stdout)
    #[clap(short, long)]
    #[clap(long_help = "Output path to write a file.
NOTE: Bytes cannot be written to stdout, so always use `-o` to specify the output when use `--reverse`.")]
    pub output: Option<String>,

    /// Reverse conversion from hexdump to bytes
    #[clap(short, long)]
    pub reserve: bool,
}

/// Output bytes to stdout/file.
pub async fn to_bytes<I, O>(input: I, output: Option<O>) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    match output {
        Some(output) => {
            let input = input.as_ref();
            let bytes = hexdump::to_bytes(&fs::read_to_string(input).await.context(
                FailedReadFileSnafu {
                    path: input.to_path_buf(),
                },
            )?);
            fs::write(output, &bytes).await?
        }
        None => return Err(Error::InvalidStdout),
    };
    Ok(())
}

/// Output hexdump to stdout/file.
pub async fn to_string<I, O>(input: I, output: Option<O>) -> Result<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = input.as_ref();
    let hexdump = hexdump::to_string(fs::read(input).await.context(FailedReadFileSnafu {
        path: input.to_path_buf(),
    })?); // NOTE: With newline.
    match output {
        Some(output) => fs::write(output, &hexdump).await?,
        None => print!("{hexdump}"),
    };
    Ok(())
}
