use std::{
    cmp::Ordering,
    fmt::{self, Write},
};

use chrono::{NaiveTime, TimeDelta, Timelike};
use colored::Colorize;
use diff::{Diff, OptionDiff};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use strum_macros::Display;
use thiserror::Error;
use uuid::Uuid;

use crate::{color::Color, subjects::Subject, Subjective};

pub(crate) mod ir;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Bell-related data.
pub struct BellTime {
    /// UUID of the bell.
    pub id: Uuid,
    /// Name of the bell.
    pub name: String,
    /// Time of the bell.
    pub time: NaiveTime,
    /// Data related to the bell.
    pub bell_data: Option<BellData>,
    /// Whether the bell is enabled. Notifications will not be sent for disabled bells.
    pub enabled: bool,
}

#[cfg(feature = "diff")]
#[derive(Debug)]
/// Differences between two [`BellTime`] instances.
pub struct BellTimeDiff {
    /// Differences in the UUID of the bell.
    pub id: Option<Uuid>,
    /// Differences in the name of the bell.
    pub name: Option<String>,
    /// Differences in the time of the bell.
    pub time: TimeDelta,
    /// Differences in the data related to the bell.
    pub bell_data: OptionDiff<BellData>,
    /// Differences in the enabled status of the bell.
    pub enabled: Option<bool>,
}

#[cfg(feature = "diff")]
impl Diff for BellTime {
    type Repr = BellTimeDiff;

    fn diff(&self, other: &Self) -> Self::Repr {
        Self::Repr {
            id: if self.id == other.id {
                None
            } else {
                Some(other.id)
            },
            name: self.name.diff(&other.name),
            time: other.time - self.time,
            bell_data: self.bell_data.diff(&other.bell_data),
            enabled: self.enabled.diff(&other.enabled),
        }
    }

    fn apply(&mut self, diff: &Self::Repr) {
        if let Some(id) = diff.id {
            self.id = id;
        }
        self.name.apply(&diff.name);
        self.time += diff.time;
        self.bell_data.apply(&diff.bell_data);
        self.enabled.apply(&diff.enabled);
    }

    fn identity() -> Self {
        Self {
            id: Uuid::nil(),
            name: String::new(),
            time: NaiveTime::default(),
            bell_data: None,
            enabled: false,
        }
    }
}

/// Errors that can occur when formatting a [`BellTime`] with [`BellTime::format`].
#[derive(Error, Debug)]
pub enum FormatBellError {
    /// The subject with the given ID was not found. This means that the data is invalid.
    #[error("No subject found matching \"{0}\". This means that your Subjective data is invalid.")]
    SubjectNotFound(Uuid),
    /// An error occurred while formatting the bell time.
    #[error(transparent)]
    FmtError(#[from] fmt::Error),
}

impl BellTime {
    pub(crate) fn from_ir(bell_time: &ir::BellTime) -> Option<Self> {
        let time = NaiveTime::from_hms_opt(bell_time.hour, bell_time.minute, 0)?;
        let bell_data = BellData::from_ir(bell_time);
        Some(Self {
            id: bell_time.id,
            name: bell_time.name.clone(),
            time,
            bell_data,
            enabled: bell_time.enabled,
        })
    }

    pub(crate) fn to_ir(&self) -> ir::BellTime {
        ir::BellTime {
            id: self.id,
            name: self.name.clone(),
            hour: self.time.hour(),
            minute: self.time.minute(),
            bell_type: self.bell_data.as_ref().and_then(BellData::to_ir),
            subject_id: self
                .bell_data
                .as_ref()
                .and_then(|bell_data| match bell_data {
                    BellData::Class { subject_id, .. } => Some(*subject_id),
                    _ => None,
                }),
            location: match self.bell_data.as_ref() {
                Some(BellData::Class { location, .. }) => location.clone(),
                _ => String::new(),
            },
            enabled: self.enabled,
        }
    }

    fn inner_format(&self, data: &Subjective, show_time: bool) -> Result<String, FormatBellError> {
        let mut output = String::new();
        let bell_name = Color::SUBJECTIVE_BLUE.color(&*self.name);
        match &self.bell_data {
            Some(BellData::Class {
                subject_id,
                location,
            }) => {
                let Subject {
                    name: subject_name,
                    color,
                    ..
                } = data
                    .get_subject(*subject_id)
                    .ok_or(FormatBellError::SubjectNotFound(*subject_id))?;
                let subject_name = color.color(&**subject_name);
                let location = color.color(&**location);
                write!(output, "{subject_name} in {location} {bell_name}")?;
            }
            Some(bell_data) => {
                let bell_data = format!("{bell_data}").dimmed();
                write!(output, "{bell_data} {bell_name}")?;
            }
            None => {
                write!(output, "{bell_name}")?;
            }
        }
        if show_time {
            let time = self.time.format("%-I:%M %p").to_string().dimmed();
            write!(output, " {time}")?;
        }
        Ok(output)
    }

    /// Format the bell time as a string, in the context of the given [`Subjective`] data.
    /// The data is used to get the name of the subject that the bell rings for.
    ///
    /// # Errors
    ///
    /// Returns an error if the subject with the given ID is not found, or if `writeln!` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use subjective::{school::{bells::{BellTime, BellData}, School, Week}, subjects::Subject, Subjective};
    /// # use uuid::Uuid;
    /// # use chrono::NaiveTime;
    /// # use std::default::Default;
    /// # let data = Subjective {
    /// #     subjects: vec![
    /// #         Subject {
    /// #             name: "Maths".to_string(),
    /// #             color: subjective::color::Color {
    /// #                 red: 0.0,
    /// #                 green: 0.0,
    /// #                 blue: 0.0,
    /// #             },
    /// #             icon: "".to_string(),
    /// #             id: Uuid::nil(),
    /// #             locations: vec!["D14".to_string()],
    /// #         }
    /// #     ],
    /// #     school: School {
    /// #         name: "School".to_string(),
    /// #         bell_times: vec![
    /// #             Week {
    /// #                 name: "Week 1".to_string(),
    /// #                 days: vec![
    /// #                     vec![
    /// #                         BellTime {
    /// #                             name: "Period 1".to_string(),
    /// #                             time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
    /// #                             bell_data: Some(BellData::Class {
    /// #                                 subject_id: Uuid::nil(),
    /// #                                 location: "D14".to_string(),
    /// #                             }),
    /// #                             enabled: true,
    /// #                         }
    /// #                     ],
    /// #                     Vec::new(),
    /// #                     Vec::new(),
    /// #                     Vec::new(),
    /// #                     Vec::new(),
    /// #                 ],
    /// #                 cyclical: true
    /// #             }
    /// #         ],
    /// #         notices: Default::default(),
    /// #         links: Default::default(),
    /// #         user_created: Default::default(),
    /// #         latitude: Default::default(),
    /// #         longitude: Default::default(),
    /// #         location: Default::default(),
    /// #         tags: Default::default(),
    /// #         version: Default::default(),
    /// #     },
    /// # };
    /// let bell_time = BellTime {
    ///     name: "Period 1".to_string(),
    ///     time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
    ///     bell_data: Some(BellData::Class {
    ///         subject_id: Uuid::nil(),
    ///         location: "D14".to_string(),
    ///     }),
    ///     enabled: true,
    /// };
    ///
    /// assert_eq!(bell_time.format(&data).unwrap(), "Maths in D14 Period 1".to_string());
    /// ```
    pub fn format(&self, data: &Subjective) -> Result<String, FormatBellError> {
        self.inner_format(data, false)
    }

    /// Format the bell time as a string, in the context of the given [`Subjective`] data, then concatenate the time at the end.
    /// The data is used to get the name of the subject that the bell rings for.
    ///
    /// # Errors
    ///
    /// Returns an error if the subject with the given ID is not found, or if `writeln!` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use subjective::{school::{bells::{BellTime, BellData}, School, Week}, subjects::Subject, Subjective};
    /// # use uuid::Uuid;
    /// # use chrono::NaiveTime;
    /// # use std::default::Default;
    /// # let data = Subjective {
    /// #     subjects: vec![
    /// #         Subject {
    /// #             name: "Maths".to_string(),
    /// #             color: subjective::color::Color {
    /// #                 red: 0.0,
    /// #                 green: 0.0,
    /// #                 blue: 0.0,
    /// #             },
    /// #             icon: "".to_string(),
    /// #             id: Uuid::nil(),
    /// #             locations: vec!["D14".to_string()],
    /// #         }
    /// #     ],
    /// #     school: School {
    /// #         name: "School".to_string(),
    /// #         bell_times: vec![
    /// #             Week {
    /// #                 name: "Week 1".to_string(),
    /// #                 days: vec![
    /// #                     vec![
    /// #                         BellTime {
    /// #                             name: "Period 1".to_string(),
    /// #                             time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
    /// #                             bell_data: Some(BellData::Class {
    /// #                                 subject_id: Uuid::nil(),
    /// #                                 location: "D14".to_string(),
    /// #                             }),
    /// #                             enabled: true,
    /// #                         }
    /// #                     ],
    /// #                     Vec::new(),
    /// #                     Vec::new(),
    /// #                     Vec::new(),
    /// #                     Vec::new(),
    /// #                 ],
    /// #                 cyclical: true
    /// #             }
    /// #         ],
    /// #         notices: Default::default(),
    /// #         links: Default::default(),
    /// #         user_created: Default::default(),
    /// #         latitude: Default::default(),
    /// #         longitude: Default::default(),
    /// #         location: Default::default(),
    /// #         tags: Default::default(),
    /// #         version: Default::default(),
    /// #     },
    /// # };
    /// let bell_time = BellTime {
    ///     name: "Period 1".to_string(),
    ///     time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
    ///     bell_data: Some(BellData::Class {
    ///         subject_id: Uuid::nil(),
    ///         location: "D14".to_string(),
    ///     }),
    ///     enabled: true,
    /// };
    ///
    /// assert_eq!(bell_time.format_with_time(&data).unwrap(), "Maths in D14 Period 1 9:00 AM".to_string());
    /// ```
    pub fn format_with_time(&self, data: &Subjective) -> Result<String, FormatBellError> {
        self.inner_format(data, true)
    }
}

impl<'de> Deserialize<'de> for BellTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bell_time = ir::BellTime::deserialize(deserializer)?;
        Self::from_ir(&bell_time).ok_or_else(|| {
            D::Error::custom(format!(
                "invalid hour {} or minute {}",
                bell_time.hour, bell_time.minute
            ))
        })
    }
}

impl Serialize for BellTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_ir().serialize(serializer)
    }
}

impl Ord for BellTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for BellTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
/// Data associated with a [`BellTime`].
pub enum BellData {
    /// Class which is related to a subject.
    Class {
        /// UUID of the subject that the bell rings for.
        subject_id: Uuid,
        /// Location of the bell. This can be a related classroom.
        location: String,
    },
    /// Important time, such as the start and end of the school day, and assemblies.
    Time,
    /// A recess, lunch, or another break between classes.
    Break,
    /// Study period.
    Study,
    /// Miscellaneous break.
    Pause,
}

#[cfg(feature = "diff")]
#[derive(Debug)]
/// Differences between two [`BellData`] instances.
pub enum BellDataDiff {
    /// The [`BellData`] differed in the class data.
    Class {
        /// Differences in the subject ID.
        subject_id: Option<Uuid>,
        /// Differences in the location.
        location: Option<String>,
    },
    /// The [`BellData`] changed to [`BellData::Time`].
    Time,
    /// The [`BellData`] changed to [`BellData::Break`].
    Break,
    /// The [`BellData`] changed to [`BellData::Study`].
    Study,
    /// The [`BellData`] changed to [`BellData::Pause`].
    Pause,
}

impl Diff for BellData {
    type Repr = Option<BellDataDiff>;

    fn diff(&self, other: &Self) -> Self::Repr {
        match (self, other) {
            (
                Self::Class {
                    subject_id: subject_id_a,
                    location: location_a,
                },
                Self::Class {
                    subject_id: subject_id_b,
                    location: location_b,
                },
            ) => Some(BellDataDiff::Class {
                subject_id: if subject_id_a == subject_id_b {
                    None
                } else {
                    Some(*subject_id_b)
                },
                location: location_a.diff(location_b),
            }),
            (Self::Time, Self::Time)
            | (Self::Break, Self::Break)
            | (Self::Study, Self::Study)
            | (Self::Pause, Self::Pause) => None,
            _ => Some(match other {
                Self::Class {
                    subject_id,
                    location,
                } => BellDataDiff::Class {
                    subject_id: Some(*subject_id),
                    location: Some(location.clone()),
                },
                Self::Time => BellDataDiff::Time,
                Self::Break => BellDataDiff::Break,
                Self::Study => BellDataDiff::Study,
                Self::Pause => BellDataDiff::Pause,
            }),
        }
    }

    fn apply(&mut self, diff: &Self::Repr) {
        match diff {
            Some(BellDataDiff::Class {
                subject_id: subject_id_diff,
                location: location_diff,
            }) => {
                if let Self::Class {
                    subject_id,
                    location,
                } = self
                {
                    if let Some(subject_id_diff) = subject_id_diff {
                        *subject_id = *subject_id_diff;
                    }
                    location.apply(location_diff);
                } else {
                    *self = Self::Class {
                        subject_id: Uuid::nil(),
                        location: String::new(),
                    };
                    self.apply(diff);
                }
            }
            Some(BellDataDiff::Time) => *self = Self::Time,
            Some(BellDataDiff::Break) => *self = Self::Break,
            Some(BellDataDiff::Study) => *self = Self::Study,
            Some(BellDataDiff::Pause) => *self = Self::Pause,
            None => {}
        }
    }

    fn identity() -> Self {
        Self::Time
    }
}

impl BellData {
    #[must_use]
    /// Get the SF Symbols name of the icon associated with the bell type.
    /// Returns [`None`] when the bell type is [`BellData::Class`].
    ///
    /// # Examples
    ///
    /// ```
    /// use subjective::school::bells::BellData;
    /// use uuid::Uuid;
    ///
    /// let class = BellData::Class {
    ///     subject_id: Uuid::new_v4(),
    ///     location: "D14".to_string(),
    /// };
    /// assert_eq!(class.icon(), None);
    /// assert_eq!(BellData::Time.icon(), Some("clock.fill".to_string()));
    /// ```
    pub fn icon(&self) -> Option<String> {
        match self {
            Self::Class { .. } => None,
            Self::Time => Some("clock.fill".to_string()),
            Self::Break => Some("fork.knife".to_string()),
            Self::Study => Some("book.fill".to_string()),
            Self::Pause => Some("pause.fill".to_string()),
        }
    }

    pub(crate) fn from_ir(bell_time: &ir::BellTime) -> Option<Self> {
        bell_time.bell_type.as_ref().map_or_else(
            || {
                if let ir::BellTime {
                    subject_id: Some(subject_id),
                    location,
                    ..
                } = bell_time
                {
                    Some(Self::Class {
                        subject_id: *subject_id,
                        location: location.clone(),
                    })
                } else {
                    None
                }
            },
            |bell_type| match bell_type.name.as_str() {
                "Time" => Some(Self::Time),
                "Break" => Some(Self::Break),
                "Study" => Some(Self::Study),
                "Pause" => Some(Self::Pause),
                _ => None,
            },
        )
    }

    pub(crate) fn to_ir(&self) -> Option<ir::BellType> {
        match self {
            Self::Class { .. } => None,
            _ => Some(ir::BellType {
                name: self.to_string(),
                icon: self.icon().unwrap_or_default(),
            }),
        }
    }

    /// Returns `true` if the bell data is [`BellData::Class`].
    #[must_use]
    pub const fn is_class(&self) -> bool {
        matches!(self, Self::Class { .. })
    }

    /// Returns `true` if the bell data is [`BellData::Time`].
    #[must_use]
    pub const fn is_time(&self) -> bool {
        matches!(self, Self::Time)
    }

    /// Returns `true` if the bell data is [`BellData::Break`].
    #[must_use]
    pub const fn is_break(&self) -> bool {
        matches!(self, Self::Break)
    }

    /// Returns `true` if the bell data is [`BellData::Study`].
    #[must_use]
    pub const fn is_study(&self) -> bool {
        matches!(self, Self::Study)
    }

    /// Returns `true` if the bell data is [`BellData::Pause`].
    #[must_use]
    pub const fn is_pause(&self) -> bool {
        matches!(self, Self::Pause)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_icon_works() {
        assert_eq!(BellData::Time.icon(), Some("clock.fill".to_string()));
        assert_eq!(BellData::Break.icon(), Some("fork.knife".to_string()));
        assert_eq!(BellData::Study.icon(), Some("book.fill".to_string()));
        assert_eq!(BellData::Pause.icon(), Some("pause.fill".to_string()));
        assert_eq!(
            BellData::Class {
                subject_id: Uuid::new_v4(),
                location: "Classroom".to_string(),
            }
            .icon(),
            None
        );
    }
}
