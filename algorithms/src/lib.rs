use rand::{
    distributions::{uniform::SampleUniform, Distribution, Standard, Uniform},
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
