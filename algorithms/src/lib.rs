use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// Generates a random number of type `T`.
///
/// # Examples
///
/// ```
/// use algorithms::generate_random_numbers;
///
/// let random_number: u32 = generate_random_numbers();
/// println!("Random number: {}", random_number);
/// ```
pub fn generate_random_numbers<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen::<T>()
}
