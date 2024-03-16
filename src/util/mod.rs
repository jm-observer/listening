#![allow(dead_code)]
use chrono::{DateTime, Days, Local, Timelike};

pub fn app_name() -> &'static str {
    "listening"
}

pub fn now_str() -> String {
    format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
}

pub fn date_time_str(date_time: DateTime<Local>) -> String {
    format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"))
}

pub fn today_zero() -> DateTime<Local> {
    let zero = chrono::Local::now()
        .with_hour(0)
        .and_then(|x| x.with_minute(0))
        .and_then(|x| x.with_second(0))
        .unwrap();
    zero
}
pub fn during_today() -> (String, String) {
    let now = chrono::Local::now();
    let zero = today_zero();
    (date_time_str(zero), date_time_str(now))
}
pub fn during_yesterday() -> (String, String) {
    let zero = today_zero();
    let yesterday = zero.checked_sub_days(Days::new(1)).unwrap();
    (date_time_str(yesterday), date_time_str(zero))
}
