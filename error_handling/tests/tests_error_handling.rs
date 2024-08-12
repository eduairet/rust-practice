use error_handling::{read_uptime, Rgb};

#[cfg(test)]
mod tests_error_handling {
    use super::*;

    #[test]
    fn test_read_uptime() {
        let uptime = read_uptime("Cargo.toml");
        print!("{:?}", uptime);
        assert!(uptime.is_ok());

        let uptime = read_uptime("non_existent_file");
        print!("{:?}", uptime);
        assert!(uptime.is_err());
    }

    #[test]
    fn test_parse_rgb_from_csv() {
        let csv_data = b"red,blue,green\n255,0,0\n";
        let rgb = Rgb::from_reader(csv_data).unwrap();
        println!("{:?}", rgb);
        assert_eq!(rgb.red, 255);

        let csv_data_invalid = b"red,blue,green\n255,0\n";
        let rgb = Rgb::from_reader(csv_data_invalid);
        if let Err(e) = rgb {
            eprint!("{:?}", e);
            assert_eq!(e.to_string(), "Cannot deserialize RGB color: CSV error: record 1 (line: 2, byte: 15): found record with 2 fields, but the previous record has 3 fields");
        };
    }
}
