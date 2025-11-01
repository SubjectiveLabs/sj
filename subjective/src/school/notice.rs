#[cfg(feature = "diff")]
use diff::Diff;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
#[cfg(feature = "diff")]
#[allow(clippy::module_name_repetitions)]
/// Differences between two [`Notice`]s.
pub struct NoticeDiff {
    /// Differences in the UUIDs of the notices.
    pub id: Option<Uuid>,
    /// Differences in the titles of the notices.
    pub title: Option<String>,
    /// Differences in the content of the notices.
    pub content: Option<String>,
    /// Differences in the priorities of the notices.
    pub priority: Option<bool>,
}

#[cfg(feature = "diff")]
impl Diff for Notice {
    type Repr = NoticeDiff;

    fn diff(&self, other: &Self) -> Self::Repr {
        Self::Repr {
            id: if self.id == other.id {
                None
            } else {
                Some(other.id)
            },
            title: self.title.diff(&other.title),
            content: self.content.diff(&other.content),
            priority: self.priority.diff(&other.priority),
        }
    }

    fn apply(&mut self, diff: &Self::Repr) {
        if let Some(id) = diff.id {
            self.id = id;
        }
        self.title.apply(&diff.title);
        self.content.apply(&diff.content);
        self.priority.apply(&diff.priority);
    }

    fn identity() -> Self {
        Self {
            id: Uuid::nil(),
            title: String::new(),
            content: String::new(),
            priority: false,
        }
    }
}
