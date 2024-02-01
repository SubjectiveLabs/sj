mod test_helper;

use chrono::{NaiveDate, NaiveTime};
use subjective::school::bells::{BellTime, BellData};

use crate::test_helper::load_data;

#[test]
fn find_first_after_works() {
    let subjective = load_data();
    let bell_time = subjective
        .find_first_after(
            NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        bell_time,
        &BellTime {
            name: "Recess".to_string(),
            time: NaiveTime::from_hms_opt(10, 1, 0).unwrap(),
            bell_data: Some(BellData::Break),
            enabled: true,
        }
    )
}

#[test]
fn find_first_before_works() {
    let subjective = load_data();
    let bell_time = subjective
        .find_first_before(
            NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        bell_time,
        &BellTime {
            name: "Period 5".to_string(),
            time: NaiveTime::from_hms_opt(11, 51, 0).unwrap(),
            bell_data: Some(BellData::Class {
                subject_id: "5db89875-0167-42ce-97f0-0a6b79fbc2b8".parse().unwrap(),
                location: "H1".to_string()
            }),
            enabled: true,
        }
    )
}
