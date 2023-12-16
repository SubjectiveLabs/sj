use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct BellTime {
    pub id: Uuid,
    pub name: String,
    pub minute: u32,
    pub hour: u32,
    pub subject_id: Uuid,
    pub location: String,
    pub bell_type: Option<BellType>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BellType {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
}

#[cfg(test)]
mod test {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn test_serialize() {
        let bell_time = BellTime {
            id: Uuid::nil(),
            name: "Test".to_string(),
            minute: 0,
            hour: 0,
            subject_id: Uuid::nil(),
            location: "Test".to_string(),
            bell_type: None,
            enabled: true,
        };
        let serialized = to_string(&bell_time).unwrap();
        assert_eq!(
            serialized,
            "{\"id\":\"00000000-0000-0000-0000-000000000000\",\"name\":\"Test\",\"minute\":0,\"hour\":0,\"subject_id\":\"00000000-0000-0000-0000-000000000000\",\"location\":\"Test\",\"bell_type\":null,\"enabled\":true}"
        );
    }

    #[test]
    fn test_deserialize() {
        let serialized = "{\"id\":\"00000000-0000-0000-0000-000000000000\",\"name\":\"Test\",\"minute\":0,\"hour\":0,\"subject_id\":\"00000000-0000-0000-0000-000000000000\",\"location\":\"Test\",\"enabled\":true}";
        let bell_time: BellTime = from_str(serialized).unwrap();
        assert_eq!(bell_time.id, Uuid::nil());
        assert_eq!(bell_time.name, "Test");
        assert_eq!(bell_time.minute, 0);
        assert_eq!(bell_time.hour, 0);
        assert_eq!(bell_time.subject_id, Uuid::nil());
        assert_eq!(bell_time.location, "Test");
        assert!(bell_time.enabled);
    }
}
