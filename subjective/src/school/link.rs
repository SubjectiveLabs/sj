use serde::{Deserialize, Serialize};

/// Link to websites related to a [`super::School`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Link {
    #[serde(rename = "title")]
    /// Name of the link.
    pub name: String,
    /// Icon associated with the link.
    pub icon: String,
    /// URL that the link points to.
    pub destination: String,
}
