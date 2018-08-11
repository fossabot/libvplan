use super::super::vplan::{VplanDate, WeekType};
use super::common::{i32, to_date, to_month, to_week_type, u32};

named!(month<&str, u32>,
    map_res!(take_until!(" "), to_month)
);

named!(week<&str, WeekType>,
    map_res!(take!(9), to_week_type)
);

named!(pub vplan_date<&str, VplanDate>,
    do_parse!(
        take_until!(",") >>
        take!(2)         >>
        day: u32         >>
        take!(2)         >>
        month: month     >>
        take!(1)         >>
        year: i32        >>
        take!(1)         >>
        week: week       >>
        (VplanDate{
            date: to_date(year, month, day, None, None, None),
            week_type: week
        })
    )
);
