mod color;
mod convert;
mod diff;
mod dump;
mod tree;

#[cfg(feature = "color")]
use self::color::get_styles;
use crate::{
    error::{Error, Result},
    logger::LogLevel,
};
use clap::CommandFactory as _;
use convert::Format;
use havok_classes::Classes;
use std::{
    io,
    path::{Path, PathBuf},
};

pub type ClassMap<'a> = indexmap::IndexMap<usize, Classes<'a>>;

pub(crate) async fn run(args: Args) -> Result<()> {
    crate::logger::init(args.log_file, args.log_level, args.stdout)?;

    if let Some(input) = args.input {
        let input = input.as_path();
        return convert::convert::<&Path, PathBuf>(input, None, Format::from_input(input)).await;
    }

    if let Some(command) = args.subcommand {
        match command {
            SubCommands::Convert(args) => {
                convert::convert(&args.input, args.output, args.format).await
            }
            SubCommands::Tree(args) => tree::output(args.input, args.output).await,
            SubCommands::Dump(args) => {
                if args.reserve {
                    dump::to_bytes(args.input, args.output).await
                } else {
                    dump::to_string(args.input, args.output).await
                }
            }
            SubCommands::Diff(args) => diff::exec(args.old, args.new, args.output).await,
            SubCommands::Completions { shell } => {
                shell.generate(&mut Args::command(), &mut std::io::stdout());
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
    pub log_file: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub(crate) enum SubCommands {
    /// Convert hkx <-> xml
    Convert(convert::Args),

    /// Show dependency tree from havok behavior state machine (hkx/xml file)
    Tree(tree::Args),

    /// Dump binary data in hexadecimal
    Dump(dump::Args),

    /// Show diff between two files.
    Diff(diff::Args),

    /// Generate shell completions
    #[clap(arg_required_else_help = true)]
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}
