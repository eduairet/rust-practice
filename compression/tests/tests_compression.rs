use compression::{compress_file, decompress_file, decompress_removing_prefix};
use std::{
    fs::{remove_dir_all, remove_file},
    path::Path,
};

#[cfg(test)]
mod tests_compression {
    use super::*;

    fn fixture_tar_gz(test_id: usize) -> (String, String) {
        let tar_file = format!("test_{}.tar.gz", test_id);
        let copy_path = "backup/logs";
        let copy_pat_src = "/var/log";
        compress_file(&tar_file, copy_path, copy_pat_src).unwrap();
        assert!(Path::new(&tar_file).exists());
        (tar_file.to_string(), copy_path.to_string())
    }

    #[test]
    fn test_compress_file() {
        let (tar_file, ..) = fixture_tar_gz(1);
        remove_file(tar_file).unwrap();
    }

    #[test]
    fn test_decompress_file() {
        let (tar_file, ..) = fixture_tar_gz(2);
        let output_dir = "output_dir";
        decompress_file(&tar_file, output_dir).unwrap();
        assert!(Path::new(&output_dir).exists());
        remove_dir_all(output_dir).unwrap();
        remove_file(tar_file).unwrap();
    }

    #[test]
    fn test_decompress_removing_prefix() {
        let (tar_file, copy_path) = fixture_tar_gz(3);
        let output_dir = "output_dir_prefix";
        decompress_removing_prefix(&tar_file, &copy_path, output_dir).unwrap();
        assert!(Path::new(output_dir).exists());
        remove_dir_all(output_dir).unwrap();
        remove_file(tar_file).unwrap();
    }
}
