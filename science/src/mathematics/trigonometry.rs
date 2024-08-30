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

/// Check if the tangent of an angle is equal to the sine of the angle divided by the cosine of the angle
///
/// # Arguments
///
/// * `angle` - The angle to check
///
/// # Example
///
/// ```
/// use science::is_tan_equal_to_sin_divided_by_cos;
///
/// let angle = 45.0;
/// let result = is_tan_equal_to_sin_divided_by_cos(angle);
/// assert!(result);
/// ```
pub fn is_tan_equal_to_sin_divided_by_cos(angle: f64) -> bool {
    let a = angle.tan();
    let b = angle.sin() / angle.cos();
    a == b
}
