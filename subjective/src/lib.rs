//! Subjective's Rust library.
//! Use this in your applications to interface with Subjective's data.
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, missing_docs)]
#![allow(clippy::multiple_crate_versions)]

use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use school::{bells::BellTime, Day, School};
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
use uuid::Uuid;

/// Errors that can occur when loading Subjective data.
#[derive(Error, Debug)]
pub enum LoadDataError {
    /// The Subjective data file was not found.
    #[error("Couldn't find Subjective data file, which was expected at {0:?}.")]
    DataFileNotFound(PathBuf, io::Error),
    /// The Subjective data file could not be read.
    #[error("Failed to read Subjective data file.")]
    DataFileReadError(io::Error),
    /// The Subjective data file could not be parsed.
    #[error("Failed to parse Subjective data file. This may be due to invalid or outdated data. Try re-exporting your data again.\n{0}")]
    DataFileParseError(serde_json::Error),
}

/// Errors that can occur when retrieving bells.
#[derive(Error, Debug)]
pub enum FindBellError {
    /// The specified weekday is out of range.
    #[error("The ISO 8601 weekday number {0} is out of the range `1..=5`.")]
    WeekdayOutOfRange(usize),
    /// No bell was found.
    #[error("No bell was found.")]
    NoBellFound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
    ///
    /// This function will return an error if the data file is not found, cannot be read, or cannot
    /// be parsed.
    pub fn from_config(config_directory: &Path) -> Result<Self, LoadDataError> {
        let timetable_path = config_directory.join(".subjective");
        let mut timetable = File::open(timetable_path.clone())
            .map_err(|error| LoadDataError::DataFileNotFound(timetable_path, error))?;
        let mut raw = String::new();
        timetable
            .read_to_string(&mut raw)
            .map_err(LoadDataError::DataFileReadError)?;
        let data = from_str(&raw).map_err(LoadDataError::DataFileParseError)?;
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

    /// Find all bells after a given time, on a specified weekday.
    /// Searches are not continued over days, so if the time is after the last bell on the specified
    /// day, it does not search the next day.
    /// The bells are returned in ascending order.
    /// Bells must be sorted in ascending order for this function to work correctly.
    ///
    /// # Errors
    ///
    /// This function will return an error if the weekday is out of range
    /// ([`FindBellError::WeekdayOutOfRange`]).
    /// If no bells are found, because there are no bell times after the given time for the
    /// specified day, it returns ([`FindBellError::NoBellFound`]).
    pub fn find_all_after(
        &self,
        date_time: NaiveDateTime,
        variant_offset: usize,
    ) -> Result<&[BellTime], FindBellError> {
        let day = self.get_day(date_time.date(), variant_offset)?;
        let time = date_time.time();
        let bells = day
            .iter()
            .position(|bell| bell.time >= time && bell.enabled)
            .ok_or(FindBellError::NoBellFound)?;
        let bells = &day[bells..];
        if bells.is_empty() {
            return Err(FindBellError::NoBellFound);
        }
        Ok(bells)
    }

    /// Find all bells before a given time, on a specified weekday.
    /// Searches are not continued over days, so if the time is before the first bell on the
    /// specified day, it does not search the previous day.
    /// The bells are returned in descending order.
    /// Bells must be sorted in ascending order for this function to work correctly.
    ///
    /// # Errors
    ///
    /// This function will return an error if the weekday is out of range
    /// ([`FindBellError::WeekdayOutOfRange`]).
    /// If no bells are found, because there are no bell times before the given time for the
    /// specified day, it returns ([`FindBellError::NoBellFound`]).
    pub fn find_all_before(
        &self,
        date_time: NaiveDateTime,
        variant_offset: usize,
    ) -> Result<&[BellTime], FindBellError> {
        let day = self.get_day(date_time.date(), variant_offset)?;
        let time = date_time.time();
        let bells = day
            .iter()
            .rposition(|bell| bell.time <= time && bell.enabled)
            .ok_or(FindBellError::NoBellFound)?;
        let bells = &day[..=bells];
        if bells.is_empty() {
            return Err(FindBellError::NoBellFound);
        }
        Ok(bells)
    }

    /// Find the first bell after a given time, on a specified weekday.
    /// Searches are not continued over days, so if the time is after the last bell on the specified
    /// day, it does not search the next day.
    ///
    /// # Errors
    ///
    /// This function will return an error if the weekday is out of range
    /// ([`FindBellError::WeekdayOutOfRange`]).
    /// If no bell is found, because there are no bell times after the given time for the specified
    /// day, it returns ([`FindBellError::NoBellFound`]).
    pub fn find_first_after(
        &self,
        date_time: NaiveDateTime,
        variant_offset: usize,
    ) -> Result<&BellTime, FindBellError> {
        let day = self.get_day(date_time.date(), variant_offset)?;
        let time = date_time.time();
        day.iter()
            .find(|bell| bell.time >= time && bell.enabled)
            .ok_or(FindBellError::NoBellFound)
    }

    /// Find the first bell before a given time, on a specified weekday.
    /// Searches are not continued over days, so if the time is before the first bell on the
    /// specified day, it does not search the previous day.
    ///
    /// # Errors
    ///
    /// This function will return an error if the weekday is out of range
    /// ([`FindBellError::WeekdayOutOfRange`]).
    /// If no bell is found, because there are no bell times before the given time for the specified
    /// day, it returns ([`FindBellError::NoBellFound`]).
    pub fn find_first_before(
        &self,
        date_time: NaiveDateTime,
        variant_offset: usize,
    ) -> Result<&BellTime, FindBellError> {
        let day = self.get_day(date_time.date(), variant_offset)?;
        let time = date_time.time();
        day.iter()
            .rev()
            .find(|bell| bell.time <= time && bell.enabled)
            .ok_or(FindBellError::NoBellFound)
    }

    /// Get the day for a given date, calculating the current variant using
    ///
    /// `current_variant = (week_number + variant_offset) % weeks`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the weekday is out of range
    /// ([`FindBellError::WeekdayOutOfRange`]).
    #[allow(clippy::cast_sign_loss)]
    pub fn get_day(&self, date: NaiveDate, variant_offset: usize) -> Result<&Day, FindBellError> {
        let weekday = date.weekday().num_days_from_monday() as usize;
        let current_variant =
            get_current_variant(date, variant_offset, self.school.bell_times.len());
        let bell_times = &self.school.bell_times[current_variant].days;
        let day = bell_times
            .get(weekday)
            .ok_or(FindBellError::WeekdayOutOfRange(weekday))?;
        Ok(day)
    }

    #[must_use]
    /// Get the subject with the given ID.
    ///
    /// # Errors
    ///
    /// This function will return [`None`] if no subject with the given ID is found.
    pub fn get_subject(&self, subject_id: Uuid) -> Option<&Subject> {
        self.subjects
            .iter()
            .find(|subject| subject.id == subject_id)
    }
}

/// Get the current variant for a given date, variant offset, and number of variants.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
#[must_use]
pub fn get_current_variant(date: NaiveDate, variant_offset: usize, variants: usize) -> usize {
    let weeks = variants;
    let week_number = date.iso_week().week() as usize;
    (week_number + variant_offset) % weeks
}
