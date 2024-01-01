use serde_json::from_str;
use subjective::Subjective;

pub fn load_data() -> Subjective {
    from_str(include_str!(
        "../Darren's Timetable and Subjects.subjective"
    ))
    .unwrap()
}
