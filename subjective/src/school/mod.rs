use serde::{Deserialize, Serialize};

use self::{bells::ir::BellTime, link::Link, notice::Notice};

/// Bell-related data.
pub mod bells;
/// Link-related data.
pub mod link;
/// Notice-related data.
pub mod notice;

type Day = Vec<BellTime>;

#[derive(Serialize, Deserialize)]
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
    pub bell_times: Vec<Day>,
}
