mod cli;
#[cfg(feature = "color")]
mod color;
mod error;
mod logger;

use crate::cli::{run_cli, Cli};
use crate::logger::init_tracing;
use clap::Parser;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();

    match run_cli(Cli::parse()).await {
        Ok(()) => {
            let elapsed = start.elapsed();
            tracing::info!(
                "Elapsed time: {}.{}secs.",
                elapsed.as_secs(),
                elapsed.subsec_millis()
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!("{err}");
            anyhow::bail!("{err}")
        }
    }
}
