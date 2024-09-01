use num::complex::Complex;

/// Adds a list of complex numbers together.
///
/// # Arguments
///
/// * `nums` - A list of complex numbers.
///
/// # Returns
///
/// The sum of the complex numbers.
///
/// # Example
///
/// ```
/// use science::mathematics::complex_numbers::add_complex_numbers;
/// use num::complex::Complex;
///
/// let nums = vec![
///    Complex::new(1.0, 2.0),
///    Complex::new(3.0, 4.0),
/// ];
/// let sum = add_complex_numbers(nums);
/// assert_eq!(sum, Complex::new(4.0, 6.0));
/// ```
pub fn add_complex_numbers(nums: Vec<Complex<f64>>) -> Complex<f64> {
    let mut sum = Complex::new(0.0, 0.0);
    for num in nums {
        sum += num;
    }
    sum
}
