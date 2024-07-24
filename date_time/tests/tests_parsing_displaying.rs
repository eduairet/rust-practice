use chrono::Utc;
use date_time::examine_date_time;
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
}
