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

/// Sort a vector of strings in parallel
///
/// # Arguments
///
/// * `vector` - A mutable vector of strings
///
/// # Returns
///
/// A vector of strings
///
/// # Example
///
/// ```
/// use threads::sort_string_vector_in_parallel;
/// use rand::{distributions::Alphanumeric, thread_rng, Rng};
/// use rayon::prelude::*;
///
/// let mut vec = vec![String::new(); 10];
/// vec.par_iter_mut().for_each(|p| {
///    let mut rng = thread_rng();
///    *p = (0..3).map(|_| rng.sample(Alphanumeric) as char).collect()
/// });
/// println!("{:?}", vec);
/// let sorted_parallel = sort_string_vector_in_parallel(&mut vec);
/// println!("{:?}", sorted_parallel);
/// ```
pub fn sort_string_vector_in_parallel(vector: &mut Vec<String>) -> Vec<String> {
    vector.par_sort_unstable();
    vector.to_vec()
}
