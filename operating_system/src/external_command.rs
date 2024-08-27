use std::{error::Error, process::Command};

/// Run an external command and return the stdout as a string
///
/// # Arguments
///
/// * `command` - A string slice that holds the command to be executed
///
/// * `args` - A vector of string slices that holds the arguments to be passed to the command
///     
/// # Returns
///
/// * A Result that holds a string on success, or a Box<dyn Error> on failure
///
/// # Example
///
/// ```
/// use operating_system::run_external_command_process_stdout;
///
/// let command = "ls";
/// let args = vec!["-l"];
/// let output = run_external_command_process_stdout(command, args).unwrap();
/// assert!(output.contains("Cargo.toml"));
/// ```
pub fn run_external_command_process_stdout(
    command: &str,
    args: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout)?;

    Ok(stdout)
}
