use algorithms::{
    generate_random_numbers, generate_random_numbers_in_range,
    generate_random_values_from_custom_type, guess_dice_roll, Point,
};

#[cfg(test)]
mod tests_random {

    use super::*;

    #[test]
    fn test_generate_random_u8() {
        let rn_u8: u8 = generate_random_numbers();
        assert!(rn_u8 >= 1 && rn_u8 <= std::u8::MAX);
    }

    #[test]
    fn test_generate_random_u16() {
        let rn_u16: u16 = generate_random_numbers();
        assert!(rn_u16 >= 1 && rn_u16 <= std::u16::MAX);
    }

    #[test]
    fn test_generate_random_u32() {
        let rn_u32: u32 = generate_random_numbers();
        assert!(rn_u32 >= 1 && rn_u32 <= std::u32::MAX);
    }

    #[test]
    fn test_generate_random_i32() {
        let rn_i32: i32 = generate_random_numbers();
        assert!(rn_i32 >= std::i32::MIN && rn_i32 <= std::i32::MAX);
    }

    #[test]
    fn test_generate_random_f64() {
        let rn_f64: f64 = generate_random_numbers();
        assert!(rn_f64 >= 0.0 && rn_f64 <= 1.0);
    }

    #[test]
    fn test_generate_random_u8_in_range() {
        let rn_u8: u8 = generate_random_numbers_in_range(1, 100);
        assert!(rn_u8 >= 1 && rn_u8 <= std::u8::MAX);
    }

    #[test]
    fn test_generate_random_f64_in_range() {
        let rn_f64: f64 = generate_random_numbers_in_range(1.0, 100.0);
        assert!(rn_f64 >= 1.0 && rn_f64 <= 100.0);
    }

    #[test]
    fn test_guess_dice_roll() {
        let guess = 3;
        let rolls = 3;
        let message = guess_dice_roll(guess, rolls);
        assert!(message == "You guessed correctly!" || message == "You ran out of rolls!");
    }

    #[test]
    #[should_panic]
    fn test_guess_dice_roll_panic() {
        let guess = 7;
        let rolls = 3;
        guess_dice_roll(guess, rolls);
    }

    #[test]
    fn test_generate_random_values_from_custom_type() {
        type CustomType = (i32, bool, f64);
        let (rand_tuple, rand_point): (CustomType, Point) =
            generate_random_values_from_custom_type();
        assert!(rand_tuple.0 >= std::i32::MIN && rand_tuple.0 <= std::i32::MAX);
        assert!(rand_tuple.1 == true || rand_tuple.1 == false);
        assert!(rand_tuple.2 >= 0.0 && rand_tuple.2 <= 1.0);
        assert!(rand_point.x >= std::i32::MIN && rand_point.x <= std::i32::MAX);
        assert!(rand_point.y >= std::i32::MIN && rand_point.y <= std::i32::MAX);
    }
}
