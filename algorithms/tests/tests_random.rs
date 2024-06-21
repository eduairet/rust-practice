use algorithms::generate_random_numbers;

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
}
