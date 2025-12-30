//! Style options for Bar widget

use serde::{Deserialize, Serialize};

/// Style options for Bar charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BarStyleOptions {
    /// Chart title
    #[serde(default)]
    pub title: Option<String>,

    /// X-axis title
    #[serde(default)]
    pub x_axis_title: Option<String>,

    /// Y-axis title
    #[serde(default)]
    pub y_axis_title: Option<String>,

    /// Stack bars on top of each other
    #[serde(default)]
    pub stacked: bool,

    /// Horizontal bars instead of vertical
    #[serde(default)]
    pub horizontal: bool,

    /// Bar width (1-100, percentage of available space)
    #[serde(default = "default_bar_width")]
    pub bar_width: u8,

    /// Corner radius for bar tops (0-20 pixels)
    #[serde(default = "default_border_radius")]
    pub border_radius: u8,

    /// Show value labels on bars
    #[serde(default)]
    pub show_labels: bool,

    /// Enable entry animation
    #[serde(default = "default_animation")]
    pub animation: bool,

    /// Animation duration in milliseconds
    #[serde(default = "default_animation_duration")]
    pub animation_duration: u64,

    /// Enable accessibility patterns for colorblind users (WCAG AAA)
    #[serde(default)]
    pub enable_patterns: bool,
}

fn default_bar_width() -> u8 { 60 }
fn default_border_radius() -> u8 { 4 }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }

impl Default for BarStyleOptions {
    fn default() -> Self {
        Self {
            title: None,
            x_axis_title: None,
            y_axis_title: None,
            stacked: false,
            horizontal: false,
            bar_width: default_bar_width(),
            border_radius: default_border_radius(),
            show_labels: false,
            animation: default_animation(),
            animation_duration: default_animation_duration(),
            enable_patterns: false,
        }
    }
}
