use chrono::{NaiveDate, Utc};
use date_time::{convert_date_to_unix, convert_unix_to_date, examine_date_time};
use regex::Regex;

#[cfg(test)]
mod tests_parsing_displaying {

    use super::*;

    #[test]
    fn test_examine_date_time() {
        let date_time = Utc::now();
        let (time, date) = examine_date_time(date_time);
        println!("{}", time);
        println!("{}", date);

        let time_regex = Regex::new(
            r"The current UTC time is \d{2}:\d{2}:\d{2} (AM|PM)\nAnd there have been \d+ seconds since midnight"
        )
        .unwrap();
        assert!(
            time_regex.is_match(&time),
            "Time output does not match expected format."
        );

        let date_regex = Regex::new(
            r"The current UTC date is \d+/\d+/\d+ (CE|BCE)\nAnd the Common Era began \d+ days ago",
        )
        .unwrap();
        assert!(
            date_regex.is_match(&date),
            "Date output does not match expected format."
        );
    }

    #[test]
    fn test_convert_date_to_unix() {
        let date_string = "2000-01-01 00:00:01";

        let date_to_unix = convert_date_to_unix(date_string);
        let expected_date_to_unix = NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 1)
            .unwrap()
            .and_utc()
            .timestamp();
        print!("{} to unix: {:?}\n", date_string, date_to_unix);
        assert_eq!(date_to_unix, expected_date_to_unix);

        let unix_to_date = convert_unix_to_date(date_to_unix);
        print!("{} to date: {:?}\n", date_to_unix, unix_to_date);
        assert_eq!(unix_to_date, date_string);
    }
}
