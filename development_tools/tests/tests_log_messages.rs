use development_tools::{log_debug_message, log_error_message};
use log::{error, LevelFilter};
use std::sync::Once;

#[cfg(test)]
mod tests_log_messages {
    use super::*;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            env_logger::builder()
                .filter_level(LevelFilter::Debug)
                .init();
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
}
