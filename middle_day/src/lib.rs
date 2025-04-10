pub use chrono::{Datelike, NaiveDate, Weekday as wd};


pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    let num_days = if is_leap { 366 } else { 365 };

    if num_days % 2 == 0 {
        return None;
    }

    let middle = num_days / 2 + 1;
    NaiveDate::from_yo_opt(year, middle as u32).map(|d| d.weekday())
}
