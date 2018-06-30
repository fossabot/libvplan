use super::super::chrono::{DateTime, Utc};
use super::common::{i32_padded, to_date, u32};

named!(pub changed<&str, DateTime<Utc>>,
    do_parse!(
        day: u32 >>
        tag!(".") >>
        month: u32 >>
        tag!(".") >>
        year: i32_padded >>
        tag!(", ") >>
        hour: u32 >>
        tag!(":") >>
        minutes: u32 >>
        (to_date(year, month, day, Some(hour), Some(minutes), None))
    )
);
