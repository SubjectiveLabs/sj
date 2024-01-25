use serde::{Deserialize, Serialize};
use uuid::Uuid;

const fn default_enabled() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BellTime {
    pub name: String,
    pub minute: u32,
    pub hour: u32,
    #[serde(rename = "subjectID")]
    pub subject_id: Option<Uuid>,
    #[serde(default)]
    pub location: String,
    pub bell_type: Option<BellType>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

impl PartialEq for BellTime {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.minute == other.minute
            && self.hour == other.hour
            && self.subject_id == other.subject_id
            && self.location == other.location
            && self.bell_type == other.bell_type
            && self.enabled == other.enabled
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct BellType {
    pub name: String,
    #[serde(rename = "iconName")]
    pub icon: String,
}

impl PartialEq for BellType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.icon == other.icon
    }
}
