use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Notices related to a [`super::School`].
pub struct Notice {
    /// Title of the notice.
    pub title: String,
    /// Content of the notice.
    pub content: String,
    /// Whether the notice is a priority. If true, displayed more prominently.
    pub priority: bool,
}
