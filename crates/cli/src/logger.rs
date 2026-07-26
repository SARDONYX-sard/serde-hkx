use serde_hkx_features::error::Result;
use std::fs::File;
use std::path::Path;
use tracing::Level;

/// Log level.
#[derive(Debug, clap::ValueEnum, Clone, Copy, Default, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[default]
    Error,
}

impl core::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value = match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        };

        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLogLevelError;

impl core::fmt::Display for ParseLogLevelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid log level")
    }
}

impl core::error::Error for ParseLogLevelError {}

impl core::str::FromStr for LogLevel {
    type Err = ParseLogLevelError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.eq_ignore_ascii_case("trace") {
            Ok(Self::Trace)
        } else if value.eq_ignore_ascii_case("debug") {
            Ok(Self::Debug)
        } else if value.eq_ignore_ascii_case("info") {
            Ok(Self::Info)
        } else if value.eq_ignore_ascii_case("warn") {
            Ok(Self::Warn)
        } else if value.eq_ignore_ascii_case("error") {
            Ok(Self::Error)
        } else {
            Err(ParseLogLevelError)
        }
    }
}

impl From<LogLevel> for Level {
    #[inline]
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => Self::TRACE,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
        }
    }
}

/// Initialize loggers globally.
///
/// # Note
/// - This will live until the end of the program.
///
/// # Panics
/// Panics if called twice.
pub(crate) fn init<P, L>(log_path: Option<P>, filter: L, with_stdout: bool) -> Result<()>
where
    P: AsRef<Path>,
    L: Into<Level>,
{
    use tracing_subscriber::{fmt, layer::SubscriberExt};

    if let Some(log_parent) = log_path.as_ref().and_then(|p| p.as_ref().parent()) {
        std::fs::create_dir_all(log_parent)?;
    }

    let subscriber_builder = fmt::Subscriber::builder()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(filter.into())
        .with_target(false);

    if with_stdout {
        if let Some(log_path) = log_path {
            let log_file = File::create(log_path.as_ref())?;

            let log_file_config = fmt::Layer::default()
                .compact()
                .with_ansi(false)
                .with_file(true)
                .with_line_number(true)
                .with_target(false)
                .with_writer(log_file);
            tracing::subscriber::set_global_default(
                subscriber_builder.pretty().finish().with(log_file_config),
            )?;
        } else {
            tracing::subscriber::set_global_default(
                subscriber_builder
                    .pretty()
                    .with_ansi(true)
                    .with_line_number(true)
                    .with_target(false)
                    .finish(),
            )?;
        }
    } else if let Some(log_path) = log_path {
        let log_file = File::create(log_path.as_ref())?;
        subscriber_builder
            .with_writer(log_file)
            .with_ansi(false)
            .init();
    }

    Ok(())
}
