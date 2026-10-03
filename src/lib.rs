pub use ::dpi::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, str::FromStr};
pub mod app;
pub mod events;
pub mod muda;
pub mod package;
pub mod webview;
pub mod window;
pub mod wire;
/// System theme.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Theme {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
}

impl Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.to_lowercase().as_str() {
            "dark" => Self::Dark,
            _ => Self::Light,
        })
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Light => "light",
                Self::Dark => "dark",
            }
        )
    }
}
/// A rectangular region.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Rect {
    /// Rect position.
    pub position: dpi::Position,
    /// Rect size.
    pub size: dpi::Size,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Position::Logical((0, 0).into()),
            size: Size::Logical((0, 0).into()),
        }
    }
}

/// A rectangular region in physical pixels.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PhysicalRect<P: dpi::Pixel, S: dpi::Pixel> {
    /// Rect position.
    pub position: dpi::PhysicalPosition<P>,
    /// Rect size.
    pub size: dpi::PhysicalSize<S>,
}

impl<P: dpi::Pixel, S: dpi::Pixel> Default for PhysicalRect<P, S> {
    fn default() -> Self {
        Self {
            position: (0, 0).into(),
            size: (0, 0).into(),
        }
    }
}

/// A rectangular region in logical pixels.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LogicalRect<P: dpi::Pixel, S: dpi::Pixel> {
    /// Rect position.
    pub position: dpi::LogicalPosition<P>,
    /// Rect size.
    pub size: dpi::LogicalSize<S>,
}

impl<P: dpi::Pixel, S: dpi::Pixel> Default for LogicalRect<P, S> {
    fn default() -> Self {
        Self {
            position: (0, 0).into(),
            size: (0, 0).into(),
        }
    }
}

/// A tuple struct of RGBA colors. Each value has minimum of 0 and maximum of 255.
#[derive(Debug, PartialEq, Eq, Serialize, Default, Clone, Copy)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl From<Color> for (u8, u8, u8, u8) {
    fn from(value: Color) -> Self {
        (value.0, value.1, value.2, value.3)
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        (value.0, value.1, value.2)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Color(value.0, value.1, value.2, value.3)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color(value.0, value.1, value.2, 255)
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        [value.0, value.1, value.2, value.3]
    }
}

impl From<Color> for [u8; 3] {
    fn from(value: Color) -> Self {
        [value.0, value.1, value.2]
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Color(value[0], value[1], value[2], value[3])
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color(value[0], value[1], value[2], 255)
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(mut color: &str) -> Result<Self, Self::Err> {
        color = color.trim().strip_prefix('#').unwrap_or(color);
        let color = match color.len() {
            3 => color
                .chars()
                .flat_map(|c| std::iter::repeat_n(c, 2))
                .chain(std::iter::repeat_n('f', 2))
                .collect(),
            6 => format!("{color}FF"),
            8 => color.to_string(),
            _ => {
                return Err(
                    "Invalid hex color length, must be either 3, 6 or 8, for example: #fff, #ffffff, or #ffffffff"
                        .into(),
                );
            }
        };

        let r = u8::from_str_radix(&color[0..2], 16).map_err(|e| e.to_string())?;
        let g = u8::from_str_radix(&color[2..4], 16).map_err(|e| e.to_string())?;
        let b = u8::from_str_radix(&color[4..6], 16).map_err(|e| e.to_string())?;
        let a = u8::from_str_radix(&color[6..8], 16).map_err(|e| e.to_string())?;

        Ok(Color(r, g, b, a))
    }
}

fn default_alpha() -> u8 {
    255
}

#[derive(Deserialize)]
#[serde(untagged)]
enum InnerColor {
    /// Color hex string, for example: #fff, #ffffff, or #ffffffff.
    String(String),
    /// Array of RGB colors. Each value has minimum of 0 and maximum of 255.
    Rgb((u8, u8, u8)),
    /// Array of RGBA colors. Each value has minimum of 0 and maximum of 255.
    Rgba((u8, u8, u8, u8)),
    /// Object of red, green, blue, alpha color values. Each value has minimum of 0 and maximum of 255.
    RgbaObject {
        red: u8,
        green: u8,
        blue: u8,
        #[serde(default = "default_alpha")]
        alpha: u8,
    },
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let color = InnerColor::deserialize(deserializer)?;
        let color = match color {
            InnerColor::String(string) => string.parse().map_err(serde::de::Error::custom)?,
            InnerColor::Rgb(rgb) => Color(rgb.0, rgb.1, rgb.2, 255),
            InnerColor::Rgba(rgb) => rgb.into(),
            InnerColor::RgbaObject {
                red,
                green,
                blue,
                alpha,
            } => Color(red, green, blue, alpha),
        };

        Ok(color)
    }
}

#[cfg(windows)]
#[derive(Debug, Serialize, Deserialize)]
pub enum FocusState {
    WindowFocused,
    WebviewFocused {
        webview_label: String,
    },
    Blured {
        last_focused_webview_label: Option<String>,
    },
}

#[cfg(windows)]
impl Default for FocusState {
    fn default() -> Self {
        Self::Blured {
            last_focused_webview_label: None,
        }
    }
}

/// A [`ResourceId`] is an integer value referencing a resource.
///
/// It could be considered to be the Tauri equivalent of a file descriptor
/// in POSIX-like operating systems.
pub type ResourceId = u32;

pub mod external {
    pub use serde_json;
    pub use url;
    pub use uuid;
}
