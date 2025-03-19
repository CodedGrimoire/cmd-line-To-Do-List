use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc, Datelike};

// Format a timestamp as a date string
pub fn format_date(timestamp: i64) -> String {
    let date_time = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let local_date = DateTime::<Local>::from_naive_utc_and_offset(
        date_time, 
        *Local::now().offset()
    );
    format!("{}", local_date.format("%b %d, %Y"))
}

// Format a timestamp as a date and time string
pub fn format_date_time(timestamp: i64) -> String {
    let date_time = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let local_date = DateTime::<Local>::from_naive_utc_and_offset(
        date_time, 
        *Local::now().offset()
    );
    format!("{}", local_date.format("%b %d, %Y at %H:%M"))
}

// Convert a date to a timestamp (end of day)
pub fn date_to_timestamp(date: NaiveDate) -> i64 {
    let midnight = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    let datetime = NaiveDateTime::new(date, midnight);
    datetime.and_utc().timestamp()
}

// Get days in month
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // Leap year check
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        },
        _ => unreachable!()
    }
}

// Get formatted time remaining or overdue
pub fn format_time_remaining(deadline_timestamp: i64, now_timestamp: i64) -> (String, bool) {
    let time_diff = deadline_timestamp - now_timestamp;
    let is_overdue = time_diff < 0;
    
    let time_text = if is_overdue {
        // Overdue
        let hours = (-time_diff) / 3600;
        if hours < 24 {
            format!("Overdue by {} hours", hours)
        } else {
            let days = hours / 24;
            format!("Overdue by {} days", days)
        }
    } else {
        // Upcoming
        let hours = time_diff / 3600;
        if hours < 24 {
            format!("{} hours remaining", hours)
        } else {
            let days = hours / 24;
            format!("{} days remaining", days)
        }
    };
    
    (time_text, is_overdue)
}

// Check if a date is today
pub fn is_today(date: NaiveDate) -> bool {
    Local::now().date_naive() == date
}

// Check if a timestamp falls on a specific day
pub fn timestamp_matches_day(timestamp: i64, year: i32, month: u32, day: u32) -> bool {
    if let Some(dt) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
        let date = dt.date();
        date.year() == year && date.month() == month && date.day() == day
    } else {
        false
    }
}

// Check if a timestamp is today
pub fn timestamp_is_today(timestamp: i64) -> bool {
    if let Some(dt) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
        is_today(dt.date())
    } else {
        false
    }
}

// Check if a timestamp is approaching (within 24 hours)
pub fn is_approaching_deadline(timestamp: i64) -> bool {
    let now = Utc::now().timestamp();
    let time_left = timestamp - now;
    time_left > 0 && time_left < 86400 // Less than 24 hours
}

// Check if a timestamp is in the future
pub fn is_future(timestamp: i64) -> bool {
    timestamp > Utc::now().timestamp()
}