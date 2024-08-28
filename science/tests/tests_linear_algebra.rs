use approx::assert_abs_diff_eq;
use ndarray::{arr1, arr2, Array, ArrayBase, Dim, OwnedRepr};
use science::{
    adding_2d_usize_matrices, multiply_scalar_with_vector_with_matrix,
    multiplying_2d_usize_matrices, vector_comparison,
};

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

    #[test]
    fn test_multiply_scalar_with_vector_with_matrix() {
        let scalar = 2;
        let vector = arr1(&[1, 2, 3]);
        let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);

        let expected_vector = arr1(&[2, 4, 6]);
        let expected_matrix = matrix.dot(&expected_vector);

        let (new_vector, new_matrix) =
            multiply_scalar_with_vector_with_matrix(scalar, &vector, &matrix);

        println!("{}", new_vector);
        assert_eq!(new_vector, expected_vector);
        println!("{}", new_matrix);
        assert_eq!(new_matrix, expected_matrix);
    }

    #[test]
    fn test_vector_comparison() {
        let a = vec![1., 2., 3., 4., 5.];
        let b = vec![5., 4., 3., 2., 1.];
        let mut c = vec![1., 2., 3., 4., 5.];
        let mut d = vec![5., 4., 3., 2., 1.];

        let (z, w) = vector_comparison(&a, &b, &mut c, &mut d);

        let expected_z: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> =
            Array::from(vec![6., 6., 6., 6., 6.]);
        let expected_w: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> =
            Array::from(vec![6., 6., 6., 6., 6.]);

        println!("{:?}", z);
        println!("{:?}", expected_z);

        assert_abs_diff_eq!(z.as_slice().unwrap(), expected_z.as_slice().unwrap());
        assert_abs_diff_eq!(w.as_slice().unwrap(), expected_w.as_slice().unwrap());
    }
}
