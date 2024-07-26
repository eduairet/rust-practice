use development_tools::{log_debug_message, log_error_message, ConsoleLogger, MultiLogger};
use env_logger::{self, Builder, Target};
use log::{debug, error, info, set_boxed_logger, set_max_level, warn, LevelFilter};
use std::sync::{Mutex, Once};

#[cfg(test)]
mod tests_log_messages {

    use super::*;

    static INIT: Once = Once::new();
    static LOGGER: Mutex<Option<MultiLogger>> = Mutex::new(None);

    fn initialize() {
        INIT.call_once(|| {
            let mut logger = MultiLogger::new();

            let logger1 = Builder::new()
                .target(Target::Stdout)
                .filter_level(LevelFilter::Debug)
                .build();

            logger.add_logger(Box::new(logger1));

            let console_logger = ConsoleLogger;
            logger.add_logger(Box::new(console_logger));

            let multi_logger = LOGGER.lock().unwrap().take().unwrap_or(logger);

            set_boxed_logger(Box::new(multi_logger)).unwrap();
            set_max_level(LevelFilter::Debug);
        });
    }

    #[test]
    fn test_log_debug_message() {
        initialize();
        log_debug_message("This is a debug message");
    }

    #[test]
    fn test_log_error_message() {
        initialize();
        let result = log_error_message("This is an error message");
        assert_eq!(result, Err("ERROR: This is an error message".to_string()));
        if let Err(err) = result {
            error!("Failed to execute query: {}", err);
        }
    }

    #[test]
    fn test_log_to_stdout() {
        initialize();
        debug!("This is a message to stdout");
    }

    #[test]
    fn test_custom_logger() {
        initialize();
        info!("This is an info message");
        warn!("This is a warning message");
        error!("This is an error message");
    }
}
