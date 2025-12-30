//! Style options for Radar widget

use serde::{Deserialize, Serialize};

/// Style options for Radar charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RadarStyleOptions {
    /// Radar shape (polygon/circle)
    #[serde(default = "default_shape")]
    pub shape: String,

    /// Show axis labels
    #[serde(default = "default_show_axis_labels")]
    pub show_axis_labels: bool,

    /// Split radar area by axis
    #[serde(default)]
    pub split_area: bool,

    /// Area opacity (10-100%)
    #[serde(default = "default_opacity")]
    pub opacity: u8,

    /// Border width / line width
    #[serde(default = "default_border_width")]
    pub border_width: u8,

    /// Show data points
    #[serde(default)]
    pub show_points: bool,

    /// Point size
    #[serde(default = "default_point_size")]
    pub point_size: u8,

    /// Fill the area under the radar
    #[serde(default = "default_filled")]
    pub filled: bool,

    /// Show labels on data points
    #[serde(default)]
    pub show_labels: bool,

    /// Chart title
    pub title: Option<String>,

    /// Use circular shape (vs polygon)
    #[serde(default)]
    pub circular: bool,

    /// Line width
    #[serde(default = "default_line_width")]
    pub line_width: u8,

    /// Enable entry animation
    #[serde(default = "default_animation")]
    pub animation: bool,

    /// Animation duration in milliseconds
    #[serde(default = "default_animation_duration")]
    pub animation_duration: u64,
}

fn default_shape() -> String { "polygon".to_string() }
fn default_show_axis_labels() -> bool { true }
fn default_opacity() -> u8 { 50 }
fn default_border_width() -> u8 { 2 }
fn default_point_size() -> u8 { 4 }
fn default_filled() -> bool { true }
fn default_line_width() -> u8 { 2 }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }
