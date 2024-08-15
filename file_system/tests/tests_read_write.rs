use file_system::{avoid_writing_reading_same_file, read_file_lines, write_to_file};
use std::{
    fs::{remove_file, File},
    path::Path,
};

#[cfg(test)]
mod tests_read_write {
    use super::*;

    #[test]
    fn test_read_write() {
        let file_path = "tests/test_read.txt";
        let lines = read_file_lines(file_path);
        assert_eq!(lines, vec!["Reading from test_read.txt", ":)"]);
    }

    #[test]
    #[ignore]
    fn test_write() {
        let file_path = "tests/test_write.txt";
        let content = "Writing to test_write.txt";
        let lines = write_to_file(file_path, content);
        assert_eq!(lines, vec!["Writing to test_write.txt"]);

        remove_file(file_path).unwrap();
        assert!(!Path::new(file_path).exists());
    }

    #[test]
    #[ignore]
    fn test_avoid_writing_reading_same_file() {
        let file_path = "tests/test_avoid.txt";

        File::create(file_path).unwrap();
        assert!(Path::new(file_path).exists());

        let content = "Writing to test_avoid.txt";
        let lines = avoid_writing_reading_same_file(file_path, content).unwrap();
        assert_eq!(lines, vec!["Writing to test_avoid.txt"]);

        remove_file(file_path).unwrap();
        assert!(!Path::new(file_path).exists());
    }
}
