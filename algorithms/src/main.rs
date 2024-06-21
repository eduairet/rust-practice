use algorithms::generate_random_numbers;

fn main() {
    // Generate random numbers
    println!("Random u8: {}", generate_random_numbers::<u8>());
    println!("Random u16: {}", generate_random_numbers::<u16>());
    println!("Random u32: {}", generate_random_numbers::<u32>());
    println!("Random i32: {}", generate_random_numbers::<i32>());
    println!("Random f64: {}", generate_random_numbers::<f64>());
}
