//! # Explora Log
//
//! This crate provides basic logging backend functionality for the Explora project.
//!
//! It does not aim to be a full-featured logger, nor the best logging
//! solution. It aims to be simple and keep the dependencies to a minimum.

use std::io::Write;
use std::str::FromStr;

use termcolor::{Color, ColorSpec, WriteColor};

const LOGGER: Logger = Logger {
    level: log::LevelFilter::Trace,
};

pub fn init() {
    Logger::default().try_init();
}

pub fn init_from_env() {
    Logger::default().env().try_init();
}

pub struct Logger {
    level: log::LevelFilter,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            level: log::LevelFilter::Trace,
        }
    }
}

impl Logger {
    /// Sets the level of verbosity for the logger from the environment variable `RUST_LOG`.
    /// 
    /// e.g `RUST_LOG=info cargo run`
    /// 
    /// If `RUST_LOG` is not set it will fallback to the default level `Trace`.
    pub fn env(mut self) -> Self {
        self.level = std::env::var("RUST_LOG")
            .ok()
            .as_deref()
            .map(log::LevelFilter::from_str)
            .and_then(Result::ok)
            .unwrap_or(self.level);
        self
    }

    /// Sets the level of verbosity for the logger.
    /// e.g:
    /// ```
    /// use explora::logger;
    /// logger::Logger::default()
    ///    .level(log::LevelFilter::Info)
    ///   .try_init();
    /// ```
    /// This will allow all log messages with level `Info` or higher to be printed.
    pub const fn level(mut self, level: log::LevelFilter) -> Self {
        self.level = level;
        self
    }

    pub fn try_init(self) {
        if let Err(e) = log::set_logger(&LOGGER) {
            eprintln!("Failed to set up logger: {}", e);
        }
        log::set_max_level(self.level);
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true // log::set_max_level() will filter out messages
    }

    fn log(&self, record: &log::Record) {
        // no need to check if enabled
        // since log::set_max_level() is already set
        const COLORS: [Color; 5] = [
            Color::Red,    // error
            Color::Yellow, // warn
            Color::Green,  // info
            Color::Blue,   // debug
            Color::Black,  // trace
        ];

        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(COLORS[record.level() as usize - 1]));
        spec.set_bold(true);

        if let Err(e) = stdout.set_color(&spec) {
            eprintln!("Failed to set color: {}", e);
        }
        // format: [level target file:line] message
        if let Err(e) = writeln!(
            &mut stdout,
            "[{} {}:{}]:\t{}",
            record.level(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        ) {
            eprintln!("Failed to write to stdout: {}", e);
        }
    }

    fn flush(&self) {}
}
