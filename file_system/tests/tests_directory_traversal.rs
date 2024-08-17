use file_system::{find_loops_for_given_path, search_modified_files_in_current_dir};
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
}
