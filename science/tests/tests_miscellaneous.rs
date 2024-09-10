use num::bigint::ToBigInt;
use science::factorial;

#[cfg(test)]
mod tests_miscellaneous {

    use super::*;

    #[test]
    fn test_factorial() {
        let x = 4;
        println!("Factorial of {} equals {}", x, factorial(x));
        assert_eq!(factorial(x), 24.to_bigint().unwrap());
    }
}
