use rayon::prelude::*;

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
pub fn mutate_array_in_parallel(array: &mut [i32], update_value: i32) -> &[i32]{
    array.par_iter_mut().for_each(|p| *p += update_value);
    array
}