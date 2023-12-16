use uuid::Uuid;

pub struct BellTime {
    pub id: Uuid,
    pub name: String,
    pub minute: u32,
    pub hour: u32,
    pub subject_id: Uuid,
    pub location: String,
    pub bell_type: BellType,
    pub enabled: bool,
}

pub struct BellType {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
}
