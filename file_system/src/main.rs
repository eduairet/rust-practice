use file_system::avoid_writing_reading_same_file;

fn main() {
    // Run cargo run >> test_avoid_error.txt to get the error
    let file_path = "test_avoid_error.txt";
    let content = "Writing to test_avoid_error.txt";
    avoid_writing_reading_same_file(file_path, content).unwrap();
}
