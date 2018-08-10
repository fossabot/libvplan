use super::chrono::{DateTime, Utc};

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
