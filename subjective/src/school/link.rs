use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Link to websites related to a [`super::School`].
#[derive(Serialize, Deserialize)]
pub struct Link {
    /// UUID of the link.
    pub id: Uuid,
    #[serde(rename = "title")]
    /// Name of the link.
    pub name: String,
    /// Icon associated with the link.
    pub icon: String,
    /// URL that the link points to.
    pub destination: String,
}
