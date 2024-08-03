use std::fs::File;
use std::path::Path;
use tracing::level_filters::LevelFilter;

#[derive(Debug, clap::ValueEnum, Clone, Copy, parse_display::Display, parse_display::FromStr)]
pub enum LogLevel {
    #[display("trace")]
    Trace,
    #[display("debug")]
    Debug,
    #[display("info")]
    Info,
    #[display("warn")]
    Warn,
    #[display("error")]
    Error,
}

/// Init logger.
pub(crate) fn init_tracing(
    log_path: Option<impl AsRef<Path>>,
    filter: impl Into<LevelFilter>,
    with_stdout: bool,
) -> anyhow::Result<()> {
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
        subscriber_builder.with_writer(log_file).init();
    }

    Ok(())
}
