use rand::{
    distributions::{uniform::SampleUniform, Alphanumeric, Distribution, Standard, Uniform},
    thread_rng, Rng,
};

mod structs;
pub use structs::*;

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

/// Generates a random number of type `T` in the range `start..end`.
///
/// # Examples
///
/// ```
/// use algorithms::generate_random_numbers_in_range;
///
/// let random_number: u32 = generate_random_numbers_in_range(1, 100);
/// println!("Random number: {}", random_number);
/// ```
pub fn generate_random_numbers_in_range<T>(start: T, end: T) -> T
where
    Standard: Distribution<T>,
    T: PartialOrd + Copy + SampleUniform,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

/// Guesses a dice roll.
///
/// # Arguments
///
/// * `guess` - A `u8` that represents the guess.
/// * `rolls` - A `u8` that represents the number of rolls.
///
/// # Returns
///
/// * `String` - A message indicating if the guess was correct or not.
///
/// # Examples
///
/// ```
/// use algorithms::guess_dice_roll;
///
/// let guess = 3;
///
/// let message = guess_dice_roll(guess, 3);
/// println!("{}", message);
/// ```
///
/// # Panics
///
/// Panics if the guess is not between 1 and 6.
pub fn guess_dice_roll(guess: u8, mut rolls: u8) -> String {
    if guess < 1 || guess > 6 {
        panic!("Invalid guess. Please guess a number between 1 and 6.");
    }

    let mut rng = rand::thread_rng();
    let die = Uniform::new_inclusive(1, 6);

    while rolls > 0 {
        let roll = die.sample(&mut rng);
        println!("Roll: {}", roll);
        if roll == guess {
            return String::from("You guessed correctly!");
        }
        rolls -= 1;
    }
    String::from("You ran out of rolls!")
}

/// Generates random values from a custom type.
///
/// # Examples
///
/// ```
/// use algorithms::{generate_random_values_from_custom_type, Point};
///
/// let (rand_tuple, rand_point) = generate_random_values_from_custom_type::<(u8, u8)>();
/// println!("Random tuple: {:?}", rand_tuple);
/// println!("Random point: {:?}", rand_point);
pub fn generate_random_values_from_custom_type<T>() -> (T, Point)
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<T>();
    let rand_point: Point = rng.gen::<Point>();
    (rand_tuple, rand_point)
}

/// Generates a random password of a given length.
/// The password will contain alphanumeric characters.
///
/// # Arguments
///
/// * `length` - A `usize` that represents the length of the password.
///
/// # Returns
///
/// * `String` - A random password.
///
/// # Examples
///
/// ```
/// use algorithms::generate_random_password;
///
/// let password = generate_random_password(10);
/// println!("Random password: {}", password);
/// ```
pub fn generate_random_password(length: usize) -> String {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    password
}
