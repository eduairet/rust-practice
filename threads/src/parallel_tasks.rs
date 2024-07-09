use rayon::prelude::*;
use std::marker::Sync;

/// Mutate an array in parallel
///
/// # Arguments
///
/// * `array` - A mutable slice of i32 values
/// * `update_value` - An i32 value to add to each element in the array
///
/// # Returns
///
/// A mutable slice of i32 values
///
/// # Example
///
/// ```
/// use threads::mutate_array_in_parallel;
///
/// let mut array = [1, 2, 3, 4, 5];
/// let update_value = 10;
/// let result = mutate_array_in_parallel(&mut array, update_value);
/// println!("{:?}", result);
/// ```
pub fn mutate_array_in_parallel(array: &mut [i32], update_value: i32) -> &[i32] {
    array.par_iter_mut().for_each(|p| *p += update_value);
    array
}

/// Match a predicate in parallel
///
/// # Arguments
///
/// * `array` - A slice of i32 values
/// * `is_all` - A boolean value to determine if all elements should match the predicate
/// * `predicate` - A closure that takes an i32 value and returns a boolean value
///
/// # Returns
///
/// A boolean value
///
/// # Example
///
/// ```
/// use threads::match_predicate_in_parallel;
///
/// let array = [1, 2, 3, 4, 5];
///
/// let is_all = true;
/// let predicate = |x| x > 0;
/// let result = match_predicate_in_parallel(&array, is_all, predicate);
/// println!("{}", result);
/// ```
pub fn match_predicate_in_parallel(
    array: &[i32],
    is_all: bool,
    predicate: impl Fn(i32) -> bool + Sync,
) -> bool {
    if is_all {
        array.par_iter().all(|&x| predicate(x))
    } else {
        array.par_iter().any(|&x| predicate(x))
    }
}
