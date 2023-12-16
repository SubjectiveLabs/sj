use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
/// Notices related to a [`super::School`].
pub struct Notice {
    /// UUID of the notice.
    pub id: Uuid,
    /// Title of the notice.
    pub title: String,
    /// Content of the notice.
    pub content: String,
    /// Whether the notice is a priority. If true, displayed more prominently.
    pub priority: bool,
}
