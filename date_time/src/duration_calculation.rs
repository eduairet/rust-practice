use chrono::{DateTime, Utc};
use std::time::Instant;

/// Get the elapsed time between two code sections
///
/// # Arguments
///
/// * `callback` - A closure that contains the code sections to measure the elapsed time
///
/// # Example
///
/// ```
/// use date_time::get_two_code_sections_elapsed_time;
///
/// let result = get_two_code_sections_elapsed_time(|| {
///    let mut _sum: u64 = 0;
///    for i in 0..200000000 {
///       _sum += i;
///    }
/// });
/// print!("{}", result);
/// ```
pub fn get_two_code_sections_elapsed_time(callback: fn()) -> String {
    let start = Instant::now();
    callback();
    let duration = start.elapsed();
    format!("Elapsed time: {:?}", duration)
}

/// Check the number of days from a given date time
///
/// # Arguments
///
/// * `date_time` - A date time to check the number of days from
///
/// # Example
///
/// ```
/// use chrono::{Duration, Utc};
/// use date_time::check_days_from_date_time;
///
/// let date_time = Utc::now() + Duration::days(2);
/// let result = check_days_from_date_time(date_time);
/// assert_eq!(result, "1 days ahead");
/// ```
pub fn check_days_from_date_time(date_time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now - date_time;
    let num_days = duration.num_days();

    match num_days {
        n if n < 0 => format!("{} days ahead", -n),
        0 => String::from("Today"),
        1 => String::from("Yesterday"),
        2..=7 => String::from("This week"),
        8..=14 => String::from("Last week"),
        _ => format!("{} days ago", num_days),
    }
}
