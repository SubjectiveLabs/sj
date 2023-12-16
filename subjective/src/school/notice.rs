use uuid::Uuid;

pub struct Notice {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub priority: bool,
}
