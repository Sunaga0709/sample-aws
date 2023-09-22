use chrono::Local;

#[allow(unused)]
pub fn now_unix() -> i64 {
    Local::now().timestamp()
}
