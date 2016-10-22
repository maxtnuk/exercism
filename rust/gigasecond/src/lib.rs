extern crate chrono;
use chrono::*;

pub fn after<T>(start_date: DateTime<T>)-> DateTime<T>
where T: TimeZone{
    let human_life: i64=1_000_000_000;
    start_date.checked_add(Duration::seconds(human_life)).unwrap()
}
