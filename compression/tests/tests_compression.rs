use compression::{compress_file, decompress_file, decompress_removing_prefix};
use std::{
    fs::{remove_dir_all, remove_file},
    path::Path,
};

#[cfg(test)]
mod tests_compression {
    use super::*;

    #[test]
    fn test_compress_file() {
        let parent_dir = "compress_test";
        let logs_dir = "logs";
        let nested_dir = "/var/log";
        let logs_path = Path::new(parent_dir).join(logs_dir);
        let nested_dir_path = Path::new(parent_dir).join(logs_dir).join(nested_dir);
        let dirs: Vec<&str> = vec![logs_path.to_str().unwrap(), nested_dir];
        let file = "compress_test.tar.gz";
        compress_file(&file, dirs).unwrap();
        assert!(Path::new(&file).exists());
        assert!(Path::new(&logs_path.to_str().unwrap().to_string()).exists());
        assert!(Path::new(&nested_dir_path.to_str().unwrap().to_string()).exists());
        remove_dir_all(parent_dir).unwrap();
        remove_file(file).unwrap();
        assert!(!Path::new(&file).exists());
        assert!(!Path::new(&logs_path.to_str().unwrap().to_string()).exists());
    }

    #[test]
    fn test_decompress_file() {
        let parent_dir = "decompress_test";
        let logs_dir = "logs";
        let logs_path = Path::new(parent_dir).join(logs_dir);
        let dirs: Vec<&str> = vec![logs_path.to_str().unwrap()];
        let file = "decompress_test.tar.gz";
        compress_file(&file, dirs).unwrap();
        let output_dir = "test_output";
        decompress_file(&file, &output_dir).unwrap();
        assert!(Path::new(output_dir).exists());
        assert!(Path::new(parent_dir).exists());
        assert!(Path::new(file).exists());
        remove_dir_all(output_dir).unwrap();
        remove_dir_all(parent_dir).unwrap();
        remove_file(file).unwrap();
        assert!(!Path::new(output_dir).exists());
        assert!(!Path::new(parent_dir).exists());
        assert!(!Path::new(file).exists());
    }

    #[test]
    fn test_decompress_removing_prefix() {
        let parent_dir = "decompress_prefix_test";
        let logs_dir = "logs";
        let logs_path = Path::new(parent_dir).join(logs_dir);
        let dirs: Vec<&str> = vec![logs_path.to_str().unwrap()];
        let file = "decompress_prefix_test.tar.gz";
        compress_file(&file, dirs).unwrap();
        let output_dir = "test_prefix_output";
        decompress_removing_prefix(&file, &output_dir).unwrap();
        assert!(Path::new(parent_dir).exists());
        assert!(Path::new(file).exists());
        remove_dir_all(parent_dir).unwrap();
        remove_file(file).unwrap();
        assert!(!Path::new(parent_dir).exists());
        assert!(!Path::new(file).exists());
    }
}
