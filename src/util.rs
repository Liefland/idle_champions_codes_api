use std::ops::Add;

pub fn time_parse_unix(unix_ts: i64) -> Result<time::PrimitiveDateTime, time::Error> {
    let dt = time::OffsetDateTime::from_unix_timestamp(unix_ts)?;
    Ok(time::PrimitiveDateTime::new(dt.date(), dt.time()))
}

pub fn time_now_add_week() -> time::PrimitiveDateTime {
    let now = time::OffsetDateTime::now_utc();

    time::PrimitiveDateTime::new(now.date(), now.time()).add(time::Duration::weeks(1))
}
