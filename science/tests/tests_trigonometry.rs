use science::{calculate_hypotenuse, is_tan_equal_to_sin_divided_by_cos};

#[cfg(test)]
mod tests_trigonometry {
    use super::*;

    #[test]
    fn test_calculate_hypotenuse() {
        let angle = 2.0;
        let side_length = 80.0;
        let hypotenuse = calculate_hypotenuse(angle, side_length);
        print!("Hypotenuse: {}", hypotenuse);
        assert_eq!(hypotenuse, 87.98001362356932);
    }

    #[test]
    fn test_is_tan_equal_to_sin_divided_by_cos() {
        let angle = 6.0;
        let result = is_tan_equal_to_sin_divided_by_cos(angle);
        print!("Result: {}", result);
        assert!(result);
    }
}
