use std::{fs::File, io::Read};

use serde_json::from_str;
use subjective::Subjective;

#[test]
fn loads_correctly() {
    let mut file = File::open("tests/Darren's Timetable and Subjects.subjective").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let subjective: Subjective = from_str(&contents).unwrap();
    println!("{:#?}", subjective);
}
