#![cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::color::Color;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Subject, normally related to a [`crate::school::bells::BellData::Class`].
pub struct Subject {
    /// Unique identifier.
    pub id: Uuid,
    /// Name of the subject.
    pub name: String,
    /// [`Color`] of the subject.
    pub color: Color,
    /// Locations where the subject is taught.
    pub locations: Vec<String>,
    #[serde(rename = "iconName")]
    /// SF Symbols icon of the subject.
    pub icon: String,
}
