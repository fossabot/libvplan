use super::super::chrono::{DateTime, TimeZone, Utc, Weekday};
use super::super::chrono_tz::Europe::Berlin;
use super::super::error::ParsingError;
use super::super::vplan::WeekType;
use std::num::ParseIntError;

pub fn to_i32(input: &str) -> Result<i32, ParseIntError> {
    i32::from_str_radix(input, 10)
}

pub fn to_i32_padded(input: &str) -> Result<i32, ParseIntError> {
    i32::from_str_radix(format!("20{}", input).as_ref(), 10)
}

pub fn to_u32(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 10)
}

named!(pub i32<&str, i32>,
    map_res!(take!(2), to_i32)
);

named!(pub i32_padded<&str, i32>,
    map_res!(take!(2), to_i32_padded)
);

named!(pub u32<&str, u32>,
    map_res!(take!(2), to_u32)
);

pub fn to_date(
    year: i32,
    month: u32,
    day: u32,
    hour: Option<u32>,
    minute: Option<u32>,
    second: Option<u32>
) -> DateTime<Utc> {
    let hour = match hour {
        Some(hour) => hour,
        None => 0
    };

    let minute = match minute {
        Some(minute) => minute,
        None => 0
    };

    let second = match second {
        Some(second) => second,
        None => 0
    };

    let date = Berlin.ymd(year, month, day).and_hms(hour, minute, second);
    date.with_timezone(&Utc)
}

pub fn to_weekday(input: &str) -> Result<Weekday, ParsingError> {
    match input {
        "Montag" => Ok(Weekday::Mon),
        "Dienstag" => Ok(Weekday::Tue),
        "Mittwoch" => Ok(Weekday::Wed),
        "Donnerstag" => Ok(Weekday::Thu),
        "Freitag" => Ok(Weekday::Fri),
        _ => Err(ParsingError::DateParsingError("invalid weekday".to_owned()))
    }
}

pub fn to_month(input: &str) -> Result<u32, ParsingError> {
    match input {
        "Januar" => Ok(1),
        "Februar" => Ok(2),
        "März" => Ok(3),
        "April" => Ok(4),
        "Mai" => Ok(5),
        "Juni" => Ok(6),
        "Juli" => Ok(7),
        "August" => Ok(8),
        "September" => Ok(9),
        "Oktober" => Ok(10),
        "November" => Ok(11),
        "Dezember" => Ok(12),
        _ => Err(ParsingError::DateParsingError("invalid month".to_owned()))
    }
}

pub fn to_week_type(input: &str) -> Result<WeekType, ParsingError> {
    match input {
        "(A-Woche)" => Ok(WeekType::A),
        "(B-Woche)" => Ok(WeekType::B),
        _ => Err(ParsingError::DateParsingError(
            "invalid type of week".to_owned()
        ))
    }
}
