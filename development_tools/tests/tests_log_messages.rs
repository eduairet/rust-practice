use development_tools::{log_debug_message, log_error_message, ConsoleLogger, MultiLogger};
use env_logger::{self, Builder, Target};
use log::{debug, error, info, set_boxed_logger, set_max_level, warn, LevelFilter};
use std::sync::{Mutex, Once};
use syslog::{BasicLogger, Facility, Formatter3164};

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

            let logger2 = ConsoleLogger;
            logger.add_logger(Box::new(logger2));

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
            error!("Failed to execute: {}", err);
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

    #[test]
    #[ignore] // Run individually and in a Unix-like environment
    fn test_log_to_syslog() {
        let formatter = Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: "syslog_rust".into(),
            pid: 0,
        };

        let logger = match syslog::unix(formatter) {
            Err(e) => {
                println!("impossible to connect to syslog: {:?}", e);
                return;
            }
            Ok(logger) => logger,
        };

        set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| set_max_level(LevelFilter::Debug))
            .unwrap();

        info!("hello world");
        debug!("This is a syslog debug");
        error!("This is a syslog error!");
    }
}
