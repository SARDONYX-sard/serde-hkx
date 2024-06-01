//! Initializing a multithreaded logger in Rust.
//!
//! This library provides functionality to initialize a multithreaded logger with flexible configuration options.
//! It supports both file I/O logging and logging to stdout with ANSI color support.
pub mod builder;

#[doc(hidden)]
pub use tracing;

use std::io;
use tracing::subscriber::DefaultGuard;
use tracing_appender::non_blocking::WorkerGuard;

/// Multithread init logger.
///
/// # Features
/// - only console output.
/// - Level: TRACE
/// - output file: None
///
/// # Examples
/// ```
/// fn main() -> std:io::Result<()> {
///     let _guard = tracing_easy::init()?;
///     tracing::info!("Hey!");
///     Ok(())
/// }
/// ```
///
/// # Returns
/// Guards
/// - If this variable is dropped, the logger stops.
pub fn init() -> io::Result<(Option<WorkerGuard>, DefaultGuard)> {
    Ok(builder::LoggerBuilder::default().build()?)
}
