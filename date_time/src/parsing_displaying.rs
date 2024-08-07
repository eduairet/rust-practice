use chrono::{DateTime, Datelike, NaiveDateTime, ParseError, TimeZone, Timelike, Utc};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref SIMPLE_DATETIME_REGEX: Regex =
        Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}").unwrap();
    pub static ref RFC2822_DATETIME_REGEX: Regex =
        Regex::new(r"\w{3}, \d{1,2} \w{3} \d{4} \d{2}:\d{2}:\d{2} [+-]\d{4}").unwrap();
    pub static ref RFC3339_DATETIME_REGEX: Regex =
        Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+[+-]\d{2}:\d{2}").unwrap();
}

/// Examine the current date and time
///
/// # Arguments
///
/// * `datetime` - A `DateTime<Utc>` instance
///
/// # Returns
///
/// A tuple containing the time and date information
///
/// # Examples
///
/// ```
/// use chrono::Utc;
/// use date_time::examine_date_time;
///
/// let date_time = Utc::now();
/// let (time, date) = examine_date_time(date_time);
/// println!("{}", time);
/// println!("{}", date);
/// ```
pub fn examine_date_time(datetime: DateTime<Utc>) -> (String, String) {
    let (is_pm, hour) = datetime.hour12();
    let mut time = format!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        datetime.minute(),
        datetime.second(),
        if is_pm { "PM" } else { "AM" }
    );
    time += &format!(
        "\nAnd there have been {} seconds since midnight",
        datetime.num_seconds_from_midnight()
    );

    let (is_common_era, year) = datetime.year_ce();
    let mut date = format!(
        "The current UTC date is {}/{:02}/{:02} {}",
        year,
        datetime.month(),
        datetime.day(),
        if is_common_era { "CE" } else { "BCE" }
    );
    date += &format!(
        "\nAnd the Common Era began {} days ago",
        datetime.num_days_from_ce()
    );

    (time, date)
}

/// Convert a date string to a Unix timestamp
///
/// # Arguments
///
/// * `datetime_string` - A date string in the format "YYYY-MM-DD HH:MM:SS"
///
/// # Returns
///
/// A Unix timestamp
///
/// # Examples
///
/// ```
/// use date_time::convert_date_to_unix;
///
/// let datetime_string = "2000-01-01 00:00:01";
/// let date_to_unix = convert_date_to_unix(datetime_string);
/// println!("{} to unix: {:?}", datetime_string, date_to_unix);
/// ```
pub fn convert_date_to_unix(datetime_string: &str) -> i64 {
    NaiveDateTime::parse_from_str(datetime_string, "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .and_utc()
        .timestamp()
}

/// Convert a Unix timestamp to a date string
///
/// # Arguments
///
/// * `unix_timestamp` - A Unix timestamp
///
/// # Returns
///
/// A date string in the format "YYYY-MM-DD HH:MM:SS"
///
/// # Examples
///
/// ```
/// use date_time::convert_unix_to_date;
///
/// let unix_timestamp = 946684801;
/// let unix_to_date = convert_unix_to_date(unix_timestamp);
/// println!("{} to date: {:?}", unix_timestamp, unix_to_date);
/// ```
pub fn convert_unix_to_date(unix_timestamp: i64) -> String {
    let date_time = DateTime::<Utc>::from_timestamp(unix_timestamp, 0).unwrap();
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Get a formatted date and time
///
/// # Arguments
///
/// * `datetime` - A `DateTime<Utc>` instance
/// * `format` - A format string
///
/// # Returns
///
/// A tuple containing the simple, RFC 2822, RFC 3339, and custom formatted date and time
///
/// # Examples
///
/// ```
/// use chrono::Utc;
/// use date_time::get_formatted_date_time;
///
/// let date_time = Utc::now();
/// let (simple, rfc2822, rfc3339, custom) = get_formatted_date_time(date_time, "%Y-%m-%d %H:%M:%S");
/// println!("{}", simple);
/// println!("{}", rfc2822);
/// println!("{}", rfc3339);
/// println!("{}", custom);
/// ```
pub fn get_formatted_date_time(
    datetime: DateTime<Utc>,
    format: &str,
) -> (String, String, String, String) {
    let simple = format!("UTC now is: {}", datetime);
    let rfc2822 = format!("UTC now in RFC 2822 is: {}", datetime.to_rfc2822());
    let rfc3339 = format!("UTC now in RFC 3339 is: {}", datetime.to_rfc3339());
    let custom = format!("UTC now in a custom format is: {}", datetime.format(format));
    (simple, rfc2822, rfc3339, custom)
}

/// Parse a date and time string to a `DateTime<Utc>` instance
///
/// # Arguments
///
/// * `datetime_string` - A date and time string
/// * `format` - A format string
///
/// # Returns
///
/// A `DateTime<Utc>` instance
///
/// # Examples
///
/// ```
/// use date_time::parse_string_to_datetime;
///
/// let datetime_string = "2000-01-01 00:00:01";
/// let format = "%Y-%m-%d %H:%M:%S";
/// let result = parse_string_to_datetime(datetime_string, format).unwrap();
/// println!("{:?}", result);
/// ```
pub fn parse_string_to_datetime(
    datetime_string: &str,
    format: &str,
) -> Result<DateTime<Utc>, ParseError> {
    let result = match datetime_string {
        _ if RFC2822_DATETIME_REGEX.is_match(datetime_string) => {
            DateTime::parse_from_rfc2822(datetime_string)?.to_utc()
        }
        _ if RFC3339_DATETIME_REGEX.is_match(datetime_string) => {
            DateTime::parse_from_rfc3339(datetime_string)?.to_utc()
        }
        _ => {
            let naive_datetime = NaiveDateTime::parse_from_str(datetime_string, format)?;
            Utc.from_utc_datetime(&naive_datetime)
        }
    };

    Ok(result)
}
