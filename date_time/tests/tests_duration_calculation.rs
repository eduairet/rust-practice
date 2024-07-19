use date_time::get_two_code_sections_elapsed_time;

#[cfg(test)]
mod tests_duration_calculation {
    use super::*;

    #[test]
    fn test_get_two_code_sections_elapsed_time() {
        let result = get_two_code_sections_elapsed_time(|| {
            let mut _sum: u64 = 0;
            for i in 0..200000000 {
                _sum += i;
            }
        });
        print!("{}", result);
        assert_eq!(result.contains("Elapsed time:"), true);
    }
}
