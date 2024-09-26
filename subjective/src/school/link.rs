use diff::Diff;
use serde::{Deserialize, Serialize};

/// Link to websites related to a [`super::School`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "diff", derive(Diff))]
#[diff(attr(
    #[derive(Debug)]
    #[allow(missing_docs)]
))]
pub struct Link {
    /// UUID of the link.
    pub id: String,
    #[serde(rename = "title")]
    /// Name of the link.
    pub name: String,
    /// Icon associated with the link.
    pub icon: String,
    /// URL that the link points to.
    pub destination: String,
}
