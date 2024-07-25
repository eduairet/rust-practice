use algorithms::*;
use chrono::{Duration, Local, TimeZone, Utc};
use command_line::*;
use compression::*;
use cryptography::*;
use data_encoding::HEXUPPER;
use database::*;
use date_time::*;
use development_tools::*;
use dirs::home_dir;
use log::error;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rayon::prelude::*;
use ring::hmac;
use rusqlite::params;
use shared::*;
use std::{
    env,
    fs::{remove_dir_all, remove_file, File},
    io::{BufReader, Write},
};
use threads::*;

fn main() {
    // Log debug message
    env_logger::init();
    log_debug_message("This is a debug message");
    // Log error message
    let error_log = log_error_message("This is an error message");
    if let Err(err) = error_log {
        error!("Failed to execute query: {}", err);
    }

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
    // Mutate array in parallel
    let mut array = [1, 2, 3, 4, 5];
    let result = mutate_array_in_parallel(&mut array, 1);
    println!("{:?}", result);
    // Match predicate in parallel
    let array = [2, 4, 6, 8, 10];
    let predicate = |x| x % 2 == 0;
    let is_all = true;
    let result = match_predicate_in_parallel(&array, is_all, predicate);
    println!("{}", result);
    // Sort a vector of people in parallel
    let mut vec = vec![String::new(); 10];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..3).map(|_| rng.sample(Alphanumeric) as char).collect()
    });
    println!("{:?}", vec);
    let sorted_parallel = sort_string_vector_in_parallel(&mut vec);
    println!("{:?}", sorted_parallel);
    // Generate thumbnails in parallel
    let thumb_dir = "thumbnails";
    generate_jpg_thumbnails_in_parallel(thumb_dir).unwrap();
    // SHA256 digest
    let file = "file.txt";
    let mut output = File::create(file).unwrap();
    write!(output, "Hello, world!").unwrap();
    let input = File::open(file).unwrap();
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader).unwrap();
    println!("SHA256: {}", HEXUPPER.encode(digest.as_ref()));
    remove_file(file).unwrap();
    // HMAC verification
    let message = "Hello, world!";
    let (key, signature) = sign_and_verify_hmac(message).unwrap();
    println!("Signature: {:?}", signature);
    hmac::verify(&key, message.as_bytes(), signature.as_ref()).unwrap();
    // Salt and hash password with PBKDF2
    let password = "ABCabc123?";
    let (salt, hash) = salt_and_hash_password_with_pbkdf2(password).unwrap();
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("Hash: {}", HEXUPPER.encode(&hash));
    // SQLite
    let database = "test.db";
    delete_cats_database(database).unwrap();
    create_sqlite_cats_database(database).unwrap();
    let cats: Vec<Cat> = vec![
        Cat {
            name: String::from("Fluffy"),
            color: String::from("White"),
        },
        Cat {
            name: String::from("Whiskers"),
            color: String::from("Black"),
        },
        Cat {
            name: String::from("Socks"),
            color: String::from("Gray"),
        },
    ];
    let result = insert_select_cats(database, &cats).unwrap();
    println!("{:?}", result);
    let query = "INSERT INTO cat_colors (name) VALUES (?1)";
    let transaction =
        submit_db_transaction(database, query, params!["Red"], TransactionType::Commit);
    println!("{:?}", transaction.unwrap());
    delete_cats_database(database).unwrap();
    remove_file(database).unwrap();
    // Calculate elapsed time
    let result = get_two_code_sections_elapsed_time(|| {
        let mut rng = thread_rng();
        let target_number = 50;
        let mut _turn = 0;
        loop {
            _turn += 1;
            let current_number = rng.gen_range(1..101);
            if current_number == target_number {
                break;
            }
        }
    });
    println!("Number guessed in: {}", result);
    // Check days from date time
    let date_time = Utc::now() + Duration::days(2);
    let result = check_days_from_date_time(date_time);
    println!("{}", result);
    // Offset date timezone
    let date_time = Local::now();
    let (adjusted_datetime, ..) = offset_date_timezone(date_time, 2);
    println!("Adjusted datetime: {:?}", adjusted_datetime);
    // Examine date time
    let date_time = Utc.with_ymd_and_hms(1992, 9, 4, 18, 0, 0).unwrap();
    let (time, date) = examine_date_time(date_time);
    println!("{}", time);
    println!("{}", date);
    // Convert date to Unix timestamp
    let date_string = "2000-01-01 00:00:01";
    let date_to_unix = convert_date_to_unix(date_string);
    println!("{} to unix: {:?}", date_string, date_to_unix);
    // Convert Unix timestamp to date
    let unix_timestamp = 946684801;
    let unix_to_date = convert_unix_to_date(unix_timestamp);
    println!("{} to date: {:?}", unix_timestamp, unix_to_date);
    // Get formatted date time
    let date_time = Utc::now();
    let (simple, rfc2822, rfc3339, custom) =
        get_formatted_date_time(date_time, "%Y-%m-%d %H:%M:%S");
    println!("Simple: {}", simple);
    println!("RFC2822: {}", rfc2822);
    println!("RFC3339: {}", rfc3339);
    println!("Custom: {}", custom);
    // Parse date time
    let datetime_string = "2000-01-01 00:00:01";
    let format = "%Y-%m-%d %H:%M:%S";
    let result = parse_string_to_datetime(datetime_string, format).unwrap();
    println!("{:?}", result);
}
