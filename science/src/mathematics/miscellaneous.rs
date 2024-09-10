use num::bigint::{BigInt, ToBigInt};

/// Calculate the factorial of a given integer.
///
/// # Arguments
///
/// * `x` - The integer for which the factorial is to be calculated.
///
/// # Returns
///
/// The factorial of the given integer.
///
/// # Example
///
/// ```
/// use num::bigint::ToBigInt;
/// use science::factorial;
///
/// let x = 4;
/// assert_eq!(factorial(x), 24.to_bigint().unwrap());
/// ```
pub fn factorial(x: i32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..=x {
            factorial = factorial * i;
        }
        factorial
    } else {
        panic!("Failed to calculate factorial!");
    }
}
