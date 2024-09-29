use diff::Diff;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Link to websites related to a [`super::School`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
#[cfg(feature = "diff")]
#[allow(clippy::module_name_repetitions)]
/// Differences between two [`Link`]s.
pub struct LinkDiff {
    /// Differences in the UUIDs of the links.
    pub id: Option<Uuid>,
    /// Differences in the names of the links.
    pub name: Option<String>,
    /// Differences in the icons of the links.
    pub icon: Option<String>,
    /// Differences in the destinations of the links.
    pub destination: Option<String>,
}

#[cfg(feature = "diff")]
impl Diff for Link {
    type Repr = LinkDiff;

    fn diff(&self, other: &Self) -> Self::Repr {
        Self::Repr {
            id: if self.id == other.id {
                None
            } else {
                Some(other.id)
            },
            name: self.name.diff(&other.name),
            icon: self.icon.diff(&other.icon),
            destination: self.destination.diff(&other.destination),
        }
    }

    fn apply(&mut self, diff: &Self::Repr) {
        if let Some(id) = diff.id {
            self.id = id;
        }
        self.name.apply(&diff.name);
        self.icon.apply(&diff.icon);
        self.destination.apply(&diff.destination);
    }

    fn identity() -> Self {
        Self {
            id: Uuid::nil(),
            name: String::new(),
            icon: String::new(),
            destination: String::new(),
        }
    }
}
