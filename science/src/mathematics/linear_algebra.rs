use ndarray::{Array1, Array2};

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
