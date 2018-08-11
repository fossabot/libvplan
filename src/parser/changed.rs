use super::super::chrono::{DateTime, Utc};
use super::common::{i32, to_date, u32};

named!(pub changed<&str, DateTime<Utc>>,
    do_parse!(
        day: u32     >>
        take!(1)     >>
        month: u32   >>
        take!(1)     >>
        year: i32    >>
        take!(2)     >>
        hour: u32    >>
        take!(1)     >>
        minutes: u32 >>
        (to_date(year, month, day, Some(hour), Some(minutes), None))
    )
);
