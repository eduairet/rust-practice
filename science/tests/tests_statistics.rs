use science::calculate_mean;

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
}
