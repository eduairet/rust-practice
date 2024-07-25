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
    // RUST_LOG=error cargo run
    Err(format!("ERROR: {}", message))
}
