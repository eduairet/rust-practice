use encoding::read_csv_records;

#[cfg(test)]
mod tests_csv_processing {
    use super::*;

    #[test]
    fn test_read_csv_records() {
        let csv_data = "name,age\nAlice,30\nBob,25\n";
        let records = read_csv_records(csv_data).unwrap();
        print!("{:?}", records);
        assert_eq!(records, vec!["Alice30", "Bob25"]);
    }
}
