use crossbeam::scope;

/// Find the maximum value in an array
///
/// # Arguments
///
/// * `arr` - A slice of i32 values
///
/// * `threshold` - A usize value that holds the threshold for the array length
///
/// # Returns
///
/// * An Option<i32> which holds the maximum value in the array
///
/// # Examples
///
/// ```
/// use threads::find_max;
///
/// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let threshold = 5;
/// assert_eq!(Some(10), find_max(&arr, threshold));
/// ```
pub fn find_max(arr: &[i32], threshold: usize) -> Option<i32> {
    if arr.len() <= threshold {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    scope(|s| {
        let thread_l = s.spawn(|_| find_max(left, threshold));
        let thread_r = s.spawn(|_| find_max(right, threshold));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}
