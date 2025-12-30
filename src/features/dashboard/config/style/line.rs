//! Style options for Line widget

use serde::{Deserialize, Serialize};

/// Style options for Line charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineStyleOptions {
    /// Chart title
    #[serde(default)]
    pub title: Option<String>,

    /// X-axis title
    #[serde(default)]
    pub x_axis_title: Option<String>,

    /// Y-axis title
    #[serde(default)]
    pub y_axis_title: Option<String>,

    /// Enable smooth curves (Bezier interpolation)
    #[serde(default = "default_smooth")]
    pub smooth: bool,

    /// Fill area under the line
    #[serde(default)]
    pub area_fill: bool,

    /// Line thickness in pixels (1-10)
    #[serde(default = "default_line_width")]
    pub line_width: u8,

    /// Show data points on the line
    #[serde(default)]
    pub show_points: bool,

    /// Point size (if show_points is true)
    #[serde(default = "default_point_size")]
    pub point_size: u8,

    /// Enable entry animation
    #[serde(default = "default_animation")]
    pub animation: bool,

    /// Animation duration in milliseconds
    #[serde(default = "default_animation_duration")]
    pub animation_duration: u64,

    /// Show data labels
    #[serde(default)]
    pub show_labels: bool,

    /// Enable accessibility patterns for colorblind users (WCAG AAA)
    #[serde(default)]
    pub enable_patterns: bool,
}

fn default_smooth() -> bool { true }
fn default_line_width() -> u8 { 3 }
fn default_point_size() -> u8 { 4 }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }

impl Default for LineStyleOptions {
    fn default() -> Self {
        Self {
            title: None,
            x_axis_title: None,
            y_axis_title: None,
            smooth: default_smooth(),
            area_fill: false,
            line_width: default_line_width(),
            show_points: false,
            point_size: default_point_size(),
            animation: default_animation(),
            animation_duration: default_animation_duration(),
            show_labels: false,
            enable_patterns: false,
        }
    }
}
