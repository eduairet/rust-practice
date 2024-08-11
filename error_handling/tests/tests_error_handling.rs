use error_handling::read_uptime;

#[cfg(test)]
mod tests_error_handling {
    use super::*;

    #[test]
    fn test_read_uptime() {
        let uptime = read_uptime("Cargo.toml");
        print!("{:?}", uptime);
        assert_eq!(uptime.unwrap(), 69);

        let uptime = read_uptime("non_existent_file");
        print!("{:?}", uptime);
        assert!(uptime.is_err());
    }
}
