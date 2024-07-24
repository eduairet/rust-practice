use chrono::{DateTime, Datelike, Timelike, Utc};

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
