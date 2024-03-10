use chrono::Timelike;

pub fn app_name() -> &'static str {
    "listening"
}

pub fn during_today() -> (i64, i64) {
    let now = chrono::Local::now();
    let now_timestamp = now.timestamp();
    let zero = now
        .with_hour(0)
        .and_then(|x| x.with_minute(0))
        .and_then(|x| x.with_second(0))
        .unwrap();
    (zero.timestamp(), now_timestamp)
}
pub fn during_yesterday() -> (i64, i64) {
    let now = chrono::Local::now();
    let zero = now
        .with_hour(0)
        .and_then(|x| x.with_minute(0))
        .and_then(|x| x.with_second(0))
        .unwrap();
    (zero.timestamp() - 24 * 60 * 60, zero.timestamp())
}
