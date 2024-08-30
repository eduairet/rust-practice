use science::calculate_hypotenuse;

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
}
