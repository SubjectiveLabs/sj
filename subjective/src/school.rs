/// Bell-related data.
pub mod bells;
/// Link-related data.
pub mod link;
/// Notice-related data.
pub mod notice;

use crate::school::{bells::BellTime, link::Link, notice::Notice};
use colored::Colorize;
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display, Formatter};

/// A day of the week, containing bell times for each period.
pub type Day = Vec<BellTime>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// School data, including bells, notices, links, and bell times.
pub struct School {
    /// Name of the school.
    pub name: String,
    /// Notices associated with the school.
    pub notices: Vec<Notice>,
    /// Links associated with the school.
    pub links: Vec<Link>,
    /// Whether the user created the school.
    #[serde(default)]
    pub user_created: bool,
    /// Bell times for each week variant.
    #[serde(deserialize_with = "from_map", serialize_with = "to_map")]
    pub bell_times: Vec<(String, [Day; 5])>,
}

fn from_map<'de, D>(deserializer: D) -> Result<Vec<(String, [Day; 5])>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: LinkedHashMap<String, Vec<Day>> = Deserialize::deserialize(deserializer)?;
    Ok(s.into_iter()
        .map(|(name, week)| (name, week.try_into().unwrap()))
        .collect())
}

fn to_map<S>(value: &[(String, [Day; 5])], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let map: LinkedHashMap<String, Vec<Day>> = value
        .iter()
        .map(|(name, week)| (name.clone(), week.to_vec()))
        .collect();
    map.serialize(serializer)
}

impl Display for School {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{: <40} ", self.name)?;
        write!(
            f,
            "{}",
            format!(
                "({} notices, {} links, {} week{}, {} bells)",
                self.notices.len(),
                self.links.len(),
                self.bell_times.len(),
                if self.bell_times.len() == 1 { "" } else { "s" },
                self.bell_times
                    .iter()
                    .map(|(_, days)| days.iter().flatten().count())
                    .sum::<usize>()
            )
            .dimmed()
        )?;
        Ok(())
    }
}
