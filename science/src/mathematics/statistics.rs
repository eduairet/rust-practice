use std::{cmp::Ordering, collections::HashMap};

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

/// Partition a dataset around a pivot value
///
/// # Arguments
///
/// * `data` - A slice of i32 values
///
/// # Returns
///
/// * `Option<(Vec<i32>, i32, Vec<i32>)>` - A tuple containing the left partition, pivot value, and right partition
///
/// # Example
///
/// ```
/// use science::partition;
///
/// let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// let part = partition(&data);
/// assert_eq!(part, Some((vec![1, 1, 2], 3, vec![4, 5, 9, 6, 5, 3])));
/// ```
pub fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}

/// Select the k-th smallest element from a dataset
///
/// # Arguments
///
/// * `data` - A slice of i32 values
/// * `k` - The index of the element to select
///
/// # Returns
///
/// * `Option<i32>` - The k-th smallest element in the dataset
///
/// # Example
///
/// ```
/// use science::select;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let k = 2;
/// let kth = select(&data, k);
/// assert_eq!(kth, Some(3));
/// ```
pub fn select(data: &[i32], k: usize) -> Option<i32> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

/// Calculate the median of a dataset
///
/// # Arguments
///
/// * `data` - A slice of i32 values
///
/// # Returns
///
/// * `Option<f32>` - The median of the dataset
///
/// # Example
///
/// ```
/// use science::calculate_median;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let median = calculate_median(&data);
/// assert_eq!(median, Some(3.0));
/// ```
pub fn calculate_median(data: &[i32]) -> Option<f32> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                _ => None,
            }
        }
        odd => select(data, odd / 2).map(|x| x as f32),
    }
}

/// Calculate the mode of a dataset
///
/// # Arguments
///
/// * `data` - A slice of i32 values
///
/// # Returns
///
/// * `Option<i32>` - The mode of the dataset
///
/// # Example
///
/// ```
/// use science::calculate_mode;
///
/// let data = vec![1, 2, 3, 4, 5, 5];
/// let mode = calculate_mode(&data);
/// assert_eq!(mode, Some(5));
/// ```
pub fn calculate_mode(data: &[i32]) -> Option<i32> {
    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    mode
}

/// Calculate the mean of a dataset
///
/// # Arguments
///
/// * `data` - A slice of i32 values
///
/// # Returns
///
/// * `Option<f32>` - The mean of the dataset
///
/// # Example
///
/// ```
/// use science::calculate_mean_i32;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let mean = calculate_mean_i32(&data);
/// assert_eq!(mean, Some(3.0));
/// ```
pub fn calculate_mean_i32(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

/// Calculate the standard deviation of a dataset
///
/// # Arguments
///
/// * `data` - A slice of i32 values
///
/// # Returns
///
/// * `Option<f32>` - The standard deviation of the dataset
///
/// # Example
///
/// ```
/// use science::calculate_standard_deviation;
///
/// let data = [12, 15, 15, 7, 3, 3, 1, 1, 1, 10];
/// let std_deviation = calculate_standard_deviation(&data);
/// assert_eq!(std_deviation, Some(5.491812));
/// ```
pub fn calculate_standard_deviation(data: &[i32]) -> Option<f32> {
    match (calculate_mean_i32(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}
