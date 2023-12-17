//! Subjective's Rust library.
//! Use this in your applications to interface with Subjective's data.
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, missing_docs)]
#![allow(clippy::multiple_crate_versions)]

use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use school::School;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use subjects::Subject;
/// Colors used for subjects.
pub mod color;
/// School related structures.
pub mod school;
/// Subject related structures.
pub mod subjects;

use thiserror::Error;

/// Errors that can occur when loading Subjective data.
#[derive(Error, Debug)]
pub enum SubjectiveError {
    /// The Subjective data file was not found.
    #[error("Couldn't find Subjective data file, which was expected at {0:?}.")]
    DataFileNotFound(PathBuf, io::Error),
    /// The Subjective data file could not be read.
    #[error("Failed to read Subjective data file.")]
    DataFileReadError(io::Error),
    /// The Subjective data file could not be parsed.
    #[error("Failed to parse Subjective data file. This may be due to invalid or outdated data. Try re-exporting your data again.")]
    DataFileParseError(serde_json::Error),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Structure of a Subjective data file.
pub struct Subjective {
    /// School data.
    pub school: School,
    /// Subject data.
    pub subjects: Vec<Subject>,
}

impl Subjective {
    /// Load Subjective data from a config directory.
    ///
    /// # Errors
    /// This function will return an error if the data file is not found, cannot be read, or cannot be parsed.
    pub fn from_config(config_directory: &Path) -> Result<Self, SubjectiveError> {
        let timetable_path = config_directory.join(".subjective");
        let mut timetable = File::open(timetable_path.clone())
            .map_err(|error| SubjectiveError::DataFileNotFound(timetable_path, error))?;
        let mut raw = String::new();
        timetable
            .read_to_string(&mut raw)
            .map_err(SubjectiveError::DataFileReadError)?;
        let data: Self = from_str(&raw).map_err(SubjectiveError::DataFileParseError)?;
        Ok(data)
    }

    #[must_use]
    /// Create a new Subjective data structure.
    pub fn new(school: School, subjects: Vec<Subject>) -> Self {
        Self { school, subjects }
    }

    #[must_use]
    /// Create a new Subjective data structure from a school and an empty subject list.
    pub fn from_school(school: School) -> Self {
        Self {
            school,
            subjects: Vec::new(),
        }
    }
}
