mod cli;
#[cfg(feature = "color")]
mod error;
mod logger;

use crate::cli::Cli;
use clap::Parser;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    match cli::run(Cli::parse()).await {
        Ok(()) => {
            let elapsed = start.elapsed();
            let time = (elapsed.as_secs(), elapsed.subsec_millis());
            tracing::info!("Elapsed time: {}.{}secs.", time.0, time.1);
        }
        Err(err) => {
            tracing::error!("{err}");
            let err = color_print::cformat!("<red>[Error]\n{err}</red>");
            eprintln!("{err}")
        }
    }
}
