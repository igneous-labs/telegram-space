use crate::envs::{DEFAULT_LOG_DIR, DEFAULT_RUST_LOG};
use flexi_logger::{style, DeferredNow, FileSpec, Level, Logger, Record, WriteMode};
use std::env;

// Error => Red
// Warn => Yellow
// Info => Aqua
// Debug => Pruple
// Trace => Grey
const LOG_COLOR_PALETTE: &str = "1;3;6;5;8";

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "{} {} [{}] {}",
        now.format_rfc3339(),
        style(level).paint(format!("{:<5}", record.level().to_string())),
        record.module_path().unwrap_or("<unnamed>"),
        format_body(level, record.args().to_string()),
    )
}

#[inline(always)]
fn format_body(level: Level, body: String) -> String {
    match level {
        // only paint Error, Warn, Info level
        Level::Error | Level::Warn | Level::Info => style(level).paint(&body).to_string(),
        _ => body,
    }
}

#[inline(always)]
pub fn init_logger() {
    Logger::try_with_env_or_str(env::var("RUST_LOG").unwrap_or(DEFAULT_RUST_LOG.to_string()))
        .unwrap()
        .use_utc()
        .format(log_format)
        .set_palette(LOG_COLOR_PALETTE.to_string())
        .log_to_file(
            FileSpec::default()
                .directory(env::var("LOG_DIR").unwrap_or(DEFAULT_LOG_DIR.to_string())),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();
}
