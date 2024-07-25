use development_tools::log_debug_message;

#[cfg(test)]
mod tests_log_messages {
    use super::*;

    #[test]
    fn test_log_debug_message() {
        env_logger::init();
        log_debug_message("This is a debug message");
    }
}
