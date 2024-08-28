use ndarray::Array2;

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
