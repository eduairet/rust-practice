use shared::EARTH_RADIUS_KM;

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

/// Calculate the distance between two points on Earth
///
/// # Arguments
///
/// * `point_a` - The latitude and longitude of the first point
/// * `point_b` - The latitude and longitude of the second point
///
/// # Example
///
/// ```
/// use science::distance_between_two_points_on_earth;
///
/// let point_a = (51.5074, 0.1278);
/// let point_b = (48.8566, 2.3522);
/// let distance = distance_between_two_points_on_earth(point_a, point_b);
/// assert_eq!(distance, 334.57613798050005);
/// ```
pub fn distance_between_two_points_on_earth(point_a: (f64, f64), point_b: (f64, f64)) -> f64 {
    let (lat1, lon1) = point_a;
    let (lat2, lon2) = point_b;

    let point_a_latitude = lat1.to_radians();
    let point_b_latitude = lat2.to_radians();

    let delta_latitude = (lat1 - lat2).to_radians();
    let delta_longitude = (lon1 - lon2).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + point_a_latitude.cos() * point_b_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = EARTH_RADIUS_KM * central_angle;
    distance
}
