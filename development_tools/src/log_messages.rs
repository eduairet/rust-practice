use log::debug;

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
    debug!("DEBUG: {}", message);
}
