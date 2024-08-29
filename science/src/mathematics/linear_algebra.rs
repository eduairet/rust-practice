use nalgebra::{ArrayStorage, Const, DMatrix, Matrix};
use ndarray::{Array, Array1, Array2, ArrayBase, ArrayView1, Dim, OwnedRepr};
use serde_json;

/// Adds two 2D matrices of `usize` values.
///
/// # Arguments
///
/// * `a` - A 2D matrix of `usize` values.
/// * `b` - A 2D matrix of `usize` values.
///
/// # Returns
///
/// A 2D matrix of `usize` values.
///
/// # Example
///
/// ```
/// use ndarray::arr2;
///
/// let a = arr2(&[[1, 2], [3, 4]]);
/// let b = arr2(&[[5, 6], [7, 8]]);
/// let c = arr2(&[[6, 8], [10, 12]]);
/// assert_eq!(c, science::adding_2d_usize_matrices(&a, &b));
/// ```
pub fn adding_2d_usize_matrices(a: &Array2<usize>, b: &Array2<usize>) -> ndarray::Array2<usize> {
    a + b
}

/// Multiplies two 2D matrices of `usize` values.
///
/// # Arguments
///
/// * `a` - A 2D matrix of `usize` values.
/// * `b` - A 2D matrix of `usize` values.
///
/// # Returns
///
/// A 2D matrix of `usize` values.
///
/// # Example
///
/// ```
/// use ndarray::arr2;
///
/// let a = arr2(&[[1, 2], [3, 4]]);
/// let b = arr2(&[[5, 6], [7, 8]]);
/// let c = arr2(&[[19, 22], [43, 50]]);
/// assert_eq!(c, science::multiplying_2d_usize_matrices(&a, &b));
/// ```
pub fn multiplying_2d_usize_matrices(
    a: &Array2<usize>,
    b: &Array2<usize>,
) -> ndarray::Array2<usize> {
    a.dot(b)
}

/// Multiplies a scalar with a vector and then multiplies the result with a matrix.
///
/// # Arguments
///
/// * `scalar` - A `usize` scalar value.
/// * `vector` - A 1D vector of `usize` values.
/// * `matrix` - A 2D matrix of `usize` values.
///
/// # Returns
///
/// A tuple of a 1D vector and a 1D vector.
///
/// # Example
///
/// ```
/// use science::multiply_scalar_with_vector_with_matrix;
/// use ndarray::{arr1, arr2};
///
/// let scalar = 2;
/// let vector = arr1(&[1, 2, 3]);
/// let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);
///
/// let expected_vector = arr1(&[2, 4, 6]);
/// let expected_matrix = matrix.dot(&expected_vector);
///
/// let (new_vector, new_matrix) = multiply_scalar_with_vector_with_matrix(scalar, &vector, &matrix);
///
/// assert_eq!(new_vector, expected_vector);
/// assert_eq!(new_matrix, expected_matrix);
/// ```
pub fn multiply_scalar_with_vector_with_matrix(
    scalar: usize,
    vector: &Array1<usize>,
    matrix: &Array2<usize>,
) -> (Array1<usize>, Array1<usize>) {
    let new_vector: Array1<_> = scalar * vector;
    let new_matrix = matrix.dot(&new_vector);

    (new_vector, new_matrix)
}

/// Compares two vectors of `f64` values.
///
/// # Arguments
///
/// * `vector_a` - A vector of `f64` values.
/// * `vector_b` - A vector of `f64` values.
///
/// # Returns
///
/// A tuple of two vectors of `f64` values.
///
/// # Example
///
/// ```
/// use science::vector_comparison;
///
/// let a = vec![1., 2., 3., 4., 5.];
/// let b = vec![5., 4., 3., 2., 1.];
/// let mut c = vec![1., 2., 3., 4., 5.];
/// let mut d = vec![5., 4., 3., 2., 1.];
///
/// let (z, w) = vector_comparison(&a, &b, &mut c, &mut d);
///
/// let expected_z = ndarray::Array::from_vec(vec![6., 6., 6., 6., 6.]);
/// let expected_w = ndarray::Array::from_vec(vec![6., 6., 6., 6., 6.]);
///
/// assert_eq!(z, expected_z);
/// assert_eq!(w, expected_w);
/// ```
pub fn vector_comparison(
    vector_a: &Vec<f64>,
    vector_b: &Vec<f64>,
    vector_c: &mut Vec<f64>,
    vector_d: &mut Vec<f64>,
) -> (
    ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
    ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
) {
    let a = Array::from_vec(vector_a.clone());
    let b = Array::from_vec(vector_b.clone());
    let c = Array::from_vec(vector_c.clone());
    let d = Array::from_vec(vector_d.clone());
    let z = a + b;
    let w = c + d;

    (z, w)
}

/// Computes the L1 norm of a 1D vector of `f64` values.
///
/// # Arguments
///
/// * `x` - A 1D vector of `f64` values.
///
/// # Returns
///
/// A `f64` value.
///
/// # Example
///
/// ```
/// use science::l1_norm;
///
/// let x = ndarray::array![1., 2., 3., 4., 5.];
///
/// let x_l1 = l1_norm(x.view());
///
/// assert_eq!(x_l1, 15.0);
/// ```
pub fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

/// Computes the L2 norm of a 1D vector of `f64` values.
///
/// # Arguments
///
/// * `x` - A 1D vector of `f64` values.
///
/// # Returns
///
/// A `f64` value.
///
/// # Example
///
/// ```
/// use science::l2_norm;
///
/// let x = ndarray::array![1., 2., 3., 4., 5.];
///
/// let x_l2 = l2_norm(x.view());
///
/// assert_eq!(x_l2, 7.416198487095663);
/// ```
pub fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

/// Normalizes a 1D vector of `f64` values.
///
/// # Arguments
///
/// * `x` - A 1D vector of `f64` values.
///
/// # Returns
///
/// A 1D vector of `f64` values.
///
/// # Example
///
/// ```
/// use science::normalize;
///
/// let x = ndarray::array![1., 2., 3., 4., 5.];
///
/// let x_norm = normalize(x);
///
/// assert_eq!(
///    x_norm,
///   ndarray::array![
///      0.13483997249264842,
///      0.26967994498529685,
///      0.40451991747794525,
///      0.5393598899705937,
///      0.674199862463242
///   ]);
pub fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e / norm);
    x
}

/// Inverts a 3x3 matrix of `f64` values.
///
/// # Arguments
///
/// * `matrix` - A 3x3 matrix of `f64` values.
///
/// # Returns
///
/// An `Option` of a 3x3 matrix of `f64` values.
///
/// # Example
///
/// ```
/// use nalgebra::Matrix3;
/// use science::invert_matrix;
///
/// let matrix = Matrix3::new(1., 2., 3., 0., 1., 4., 5., 6., 0.);
///
/// let inv = invert_matrix(matrix);
///
/// assert_eq!(
///    inv.unwrap(),
///    Matrix3::new(-24., 18., 5., 20., -15., -4., -5., 4., 1.)
/// );
/// ```
pub fn invert_matrix(
    matrix: Matrix<f64, Const<3>, Const<3>, ArrayStorage<f64, 3, 3>>,
) -> Option<Matrix<f64, Const<3>, Const<3>, ArrayStorage<f64, 3, 3>>> {
    match matrix.try_inverse() {
        Some(inv) => Some(inv),
        None => None,
    }
}

/// Serializes a 2D matrix of `i32` values to a JSON string.
///
/// # Arguments
///
/// * `matrix` - A 2D matrix of `i32` values.
///
/// # Returns
///
/// A `String`.
///
/// # Example
///
/// ```
/// use nalgebra::DMatrix;
///
/// let row_slice: Vec<i32> = (1..7).collect();
/// let matrix = DMatrix::from_row_slice(2, 3, &row_slice);
/// let serialized_matrix = science::serialize_matrix(matrix);
///
/// assert_eq!(serialized_matrix, "[[1,4,2,5,3,6],2,3]");
/// ```
pub fn serialize_matrix(matrix: DMatrix<i32>) -> String {
    let serialized_matrix = serde_json::to_string(&matrix).unwrap();
    serialized_matrix
}

/// Deserializes a JSON string to a 2D matrix of `i32` values.
///
/// # Arguments
///
/// * `serialized_matrix` - A JSON string.
///
/// # Returns
///
/// A 2D matrix of `i32` values.
///
/// # Example
///
/// ```
/// use nalgebra::DMatrix;
/// use science::{serialize_matrix, deserialize_matrix};
///
/// let row_slice: Vec<i32> = (1..5001).collect();
/// let matrix = DMatrix::from_row_slice(50, 100, &row_slice);
/// let serialized_matrix = serialize_matrix(matrix.clone());
/// let deserialized_matrix = deserialize_matrix(&serialized_matrix);
///
/// assert_eq!(matrix, deserialized_matrix);
/// ```
pub fn deserialize_matrix(serialized_matrix: &str) -> DMatrix<i32> {
    let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix).unwrap();
    deserialized_matrix
}
