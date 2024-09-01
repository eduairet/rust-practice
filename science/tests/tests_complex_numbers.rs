use num::complex::Complex;
use science::add_complex_numbers;

#[cfg(test)]
mod tests_complex_numbers {

    use super::*;

    #[test]
    fn test_complex_numbers() {
        let nums = vec![Complex::new(10.0, 20.0), Complex::new(3.1, -4.2)];
        let sum = add_complex_numbers(nums);
        println!("sum: {}", sum);
        assert_eq!(sum, Complex::new(13.1, 15.8));
    }
}
