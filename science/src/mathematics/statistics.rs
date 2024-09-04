/// Calculate the mean of a dataset
///
/// # Arguments
///
/// * `data` - A slice of f64 values
///
/// # Returns
///
/// * `Option<f64>` - The mean of the dataset
///
/// # Example
///
/// ```
/// use science::calculate_mean;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let mean = calculate_mean(&data);
/// assert_eq!(mean, Some(3.0));
/// ```
pub fn calculate_mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>();
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    };

    mean
}
