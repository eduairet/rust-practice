use algorithms::{generate_random_numbers, generate_random_numbers_in_range, guess_dice_roll};

fn main() {
    // Generate random numbers
    println!("Random u8: {}", generate_random_numbers::<u8>());
    println!("Random u16: {}", generate_random_numbers::<u16>());
    println!("Random u32: {}", generate_random_numbers::<u32>());
    println!("Random i32: {}", generate_random_numbers::<i32>());
    println!("Random f64: {}", generate_random_numbers::<f64>());
    // Generate random numbers in range
    println!("Integer: {}", generate_random_numbers_in_range(5, 20));
    println!("Float: {}", generate_random_numbers_in_range(2.5, 4.5));
    // Guess dice roll
    let guess = 3;
    let rolls = 3;
    let message = guess_dice_roll(guess, rolls);
    println!("{}", message);
}
