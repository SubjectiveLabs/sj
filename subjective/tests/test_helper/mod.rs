#![cfg(feature = "std")]

use serde_json::from_str;
use subjective::Subjective;

pub fn load_data() -> Subjective {
    from_str(include_str!("../Timetable and Subjects.subjective")).unwrap()
}
