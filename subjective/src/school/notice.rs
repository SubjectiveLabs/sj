use diff::Diff;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "diff", derive(Diff))]
#[diff(attr(
    #[derive(Debug)]
    #[allow(missing_docs)]
))]
/// Notices related to a [`super::School`].
pub struct Notice {
    /// UUID of the notice.
    pub id: String,
    /// Title of the notice.
    pub title: String,
    /// Content of the notice.
    pub content: String,
    /// Whether the notice is a priority. If true, displayed more prominently.
    pub priority: bool,
}
