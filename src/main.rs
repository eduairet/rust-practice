use algorithms::{
    generate_random_numbers, generate_random_numbers_in_range, generate_random_password,
    generate_random_password_with_custom_characters, generate_random_values_from_custom_type,
    guess_dice_roll, sort_num_vector, sort_people,
};
use command_line::{create_cmd, formatted_cli_message};
use shared::Point;
use std::path::Path;

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
    // Sort a vector of numbers
    let mut vec_i32: Vec<i32> = vec![3, 1, 2];
    sort_num_vector(&mut vec_i32);
    println!("{:?}", vec_i32);
    let mut vec_f64: Vec<f64> = vec![3.0, 1.0, 2.0];
    sort_num_vector(&mut vec_f64);
    println!("{:?}", vec_f64);
    // Sort a vector of people
    let mut people = vec![
        shared::Person::new("John".to_string(), 25),
        shared::Person::new("Jane".to_string(), 20),
        shared::Person::new("Alice".to_string(), 30),
    ];
    sort_people(&mut people, true, false);
    println!("{:?}", people);
    let mut people = vec![
        shared::Person::new("Paul".to_string(), 69),
        shared::Person::new("Emile".to_string(), 18),
        shared::Person::new("Marie".to_string(), 25),
    ];
    sort_people(&mut people, false, true);
    println!("{:?}", people);
    let mut people = vec![
        shared::Person::new("Emilio".to_string(), 48),
        shared::Person::new("Álvaro".to_string(), 30),
        shared::Person::new("Joaquín".to_string(), 25),
    ];
    sort_people(&mut people, true, true);
    println!("{:?}", people);
    // Command line
    let cmd = create_cmd().get_matches();
    let file: &String = cmd.get_one("file").unwrap();
    println!("The file passed is {}", file);
    let num: Option<&String> = cmd.get_one("num");
    match num {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        },
    }
    // Formatted CLI message
    let message = "Hello, world!";
    let colors = vec![
        shared::TerminalColor::new(Some(shared::Colors::Red), true),
        shared::TerminalColor::new(None, false),
    ];
    formatted_cli_message(message, colors);
    // Compression
    let root = Path::new(".").parent();
    print!("Root: {:?}", root);
}
