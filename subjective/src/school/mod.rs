use self::{bells::ir::BellTime, link::Link, notice::Notice};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Bell-related data.
pub mod bells;
/// Link-related data.
pub mod link;
/// Notice-related data.
pub mod notice;

type Day = Vec<BellTime>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// School data, including bells, notices, links, and bell times.
pub struct School {
    /// Name of the school.
    pub name: String,
    /// Notices associated with the school.
    pub notices: Vec<Notice>,
    #[serde(default)]
    /// Links associated with the school.
    pub links: Vec<Link>,
    /// Whether the user created the school.
    pub user_created: bool,
    /// Bell times for each day of the week.
    pub bell_times: Vec<Day>,
}

impl Display for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{: <40} ", self.name)?;
        write!(
            f,
            "{}",
            format!(
                "({} notices, {} links, {} bells)",
                self.notices.len(),
                self.links.len(),
                self.bell_times.iter().flatten().count()
            )
            .dimmed()
        )?;
        Ok(())
    }
}
