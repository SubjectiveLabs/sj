/// Bell-related data.
pub mod bells;
/// Link-related data.
pub mod link;
/// Notice-related data.
pub mod notice;

use crate::school::{bells::BellTime, link::Link, notice::Notice};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
    pub user_created: bool,
    /// Bell times for each day of the week.
    pub bell_times: [Day; 5],
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
