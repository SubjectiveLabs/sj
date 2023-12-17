//! Subjective's Rust library. Use this in your applications to interface with Subjective's data.
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, missing_docs)]

use school::School;
use serde::{Deserialize, Serialize};
use subjects::Subject;
/// School related structures.
pub mod school;
/// Subject related structures.
pub mod subjects;
/// Colors used for subjects.
pub mod color;


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Structure of a Subjective data file.
pub struct Subjective {
    /// School data.
    pub school: School,
    /// Subject data.
    pub subjects: Vec<Subject>,
}
