use hardware_support::check_number_of_logical_cpu_cores;

#[cfg(test)]
mod tests_processor {
    use super::*;

    #[test]
    fn test_check_number_of_logical_cpu_cores() {
        let result = check_number_of_logical_cpu_cores();
        print!("{}", result);
        assert!(result.contains("Number of logical CPU cores:"));
    }
}
