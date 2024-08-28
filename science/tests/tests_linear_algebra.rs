use ndarray::arr2;
use science::{adding_2d_usize_matrices, multiplying_2d_usize_matrices};

#[cfg(test)]
mod tests_linear_algebra {

    use super::*;

    #[test]
    fn test_adding_2d_usize_matrices() {
        let a = arr2(&[[1, 2], [3, 4]]);
        let b = arr2(&[[5, 6], [7, 8]]);
        let c = arr2(&[[6, 8], [10, 12]]);
        print!("a: {:?}\n", a);
        print!("b: {:?}\n", b);
        print!("c: {:?}\n", c);
        assert_eq!(c, adding_2d_usize_matrices(&a, &b));
    }

    #[test]
    fn test_multiplying_2d_usize_matrices() {
        let a = arr2(&[[1, 2], [3, 4]]);
        let b = arr2(&[[5, 6], [7, 8]]);
        let c = arr2(&[[19, 22], [43, 50]]);
        print!("a: {:?}\n", a);
        print!("b: {:?}\n", b);
        print!("c: {:?}\n", c);
        assert_eq!(c, multiplying_2d_usize_matrices(&a, &b));
    }
}
