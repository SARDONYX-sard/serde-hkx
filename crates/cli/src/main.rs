pub mod args;
mod logger;

use crate::args::Args;
use clap::Parser;
use std::process::exit;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    match crate::args::run(Args::parse()).await {
        Ok(()) => {
            let elapsed = start.elapsed();
            let time = (elapsed.as_secs(), elapsed.subsec_millis());
            tracing::info!("Elapsed time: {}.{}secs.", time.0, time.1);
            exit(0);
        }
        Err(err) => {
            tracing::error!("{err}");
            let err = color_print::cformat!("<red>[Error]\n{err}</red>");
            eprintln!("{err}");
            exit(1);
        }
    }
}
