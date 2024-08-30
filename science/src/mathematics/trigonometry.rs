/// Calculate the hypotenuse of a right-angle triangle
///
/// # Arguments
///
/// * `angle` - The angle of the triangle
/// * `side_length` - The length of the side of the triangle
///
/// # Example
///
/// ```
/// use science::calculate_hypotenuse;
///
/// let angle = 45.0;
/// let side_length = 1.0;
/// let hypotenuse = calculate_hypotenuse(angle, side_length);
/// assert_eq!(hypotenuse, 1.175221363135749);
/// ```
pub fn calculate_hypotenuse(angle: f64, side_length: f64) -> f64 {
    let hypotenuse = side_length / angle.sin();
    hypotenuse
}
