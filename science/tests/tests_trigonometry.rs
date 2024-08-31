use science::{
    calculate_hypotenuse, distance_between_two_points_on_earth, is_tan_equal_to_sin_divided_by_cos,
};

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

    #[test]
    fn test_distance_between_two_points_on_earth() {
        let point_a = (48.85341_f64, -2.34880_f64);
        let point_b = (51.50853_f64, -0.12574_f64);
        let distance = distance_between_two_points_on_earth(point_a, point_b);
        print!("Distance: {}", distance);
        assert_eq!(distance, 334.95585243753624);
    }
}
