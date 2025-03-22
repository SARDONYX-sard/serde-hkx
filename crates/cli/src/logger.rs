use parse_display::{Display, FromStr};
use serde_hkx_features::error::Result;
use std::fs::File;
use std::path::Path;
use tracing::Level;

/// Log level.
#[derive(Debug, clap::ValueEnum, Clone, Copy, Default, PartialEq, Eq, Display, FromStr)]
pub enum LogLevel {
    #[display("trace")]
    Trace,
    #[display("debug")]
    Debug,
    #[display("info")]
    Info,
    #[display("warn")]
    Warn,
    #[default]
    #[display("error")]
    Error,
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
        match log_path {
            Some(log_path) => {
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
            }
            _ => {
                tracing::subscriber::set_global_default(
                    subscriber_builder
                        .pretty()
                        .with_ansi(true)
                        .with_line_number(true)
                        .with_target(false)
                        .finish(),
                )?;
            }
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
