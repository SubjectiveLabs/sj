mod test_helper;

use chrono::{NaiveDate, NaiveTime};
use subjective::school::bells::{BellData, BellTime};
use uuid::{uuid, Uuid};

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
            id: Uuid::new_v4(),
            name: "Period 2".to_string(),
            time: NaiveTime::from_hms_opt(9, 21, 0).unwrap(),
            bell_data: Some(BellData::Class {
                subject_id: uuid!("40e0f233-d1e3-4402-b5c3-3094122126e6"),
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
            id: Uuid::new_v4(),
            name: "Period 5".to_string(),
            time: NaiveTime::from_hms_opt(11, 51, 0).unwrap(),
            bell_data: Some(BellData::Class {
                subject_id: uuid!("7b1efb1b-cbf4-4e0a-82d9-770ef588e329"),
                location: "G16".to_string()
            }),
            enabled: true,
        }
    );
}
