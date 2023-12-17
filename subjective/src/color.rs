use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
/// A color with red, green, and blue components.
pub struct Color {
    /// Red component. (0_f32..=1_f32)
    pub red: f32,
    /// Green component. (0_f32..=1_f32)
    pub green: f32,
    /// Blue component. (0_f32..=1_f32)
    pub blue: f32,
}
