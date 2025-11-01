#![cfg(feature = "std")]

use serde::{Deserialize, Deserializer, Serialize, de::Error};
use uuid::Uuid;

const fn default_enabled() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BellTime {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BellType {
    pub name: String,
    #[serde(rename = "iconName")]
    pub icon: String,
}
