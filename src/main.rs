use algorithms::{
    generate_random_numbers, generate_random_numbers_in_range, generate_random_password,
    generate_random_password_with_custom_characters, generate_random_values_from_custom_type,
    guess_dice_roll, Point,
};

fn main() {
    // Generate random numbers
    println!("Random u8: {}", generate_random_numbers::<u8>());
    println!("Random f64: {}", generate_random_numbers::<f64>());
    // Generate random numbers in range
    println!("Integer: {}", generate_random_numbers_in_range(5, 20));
    println!("Float: {}", generate_random_numbers_in_range(2.5, 4.5));
    // Guess dice roll
    let (guess, rolls) = (3, 3);
    let message = guess_dice_roll(guess, rolls);
    println!("{}", message);
    // Generate random values from custom type
    type CustomType = (i32, bool, f64);
    let (rand_tuple, rand_point): (CustomType, Point) = generate_random_values_from_custom_type();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random point: {:?}", rand_point);
    // Generate random password
    let password = generate_random_password(10);
    println!("Random password: {}", password);
    // Generate random password with custom characters
    let password = generate_random_password_with_custom_characters(10, b"abc123");
    println!("Random password with custom characters: {}", password);
}
