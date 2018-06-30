mod changed;
mod common;
mod day_off;
mod vplan_date;

use self::changed::changed;
use self::day_off::day_off;
use self::vplan_date::vplan_date;

use super::chrono::{DateTime, Utc};
use super::document;
use super::error::ParsingError;
use super::serde_xml_rs;
use super::vplan;

/// Parses the given XML input into a vplan
///
/// # Example
/// ```rust,ignore
/// extern crate libvplan;
///
/// use libvplan::parser;
/// use std::fs::File;
/// use std::io::{BufReader, Read};
///
/// let file = match File::open("path/to/file.xml") {
///     Ok(file) => file,
///     Err(error) => panic!("{}", error)
/// };
///
/// let mut reader = BufReader::new(file);
/// let mut input = String::new();
/// reader.read_to_string(&mut input);
///
/// match parser::parse(&input) {
///     Ok(vplan) => println!("{:#?}", vplan),
///     Err(error) => panic!("{}", error)
/// }
/// ```
pub fn parse(input: &str) -> Result<vplan::Vplan, ParsingError> {
    match serde_xml_rs::from_str::<document::Vplan>(input) {
        Ok(document) => {
            let mut days_off: Vec<DateTime<Utc>> = Vec::new();

            for day in document.days_off.items {
                let day = self::day_off(day.value.as_ref());

                if let Err(error) = day {
                    return Err(ParsingError::DateParsingError(format!("{:#?}", error)));
                }

                let (_, day) = day.unwrap();

                days_off.push(day);
            }

            let mut changes: Vec<vplan::Change> = Vec::new();

            for change in document.main.items {
                changes.push(vplan::Change {
                    class: match change.class.value {
                        Some(value) => value,
                        None => "".to_owned()
                    },
                    lesson: match change.lesson.value {
                        Some(value) => value,
                        None => "".to_owned()
                    },
                    subject: match change.subject.value {
                        Some(value) => value,
                        None => "".to_owned()
                    },
                    teacher: match change.teacher.value {
                        Some(value) => value,
                        None => "".to_owned()
                    },
                    room: match change.room.value {
                        Some(value) => value,
                        None => "".to_owned()
                    },
                    info: match change.info.value {
                        Some(value) => value,
                        None => "".to_owned()
                    }
                });
            }

            let mut info: Vec<String> = Vec::new();

            for item in document.footer.items {
                match item.inner.value {
                    Some(value) => info.push(value),
                    None => info.push("".to_owned())
                }
            }

            if document.header.title.value.is_none() || document.header.date.value.is_none() {
                return Err(ParsingError::DateParsingError("empty date".to_owned()));
            }

            let date = document.header.title.value.unwrap();
            let date = self::vplan_date(date.as_ref());

            if let Err(error) = date {
                return Err(ParsingError::DateParsingError(format!("{:#?}", error)));
            }

            let (_, date) = date.unwrap();

            let changed = document.header.date.value.unwrap();
            let changed = self::changed(changed.as_ref());

            if let Err(error) = changed {
                return Err(ParsingError::DateParsingError(format!("{:#?}", error)));
            }

            let (_, changed) = changed.unwrap();

            Ok(vplan::Vplan {
                date,
                changed,
                days_off,
                changes,
                info
            })
        },
        Err(error) => Err(ParsingError::DocumentParsingError(error))
    }
}
