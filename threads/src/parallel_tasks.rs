use glob::{glob_with, MatchOptions};
use rayon::prelude::*;
use shared::make_thumbnail;
use shared::Person;
use std::{error::Error, fs::create_dir_all, marker::Sync};

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

/// Map and reduce the age of people in parallel
///
/// # Arguments
///
/// * `array` - A slice of Person values
/// * `condition` - A closure that takes an u32 value and returns a boolean value
///
/// # Returns
///
/// An i32 value
///
/// # Example
///
/// ```
/// use threads::map_reduce_person_age_in_parallel;
/// use shared::Person;
/// use rayon::prelude::*;
///
/// let vec = [
///    Person {
///       name: "John".to_string(),
///       age: 25,
///    },
///    Person {
///       name: "Jane".to_string(),
///       age: 20,
///    },
/// ];
///
/// let condition = |x| x > 20;
/// let result = map_reduce_person_age_in_parallel(&vec, condition);
/// println!("{}", result);
/// ```
pub fn map_reduce_person_age_in_parallel(
    array: &[Person],
    condition: impl Fn(u32) -> bool + Sync,
) -> i32 {
    let filtered_array: Vec<&Person> = array.par_iter().filter(|p| condition(p.age)).collect();
    filtered_array
        .par_iter()
        .map(|p| p.age as i32)
        .reduce(|| 0, |acc, x| acc + x)
}

pub fn generate_jpg_thumbnails_in_parallel(thumb_dir: &str) -> Result<(), Box<dyn Error>> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        eprintln!("No JPG files found");
        return Ok(());
    }

    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .filter_map(|x| make_thumbnail(x, thumb_dir, 100).err())
        .collect();

    image_failures
        .iter()
        .for_each(|e| eprintln!("Error: {}", e));

    println!(
        "{} thumbnails saved successfully",
        files.len() - image_failures.len()
    );

    Ok(())
}
