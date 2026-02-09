mod color;
mod convert;
mod diff;
mod dump;
mod progress_handler;
mod tree;
mod verify;

#[cfg(feature = "color")]
use self::color::get_styles;
use self::convert::{convert, tokio_convert};
use crate::logger::LogLevel;
use clap::CommandFactory as _;
use serde_hkx_features::{
    convert::Format,
    diff::write_diff,
    dump as hexdump,
    error::{Error, Result},
    tree::write_tree,
};
use std::{io, path::PathBuf};

pub(crate) async fn run(args: Args) -> Result<()> {
    crate::logger::init(args.log_file, args.log_level, args.stdout)?;

    // For drag & drop
    if let Some(input) = args.input {
        let out_fmt = Format::infer_output_from_input(&input)?;
        let output: Option<PathBuf> = None;
        return tokio_convert(input, output, out_fmt).await;
    }

    if let Some(command) = args.subcommand {
        match command {
            SubCommands::Convert(args) => {
                convert(&args.input, args.output, args.format, args.runtime).await
            }
            SubCommands::Tree(args) => write_tree(args.input, args.output).await,
            SubCommands::Dump(args) => match args.reverse {
                true => hexdump::to_bytes(args.input, args.output).await,
                false => hexdump::to_string(args.input, args.output).await,
            },
            SubCommands::Diff(args) => {
                write_diff(args.old, args.new, args.output, args.color).await
            }
            SubCommands::Verify(args) => self::verify::verify(&args.path, args.color),
            SubCommands::Completions { shell } => {
                shell.generate(&mut Args::command(), &mut io::stdout());
                Ok(())
            }
        }
    } else {
        Args::command().print_long_help()?;
        pub const ERR_MSG: &str = color_print::cstr!(
            r#"
<red>error:</red> '<red>hkxc</red>' requires a subcommand or hkx/xml path.
For more information, try '<cyan!>--help</cyan!>'.
"#
        );
        Err(Error::IoError {
            source: io::Error::new(io::ErrorKind::InvalidInput, ERR_MSG),
        })
    }
}

/// CLI command arguments
#[derive(Debug, clap::Parser)]
#[clap(version, about, author)]
#[cfg_attr(feature = "color", clap(styles=get_styles()))]
#[clap(arg_required_else_help = true, args_conflicts_with_subcommands = true, after_long_help = convert::EXAMPLES)]
pub(crate) struct Args {
    /// The path of the target for drag and drop hkx(64bit) <-> xml conversion (The converted file will be created in the same location)
    pub input: Option<PathBuf>,

    #[clap(subcommand)]
    subcommand: Option<SubCommands>,

    // --logger (Global options)
    #[clap(global = true, long, display_order = 100)]
    /// Enable standard output of the log
    pub stdout: bool,
    #[clap(global = true, long, display_order = 101)]
    #[clap(ignore_case = true, default_value = "error")]
    /// Log level to be recorded in logger
    pub log_level: LogLevel,
    #[clap(global = true, long, display_order = 102)]
    /// Output path of log file
    pub log_file: Option<PathBuf>,
}

#[derive(Debug, clap::Parser)]
pub(crate) enum SubCommands {
    /// Convert hkx <-> xml
    Convert(convert::Args),

    /// Show dependency tree from havok behavior(hkx/xml file)
    Tree(tree::Args),

    /// Dump binary data in hexadecimal
    Dump(dump::Args),

    /// Show diff between two files.(In the case of `.hkx`, it is automatically converted to hexdump)
    Diff(diff::Args),

    /// Parallel hkx reproduction checks. If an error occurs, return a diff showing the location of each error.
    Verify(verify::Args),

    /// Generate shell completions
    #[clap(arg_required_else_help = true)]
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}
