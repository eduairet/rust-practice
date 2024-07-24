use chrono::{NaiveDate, Utc};
use date_time::{
    convert_date_to_unix, convert_unix_to_date, examine_date_time, get_formatted_date_time,
    parse_string_to_datetime, RFC2822_DATETIME_REGEX, RFC3339_DATETIME_REGEX,
    SIMPLE_DATETIME_REGEX,
};
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

    #[test]
    fn test_get_formatted_date_time() {
        let date_time = Utc::now();
        let (simple, rfc2822, rfc3339, custom) =
            get_formatted_date_time(date_time, "%Y-%m-%d %H-%M-%S");

        println!("Simple: {}", simple);
        println!("RFC2822: {}", rfc2822);
        println!("RFC3339: {}", rfc3339);
        println!("Custom: {}", custom);

        assert!(
            SIMPLE_DATETIME_REGEX.is_match(&simple),
            "Simple output does not match expected format."
        );

        assert!(
            RFC2822_DATETIME_REGEX.is_match(&rfc2822),
            "RFC2822 output does not match expected format."
        );

        assert!(
            RFC3339_DATETIME_REGEX.is_match(&rfc3339),
            "RFC3339 output does not match expected format."
        );

        let custom_regex = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}-\d{2}-\d{2}").unwrap();
        assert!(
            custom_regex.is_match(&custom),
            "Custom output does not match expected format."
        );
    }

    #[test]
    fn test_parse_string_to_datetime() {
        let datetime_string_rfc2822 = "Tue, 1 Jul 2003 10:52:37 +0200";
        let datetime_string_rfc3339 = "2024-07-24T04:43:01.104103+00:00";
        let datetime_string_custom = "1996-12-19 16:39:57";

        let datetime_rfc2822 = parse_string_to_datetime(datetime_string_rfc2822, "");
        let datetime_rfc3339 = parse_string_to_datetime(datetime_string_rfc3339, "");
        let datetime_custom = parse_string_to_datetime(datetime_string_custom, "%Y-%m-%d %H:%M:%S");

        println!("RFC 2822 datetime: {:?}", datetime_rfc2822);
        println!("RFC 3339 datetime: {:?}", datetime_rfc3339);
        println!("Custom datetime: {:?}", datetime_custom);

        assert!(
            datetime_rfc2822.is_ok(),
            "Failed to parse RFC 2822 datetime string."
        );
        assert!(
            datetime_rfc3339.is_ok(),
            "Failed to parse RFC 3339 datetime string."
        );
        assert!(
            datetime_custom.is_ok(),
            "Failed to parse custom datetime string."
        );
    }
}
