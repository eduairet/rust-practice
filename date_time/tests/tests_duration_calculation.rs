use chrono::{Duration, Local, Utc};
use date_time::{
    check_days_from_date_time, get_two_code_sections_elapsed_time, offset_date_timezone,
};

#[cfg(test)]
mod tests_duration_calculation {
    use super::*;

    #[test]
    fn test_get_two_code_sections_elapsed_time() {
        let result = get_two_code_sections_elapsed_time(|| {
            let mut _sum: u64 = 0;
            for i in 0..200000000 {
                _sum += i;
            }
        });
        print!("{}", result);
        assert_eq!(result.contains("Elapsed time:"), true);
    }

    #[test]
    fn test_check_days_from_date_time() {
        let date_time = Utc::now() + Duration::days(2);
        let result = check_days_from_date_time(date_time);
        assert_eq!(result, "1 days ahead");

        let date_time = Utc::now();
        let result = check_days_from_date_time(date_time);
        assert_eq!(result, "Today");

        let date_time = Utc::now() - Duration::days(1);
        let result = check_days_from_date_time(date_time);
        assert_eq!(result, "Yesterday");

        let date_time = Utc::now() - Duration::days(2);
        let result = check_days_from_date_time(date_time);
        assert_eq!(result, "This week");

        let date_time = Utc::now() - Duration::days(8);
        let result = check_days_from_date_time(date_time);
        assert_eq!(result, "Last week");

        let date_time = Utc::now() - Duration::days(15);
        let result = check_days_from_date_time(date_time);
        assert_eq!(result, "15 days ago");
    }

    #[test]
    fn test_offset_date_timezone() {
        let date_time = Local::now();
        let (adjusted_datetime, ..) = offset_date_timezone(date_time, 2);
        println!("Adjusted datetime: {:?}", adjusted_datetime);
        assert_eq!(adjusted_datetime.offset().local_minus_utc(), 2 * 3600);
    }
}
