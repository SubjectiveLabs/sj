use chrono::NaiveTime;
use strum_macros::Display;
use uuid::Uuid;

pub(crate) mod ir;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Bell-related data.
pub struct BellTime {
    /// Name of the bell.
    pub name: String,
    /// Time of the bell.
    pub time: NaiveTime,
    /// Data related to the bell.
    pub bell_type: Option<BellData>,
    /// Whether the bell is enabled. Notifications will not be sent for disabled bells.
    pub enabled: bool,
}

#[derive(Display, Debug, Clone, PartialEq, Eq)]
/// Type of a bell.
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
    use uuid::Uuid;

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
