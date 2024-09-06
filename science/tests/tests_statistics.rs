use science::{calculate_mean, calculate_median, calculate_mode};

#[cfg(test)]
mod tests_statistics {

    use super::*;

    #[test]
    fn test_calculate_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = calculate_mean(&data);
        print!("mean: {:?}", mean);
        assert_eq!(mean, Some(3.0));
    }

    #[test]
    fn test_calculate_median() {
        let data = vec![1, 2, 3, 4, 5];
        let median = calculate_median(&data);
        print!("median: {:?}", median);
        assert_eq!(median, Some(3.0));
    }

    #[test]
    fn test_calculate_mode() {
        let data = vec![1, 2, 3, 4, 5, 5];
        let mode = calculate_mode(&data);
        print!("mode: {:?}", mode);
        assert_eq!(mode, Some(5));
    }
}
