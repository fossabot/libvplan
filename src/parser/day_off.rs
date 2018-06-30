use super::super::chrono::{DateTime, Utc};
use super::common::{i32_padded, to_date, u32};

named!(pub day_off<&str, DateTime<Utc>>,
    do_parse!(
        year:  i32_padded >>
        month: u32        >>
        day:   u32        >>
        (to_date(year, month, day, None, None, None))
    )
);
