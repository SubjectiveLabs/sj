use chrono::NaiveTime;
use strum_macros::Display;
use uuid::Uuid;

pub(crate) mod ir {
    use uuid::Uuid;

    pub struct BellTime {
        pub id: Uuid,
        pub name: String,
        pub minute: u32,
        pub hour: u32,
        pub subject_id: Uuid,
        pub location: String,
        pub bell_type: Option<BellType>,
        pub enabled: bool,
    }

    pub struct BellType {
        pub id: Uuid,
        pub name: String,
        pub icon: String,
    }
}

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

/// Type of a bell.
#[derive(Display)]
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
    pub fn icon(&self) -> Option<String> {
        match self {
            Self::Class { .. } => None,
            Self::Time => Some("clock.fill".to_string()),
            Self::Break => Some("fork.knife".to_string()),
            Self::Study => Some("book.fill".to_string()),
            Self::Pause => Some("pause.fill".to_string()),
        }
    }
}
