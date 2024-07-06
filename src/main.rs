use algorithms::*;
use command_line::*;
use compression::*;
use dirs::home_dir;
use shared::*;
use std::{
    env,
    fs::{remove_dir_all, remove_file, File},
};
use threads::*;

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
        Person::new("John".to_string(), 25),
        Person::new("Jane".to_string(), 20),
        Person::new("Alice".to_string(), 30),
    ];
    sort_people(&mut people, true, false);
    println!("{:?}", people);
    let mut people = vec![
        Person::new("Paul".to_string(), 69),
        Person::new("Emile".to_string(), 18),
        Person::new("Marie".to_string(), 25),
    ];
    sort_people(&mut people, false, true);
    println!("{:?}", people);
    let mut people = vec![
        Person::new("Emilio".to_string(), 48),
        Person::new("Álvaro".to_string(), 30),
        Person::new("Joaquín".to_string(), 25),
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
        TerminalColor::new(Some(Colors::Red), true),
        TerminalColor::new(None, false),
    ];
    formatted_cli_message(message, colors);
    // Compression
    let tar_file = "test.tar.gz";
    let copy_path = "backup/logs";
    let copy_pat_src = "/var/log";
    let output_dir = "output_dir";
    compress_file(tar_file, copy_path, copy_pat_src).unwrap();
    println!(
        "Compressed file created. {}/{}\n",
        env::current_dir().unwrap().display(),
        tar_file
    );
    decompress_file(tar_file, output_dir).unwrap();
    println!(
        "Decompressed file created. {}/{}\n",
        env::current_dir().unwrap().display(),
        output_dir
    );
    remove_dir_all(output_dir).unwrap();
    decompress_removing_prefix(tar_file, copy_path, output_dir).unwrap();
    println!(
        "Decompressed file with prefix created. {}/{}\n",
        env::current_dir().unwrap().display(),
        output_dir
    );
    remove_dir_all(output_dir).unwrap();
    remove_file(tar_file).unwrap();
    // Threads
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let threshold = 5;
    println!("Max value: {:?}", find_max(&arr, threshold));
    parallel_pipeline(10, 2);
    let num_messages = 3;
    let (.., receiver) = pass_data_between_two_threads(num_messages);
    for _ in 0..num_messages {
        println!("Received: {}", receiver.recv().unwrap());
    }
    // Global state
    let global_state = create_global_state();
    global_state_insert("BTC", global_state).unwrap();
    global_state_insert("ETH", global_state).unwrap();
    {
        let global_state = global_state.lock().unwrap();
        println!("Cryptos: {:?}", global_state);
    }
    // Calculate digest
    let home_dir = home_dir().unwrap();
    let file_out = home_dir.join("Downloads/test.iso");
    File::create(&file_out).expect("Failed to create .iso file");
    let rx = calculate_sha256_sum_of_iso_files().unwrap();
    for t in rx.iter() {
        let (sha, path) = t.unwrap();
        println!("{:?} {:?}", sha, path);
    }
    remove_file(file_out).expect("Failed to remove .iso file");
    // Draw fractal
    let output_file = "fractal.png";
    draw_fractal(1024, 1024, 300, output_file).unwrap();
    println!(
        "Fractal image created. {}/{}",
        env::current_dir().unwrap().display(),
        output_file
    );
}
