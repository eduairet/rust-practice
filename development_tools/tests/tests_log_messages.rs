use development_tools::{log_debug_message, log_error_message};
use log::error;

#[cfg(test)]
mod tests_log_messages {
    use super::*;

    #[test]
    fn test_log_debug_message() {
        env_logger::init();
        log_debug_message("This is a debug message");
    }

    #[test]
    fn test_log_error_message() {
        env_logger::init();
        let result = log_error_message("This is an error message");
        assert_eq!(result, Err("ERROR: This is an error message".to_string()));
        if let Err(err) = result {
            error!("Failed to execute query: {}", err);
        }
    }
}
