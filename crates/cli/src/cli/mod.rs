mod commands;
mod convert;

use crate::cli::commands::Commands;
use crate::init_tracing;
use std::str::FromStr;
use tracing::Level;

/// Converter CLI version
#[derive(Debug, clap::Parser)]
#[clap(about)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

macro_rules! global_init_logger {
    ($args:ident) => {
        init_tracing(
            $args.log_file,
            Level::from_str(&$args.log_level).unwrap_or(Level::ERROR),
            $args.stdout,
        )?;
    };
}

pub(crate) async fn run_cli(args: Cli) -> anyhow::Result<()> {
    match args.command {
        Commands::Convert(args) => {
            global_init_logger!(args);
            convert::convert(&args.input, args.output, args.format).await?;
        }
    }

    Ok(())
}
