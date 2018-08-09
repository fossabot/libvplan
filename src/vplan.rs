use super::chrono::{DateTime, Utc};
use super::simple;

#[derive(Clone, Eq, PartialEq, Debug)]
/// A plan of timetable changes
pub struct Vplan {
    /// Vplan date
    pub date: VplanDate,
    /// Last time vplan was changed
    pub changed: DateTime<Utc>,
    /// Days off school
    pub days_off: Vec<DateTime<Utc>>,
    /// Changes to the timetable
    pub changes: Vec<Change>,
    /// Additional info
    pub info: Vec<String>
}

#[derive(Clone, Eq, PartialEq, Debug)]
/// A change to the timetable
pub struct Change {
    /// Class which got the change
    pub class: String,
    /// In which lesson
    pub lesson: String,
    /// (new) Subject
    pub subject: String,
    /// (new) Teacher
    pub teacher: String,
    /// (new) Room
    pub room: String,
    /// Additional info
    pub info: String
}

#[derive(Clone, Eq, PartialEq, Debug)]
/// A date type specific to vplan
pub struct VplanDate {
    pub date: DateTime<Utc>,
    /// Week type
    pub week_type: WeekType
}

#[derive(Clone, Eq, PartialEq, Debug)]
/// A type specific to vplan
pub enum WeekType {
    A,
    B
}

impl Vplan {
    /// Turns a vplan into its simple representation
    pub fn to_simple(&self) -> simple::Vplan {
        let week_type = match self.date.week_type {
            WeekType::A => simple::WeekType::A,
            WeekType::B => simple::WeekType::B
        };

        let days_off = self
            .days_off
            .iter()
            .map(|day| day.timestamp())
            .collect::<Vec<i64>>();

        let changes = self
            .changes
            .iter()
            .map(|change| simple::Change {
                class: change.class.clone(),
                lesson: change.lesson.clone(),
                subject: change.subject.clone(),
                teacher: change.teacher.clone(),
                room: change.room.clone(),
                info: change.info.clone()
            }).collect::<Vec<simple::Change>>();

        simple::Vplan {
            date: simple::VplanDate {
                date: self.date.date.timestamp(),
                week_type
            },
            changed: self.changed.timestamp(),
            days_off,
            changes,
            info: self.info.clone()
        }
    }
}
