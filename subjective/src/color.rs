use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
/// A color with red, green, and blue components.
pub struct Color {
    /// Red component. (`0_f32..=1_f32`)
    pub red: f32,
    /// Green component. (`0_f32..=1_f32`)
    pub green: f32,
    /// Blue component. (`0_f32..=1_f32`)
    pub blue: f32,
}

impl Color {
    /// The Subjective blue color.
    pub const SUBJECTIVE_BLUE: Self = Self {
        red: 0.212,
        green: 0.525,
        blue: 1.,
    };

    /// Colorize a string with this color.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn color<S: Colorize>(&self, string: S) -> ColoredString {
        string.truecolor(
            (self.red * 255_f32) as u8,
            (self.green * 255_f32) as u8,
            (self.blue * 255_f32) as u8,
        )
    }
}
