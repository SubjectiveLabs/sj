/// Bell-related data.
pub mod bells;
/// Link-related data.
pub mod link;
/// Notice-related data.
pub mod notice;

use crate::school::bells::BellTime;
#[cfg(feature = "std")]
use crate::school::{link::Link, notice::Notice};
use cfg_if::cfg_if;
#[cfg(feature = "std")]
use colored::Colorize;
#[cfg(feature = "std")]
use core::fmt::{self, Display, Formatter};
#[cfg(all(feature = "diff", feature = "std"))]
use diff::{Diff, VecDiff};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "std")] {
        #[derive(Debug, Clone, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
        /// A week variant of a Subjective timetable.
        pub struct Week {
            #[serde(default = "Uuid::new_v4")]
            /// UUID of the week variant.
            pub id: Uuid,
            /// Name of the week variant.
            pub name: String,
            /// Days of the week.
            pub days: Vec<Day>,
            /// Whether the week variant is included in the automatic cycle.
            pub cyclical: bool,
        }
    } else {
        #[derive(Debug, Clone, Default, Hash)]
        /// A week variant of a Subjective timetable.
        pub struct Week<'a, 'b> {
            /// Days of the week.
            pub days: &'a [Day<'b>],
        }
    }
}

#[cfg(all(feature = "diff", feature = "std"))]
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

#[cfg(all(feature = "diff", feature = "std"))]
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

cfg_if! {
    if #[cfg(feature = "std")] {
        /// A day of the week, containing bell times for each period.
        pub type Day = Vec<BellTime>;
    }
    else {
        /// A day of the week, containing bell times for each period.
        pub type Day<'a> = &'a [BellTime];
    }
}

cfg_if! {
    if #[cfg(feature = "std")] {
        #[derive(Debug, Clone, Serialize, Deserialize)]
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
    } else {
        #[derive(Debug, Clone)]
        /// School data, including bells, notices, links, and bell times.
        pub struct School<'a, 'b, 'c> {
            /// Bell times for each week variant.
            pub bell_times: &'a [Week<'b, 'c>],
        }
    }
}

#[cfg(feature = "std")]
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
