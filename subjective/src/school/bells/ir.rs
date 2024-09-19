use serde::{de::Error, Deserialize, Deserializer, Serialize};
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
    #[serde(
        rename = "subjectID",
        deserialize_with = "deserialise_subject_id",
        default
    )]
    pub subject_id: Option<Uuid>,
    #[serde(default)]
    pub location: String,
    pub bell_type: Option<BellType>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn deserialise_subject_id<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
    D: Deserializer<'de>,
{
    let subject_id = Option::<String>::deserialize(deserializer)?;
    match subject_id {
        Some(id) if id.is_empty() => Ok(None),
        Some(id) => Uuid::parse_str(&id).map(Some).map_err(Error::custom),
        None => Ok(None),
    }
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
