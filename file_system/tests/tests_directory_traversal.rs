use file_system::{
    find_all_files_recursively, find_loops_for_given_path,
    recursively_calculate_file_sizes_at_given_depth, recursively_find_all_files_with_predicate,
    recursively_find_duplicate_file_names, search_modified_files_in_current_dir,
    traverse_directories_skipping_dotfiles,
};
use std::path::PathBuf;

#[cfg(test)]
mod tests_directory_traversal {

    use super::*;

    #[test]
    fn test_search_modified_files_in_current_dir() {
        let hours_back = 24;
        let modified_files = search_modified_files_in_current_dir(hours_back);
        println!("{:?}", modified_files);
        assert!(modified_files.is_ok());
    }

    #[test]
    #[ignore]
    fn test_find_loops_for_given_path() {
        let path = "/tmp/foo/bar/baz/qux/bar/baz";
        let loops = find_loops_for_given_path(path).unwrap();
        println!("{:?}", loops);
        assert_eq!(
            loops,
            Some((
                PathBuf::from("/tmp/foo"),
                PathBuf::from("/tmp/foo/bar/baz/qux")
            ))
        );
    }

    #[test]
    fn test_recursively_find_duplicate_file_names() {
        let path = ".";
        let duplicates = recursively_find_duplicate_file_names(path).unwrap();
        println!("{:?}", duplicates);
        assert!(!duplicates);
    }

    #[test]
    fn test_recursively_find_all_files_with_predicate() {
        let path = ".";
        let predicate = ".rs";
        let files = recursively_find_all_files_with_predicate(path, predicate).unwrap();
        println!("{:?}", files);
        assert!(files.contains(&"tests_directory_traversal.rs".to_string()));
    }

    #[test]
    fn test_traverse_directories_skipping_dotfiles() {
        let path = ".";
        let files = traverse_directories_skipping_dotfiles(path).unwrap();
        println!("{:?}", files);
        assert!(!files
            .iter()
            .any(|file| file.file_name().to_str().unwrap() == ".env"));
        assert!(files
            .iter()
            .any(|file| file.file_name().to_str().unwrap() == "Cargo.toml"));
    }

    #[test]
    fn test_recursively_calculate_file_sizes_at_given_depth() {
        let path = ".";
        let min_depth = 1;
        let max_depth = 2;
        let file_sizes =
            recursively_calculate_file_sizes_at_given_depth(path, min_depth, max_depth);
        println!("{:?}", file_sizes);
        assert!(file_sizes.len() > 0);
    }

    #[test]
    fn test_find_all_files_recursively() {
        let path = "**/*.png";
        let files = find_all_files_recursively(path).unwrap();
        println!("{:?}", files);
        assert!(files.contains(&"src/brisket.png".to_string()));
    }
}
