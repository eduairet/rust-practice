use file_system::search_modified_files_in_current_dir;

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
}
