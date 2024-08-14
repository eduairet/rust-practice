use file_system::read_file_lines;

#[cfg(test)]
mod tests_read_write {
    use super::*;

    #[test]
    fn test_read_write() {
        let file_path = "tests/test_read.txt";
        let lines = read_file_lines(file_path);
        assert_eq!(lines, vec!["Reading from test_read.txt", ":)"]);
    }
}
