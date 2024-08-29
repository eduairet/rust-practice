use approx::assert_abs_diff_eq;
use nalgebra::{DMatrix, Matrix3};
use ndarray::{arr1, arr2, array, Array, ArrayBase, Dim, OwnedRepr};
use science::{
    adding_2d_usize_matrices, deserialize_matrix, invert_matrix, l1_norm, l2_norm,
    multiply_scalar_with_vector_with_matrix, multiplying_2d_usize_matrices, normalize,
    serialize_matrix, vector_comparison,
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

    #[test]
    fn test_vec_nom() {
        let x = array![1., 2., 3., 4., 5.];

        let x_l1 = l1_norm(x.view());
        let x_l2 = l2_norm(x.view());
        let x_norm = normalize(x);

        println!("||x||_2 = {}", x_l1);
        assert_eq!(x_l1, 15.0);
        println!("||x||_1 = {}", x_l2);
        assert_eq!(x_l2, 7.416198487095663);
        println!("Normalizing x yields {:?}", x_norm);
        assert_eq!(
            x_norm,
            array![
                0.13483997249264842,
                0.26967994498529685,
                0.40451991747794525,
                0.5393598899705937,
                0.674199862463242
            ]
        );
    }

    #[test]
    fn test_invert_matrix() {
        let matrix = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
        println!("matrix = {}", matrix);
        let inverted_matrix = invert_matrix(matrix);
        println!("inverted_matrix = {}", inverted_matrix.unwrap());
        let expected_inverted_matrix =
            Matrix3::new(3.0, -1.0, -1.0, -4.0, 2.0, 1.0, -1.0, 0.0, 1.0);
        assert_eq!(inverted_matrix.unwrap(), expected_inverted_matrix);
    }

    #[test]
    fn test_de_serialize_matrix() {
        let row_slice: Vec<i32> = (1..11).collect();
        let matrix = DMatrix::from_row_slice(2, 5, &row_slice);
        let expected_serialized_matrix = "[[1,6,2,7,3,8,4,9,5,10],2,5]";

        let serialized_matrix = serialize_matrix(matrix.clone());
        println!("serialized_matrix = {:?}", serialized_matrix);
        assert_eq!(serialized_matrix, expected_serialized_matrix);

        let deserialized_matrix = deserialize_matrix(&serialized_matrix);
        println!("deserialized_matrix = {:?}", deserialized_matrix);
        assert_eq!(matrix, deserialized_matrix);
    }
}
