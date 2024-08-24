use num_cpus::get;

/// Checks the number of logical CPU cores
///
/// # Returns
///
/// A string that holds the number of logical CPU cores
///
/// # Examples
///
/// ```
/// use hardware_support::check_number_of_logical_cpu_cores;
///
/// let result = check_number_of_logical_cpu_cores();
/// assert!(result.contains("Number of logical CPU cores:"));
/// ```
pub fn check_number_of_logical_cpu_cores() -> String {
    let cpu_cores = get();
    format!("Number of logical CPU cores: {}", cpu_cores)
}
