use ndarray::{Array, Array1, Array2, ArrayBase, Dim, OwnedRepr};

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
