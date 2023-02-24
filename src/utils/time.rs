use time::OffsetDateTime;

pub fn seconds() -> u64 {
    let current_time = OffsetDateTime::now_utc();
    current_time.unix_timestamp() as u64
}
