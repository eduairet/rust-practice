use log::debug;
use log::{Level, Log, Metadata, Record};

/// Log a debug message to the console
///
/// # Arguments
///
/// * `message` - A string slice
///
/// # Examples
///
/// ```
/// use development_tools::log_debug_message;
///
/// log_debug_message("This is a debug message");
/// ```
pub fn log_debug_message(message: &str) {
    // RUST_LOG=debug cargo run
    debug!("DEBUG: {}", message);
}

/// Log an error message to the console
///
/// # Arguments
///
/// * `message` - A string slice
///
/// # Returns
///
/// * A Result object
///
/// # Examples
///
/// ```
/// use development_tools::log_error_message;
///
/// env_logger::init();
///
/// let result = log_error_message("This is an error message");
/// if let Err(err) = result {
///    eprintln!("Failed to execute query: {}", err);
/// }
/// ```
pub fn log_error_message(message: &str) -> Result<(), String> {
    Err(format!("ERROR: {}", message))
}

/// A custom logger that logs messages to the console
///
/// # Examples
///
/// ```
/// use development_tools::ConsoleLogger;
/// use log::{set_logger, set_max_level, LevelFilter, info, error, warn};
///
/// static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;
///
/// if set_logger(&CONSOLE_LOGGER).is_ok() {
///    set_max_level(LevelFilter::Info);
/// }
///
/// info!("This is an info message");
/// warn!("This is a warning message");
/// error!("This is an error message");
/// ```
pub struct ConsoleLogger;
impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

/// A custom logger that logs messages to multiple loggers
///
/// # Examples
///
/// ```
/// use development_tools::{ConsoleLogger, MultiLogger};
/// use log::{set_boxed_logger, set_max_level, LevelFilter, info, error, warn};
///
/// let mut logger = MultiLogger::new();
///
/// let logger1 = ConsoleLogger;
/// logger.add_logger(Box::new(logger1));
///
/// if set_boxed_logger(Box::new(logger)).is_ok() {
///    set_max_level(LevelFilter::Info);
/// }
///
/// info!("This is an info message");
/// warn!("This is a warning message");
/// error!("This is an error message");
/// ```
pub struct MultiLogger {
    loggers: Vec<Box<dyn Log + Sync + Send>>,
}
impl MultiLogger {
    pub fn new() -> Self {
        Self {
            loggers: Vec::new(),
        }
    }

    pub fn add_logger(&mut self, logger: Box<dyn Log + Sync + Send>) {
        self.loggers.push(logger);
    }
}
impl Log for MultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.loggers.iter().any(|logger| logger.enabled(metadata))
    }

    fn log(&self, record: &Record) {
        for logger in &self.loggers {
            if logger.enabled(record.metadata()) {
                logger.log(record);
            }
        }
    }

    fn flush(&self) {
        for logger in &self.loggers {
            logger.flush();
        }
    }
}
