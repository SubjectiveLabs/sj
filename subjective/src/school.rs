/// Bell-related data.
pub mod bells;
/// Link-related data.
pub mod link;
/// Notice-related data.
pub mod notice;

use crate::school::{bells::BellTime, link::Link, notice::Notice};
use colored::Colorize;
#[cfg(feature = "diff")]
use diff::{Diff, VecDiff};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Error};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Hash)]
/// A week variant of a Subjective timetable.
pub struct Week {
    /// UUID of the week variant.
    pub id: Uuid,
    /// Name of the week variant.
    pub name: String,
    /// Days of the week.
    pub days: Vec<Day>,
    /// Whether the week variant is included in the automatic cycle.
    pub cyclical: bool,
}

#[cfg(feature = "diff")]
#[derive(Debug)]
/// Differences between two [`Week`]s.
pub struct WeekDiff {
    /// Differences in the UUID of the week.
    pub id: Option<Uuid>,
    /// Differences in the name of the week.
    pub name: Option<String>,
    /// Differences in the days of the week.
    pub days: VecDiff<Day>,
    /// Differences in the cyclical status of the week.
    pub cyclical: Option<bool>,
}

#[cfg(feature = "diff")]
impl Diff for Week {
    type Repr = WeekDiff;

    fn diff(&self, other: &Self) -> Self::Repr {
        Self::Repr {
            id: if self.id == other.id {
                None
            } else {
                Some(other.id)
            },
            name: self.name.diff(&other.name),
            days: self.days.diff(&other.days),
            cyclical: self.cyclical.diff(&other.cyclical),
        }
    }

    fn apply(&mut self, diff: &Self::Repr) {
        if let Some(id) = diff.id {
            self.id = id;
        }
        self.name.apply(&diff.name);
        self.days.apply(&diff.days);
        self.cyclical.apply(&diff.cyclical);
    }

    fn identity() -> Self {
        Self {
            id: Uuid::nil(),
            name: String::new(),
            days: Vec::new(),
            cyclical: false,
        }
    }
}

/// A day of the week, containing bell times for each period.
pub type Day = Vec<BellTime>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "diff", derive(Diff))]
#[cfg_attr(feature = "diff", diff(attr(
    #[derive(Debug)]
    #[allow(missing_docs)]
)))]
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
    /// Latitude of the school in degrees.
    pub latitude: f64,
    /// Longitude of the school in degrees.
    pub longitude: f64,
    /// Location of the school, normally a suburb and state.
    pub location: String,
    /// Tags associated with the school; nicknames, abbreviations, etc.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Version of the school data.
    pub version: String,
}

impl Eq for School {}

impl FromStr for School {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str(s)
    }
}

impl Display for School {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{: <40} ", self.name)?;
        write!(
            f,
            "{}",
            format!(
                "({} links, {} week{}, {} bells, in {})",
                self.links.len(),
                self.bell_times.len(),
                if self.bell_times.len() == 1 { "" } else { "s" },
                self.bell_times
                    .iter()
                    .map(|Week { days, .. }| days.iter().flatten().count())
                    .sum::<usize>(),
                self.location
            )
            .dimmed()
        )?;
        Ok(())
    }
}
