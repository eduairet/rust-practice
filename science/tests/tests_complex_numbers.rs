use num::complex::Complex;
use science::add_complex_numbers;
use std::f64::consts::PI;

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

    #[test]
    fn test_complex_numbers_math() {
        let x = Complex::new(0.0, 2.0 * PI);
        println!("e^(2i * pi) = {}", x.exp());
        assert_eq!(x.exp(), Complex::new(1.0, -2.4492935982947064e-16));
    }
}
