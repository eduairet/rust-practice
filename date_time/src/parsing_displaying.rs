use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

/// Examine the current date and time
///
/// # Arguments
///
/// * `data_time` - A `DateTime<Utc>` instance
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
pub fn examine_date_time(data_time: DateTime<Utc>) -> (String, String) {
    let (is_pm, hour) = data_time.hour12();
    let mut time = format!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        data_time.minute(),
        data_time.second(),
        if is_pm { "PM" } else { "AM" }
    );
    time += &format!(
        "\nAnd there have been {} seconds since midnight",
        data_time.num_seconds_from_midnight()
    );

    let (is_common_era, year) = data_time.year_ce();
    let mut date = format!(
        "The current UTC date is {}/{:02}/{:02} {}",
        year,
        data_time.month(),
        data_time.day(),
        if is_common_era { "CE" } else { "BCE" }
    );
    date += &format!(
        "\nAnd the Common Era began {} days ago",
        data_time.num_days_from_ce()
    );

    (time, date)
}

/// Convert a date string to a Unix timestamp
///
/// # Arguments
///
/// * `date` - A date string in the format "YYYY-MM-DD HH:MM:SS"
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
/// let date_string = "2000-01-01 00:00:01";
/// let date_to_unix = convert_date_to_unix(date_string);
/// println!("{} to unix: {:?}", date_string, date_to_unix);
/// ```
pub fn convert_date_to_unix(date: &str) -> i64 {
    NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
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
