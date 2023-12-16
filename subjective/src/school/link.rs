use uuid::Uuid;

pub struct Link {
    pub id: Uuid,
    pub title: String,
    pub icon: String,
    pub destination: String,
}
