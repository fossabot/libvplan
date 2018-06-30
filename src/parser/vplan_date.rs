use super::super::chrono::Weekday;
use super::super::vplan::{VplanDate, WeekType};
use super::common::{i32, to_date, to_month, to_week_type, to_weekday, u32};

named!(weekday<&str, Weekday>,
    map_res!(take_until!(", "), to_weekday)
);

named!(month<&str, u32>,
    map_res!(take_until!(" "), to_month)
);

named!(week<&str, WeekType>,
    map_res!(take_until!(" "), to_week_type)
);

named!(pub vplan_date<&str, VplanDate>,
    do_parse!(
        weekday      >>
        day: u32     >>
        tag!(". ")   >>
        month: month >>
        year: i32    >>
        tag!(" ")    >>
        week: week   >>
        (VplanDate{
            date: to_date(year, month, day, None, None, None),
            week_type: week
        })
    )
);
