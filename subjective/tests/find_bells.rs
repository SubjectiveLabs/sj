mod test_helper;

use chrono::{NaiveDate, NaiveTime};
use subjective::school::bells::{BellData, BellTime};
use uuid::Uuid;

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
            0,
        )
        .unwrap();
    assert_eq!(
        bell_time,
        &BellTime {
            name: "Period 2".to_string(),
            time: NaiveTime::from_hms_opt(9, 21, 0).unwrap(),
            bell_data: Some(BellData::Class {
                subject_id: Uuid::parse_str("e9dc7006-edd1-4674-bb62-48751868dfc6").unwrap(),
                location: "H1".to_string()
            }),
            enabled: true,
        }
    );
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
            0,
        )
        .unwrap();
    assert_eq!(
        bell_time,
        &BellTime {
            name: "Period 5".to_string(),
            time: NaiveTime::from_hms_opt(11, 51, 0).unwrap(),
            bell_data: Some(BellData::Class {
                subject_id: "4acf3b57-2b01-4e0e-ae7f-9ca210ddaf6e".parse().unwrap(),
                location: "G16".to_string()
            }),
            enabled: true,
        }
    );
}
