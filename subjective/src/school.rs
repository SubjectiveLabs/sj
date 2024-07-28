/// Bell-related data.
pub mod bells;
/// Link-related data.
pub mod link;
/// Notice-related data.
pub mod notice;

use crate::school::{bells::BellTime, link::Link, notice::Notice};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// A week variant of a Subjective timetable.
pub struct Week {
    /// Name of the week variant.
    pub name: String,
    /// Days of the week.
    pub days: Vec<Day>,
    /// Whether the week variant is included in the automatic cycle.
    pub cyclical: bool,
}

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
    pub bell_times: Vec<Week>,
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
                    .map(|Week { days, .. }| days.iter().flatten().count())
                    .sum::<usize>()
            )
            .dimmed()
        )?;
        Ok(())
    }
}
