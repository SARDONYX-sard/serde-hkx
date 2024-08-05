mod color;
mod convert;
mod tree;

#[cfg(feature = "color")]
use self::color::get_styles;
use crate::{error::Result, logger::LogLevel};
use clap::CommandFactory as _;
use havok_classes::Classes;

pub type ClassMap<'a> = indexmap::IndexMap<usize, Classes<'a>>;

pub(crate) async fn run(args: Cli) -> Result<()> {
    crate::logger::init(args.log_file, args.log_level, args.stdout)?;
    match args.command {
        Commands::Convert(sub_args) => {
            convert::convert(&sub_args.input, sub_args.output, sub_args.format).await
        }
        Commands::Tree(sub_args) => tree::print(sub_args.input).await,
        Commands::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
            Ok(())
        }
    }
}

#[derive(Debug, clap::Parser)]
#[clap(version, about, author)]
#[cfg_attr(feature = "color", command(styles=get_styles()))]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,

    // --logger (Global options)
    #[clap(global = true, long, display_order = 100)]
    /// ON/OFF whether logging is also output to standard output
    pub stdout: bool,
    #[clap(global = true, long, default_value = "error", display_order = 101)]
    /// Log level to be recorded in logger
    pub log_level: LogLevel,
    #[clap(global = true, long, display_order = 102)]
    /// Output path of log file
    pub log_file: Option<String>,
}

#[derive(Debug, clap::Parser)]
#[clap(version, about)]
pub(crate) enum Commands {
    /// Convert hkx <-> xml
    #[clap(arg_required_else_help = true, after_long_help = convert::EXAMPLES)]
    Convert(convert::Convert),

    /// Show dependency tree from havok behavior state machine (hkx/xml file)
    #[clap(arg_required_else_help = true, after_long_help = tree::EXAMPLES)]
    Tree(tree::Tree),

    /// Generate shell completions
    #[clap(arg_required_else_help = true)]
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}
