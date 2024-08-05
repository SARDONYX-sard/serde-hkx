mod color;
mod convert;
mod help;

#[cfg(feature = "color")]
use self::color::get_styles;
use crate::{error::Result, logger::LogLevel};
use clap::CommandFactory as _;

pub(crate) async fn run(args: Cli) -> Result<()> {
    match args.command {
        Commands::Convert(sub_args) => {
            crate::logger::init(args.log_file, args.log_level, args.stdout)?;
            convert::convert(&sub_args.input, sub_args.output, sub_args.format).await
        }
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
    #[clap(arg_required_else_help = true, after_long_help = help::AFTER_HELP)]
    Convert(convert::Convert),

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}
