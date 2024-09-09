use science::{
    calculate_mean, calculate_mean_i32, calculate_median, calculate_mode,
    calculate_standard_deviation,
};

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

    #[test]
    fn test_calculate_standard_deviation() {
        let data = [12, 15, 15, 7, 3, 3, 1, 1, 1, 10];

        let data_mean = calculate_mean_i32(&data);
        println!("Mean: {:?}", data_mean);
        assert_eq!(data_mean, Some(6.8));

        let data_std_deviation = calculate_standard_deviation(&data);
        println!("Standard Deviation: {:?}", data_std_deviation);
        assert_eq!(data_std_deviation, Some(5.491812));

        let z_score = match (data_mean, data_std_deviation) {
            (Some(mean), Some(std_deviation)) => {
                let diff = data[4] as f32 - mean;
                Some(diff / std_deviation)
            }
            _ => None,
        };
        println!(
            "Z-score of data at index 4 (with value {}) is {:?}",
            data[4], z_score
        );
        assert_eq!(data[4], 3);
        assert_eq!(z_score, Some(-0.69193923));
    }
}
