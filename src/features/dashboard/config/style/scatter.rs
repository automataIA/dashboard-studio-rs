//! Style options for Scatter widget

use serde::{Deserialize, Serialize};

/// Style options for Scatter charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScatterStyleOptions {
    /// Size of scatter points (2-20px)
    #[serde(default = "default_point_size")]
    pub point_size: u8,

    /// Minimum point size (for bubble charts)
    #[serde(default = "default_point_size_min")]
    pub point_size_min: u8,

    /// Maximum point size (for bubble charts)
    #[serde(default = "default_point_size_max")]
    pub point_size_max: u8,

    /// Point opacity (10-100%)
    #[serde(default = "default_opacity")]
    pub opacity: u8,

    /// Enable bubble chart mode
    #[serde(default)]
    pub show_bubble: bool,

    /// Show symbol labels
    #[serde(default)]
    pub show_labels: bool,

    /// Label position (inside/outside)
    #[serde(default = "default_label_position")]
    pub label_position: String,

    /// X-axis title
    pub x_axis_title: Option<String>,

    /// Y-axis title
    pub y_axis_title: Option<String>,

    /// Chart title
    pub title: Option<String>,

    /// Enable entry animation
    #[serde(default = "default_animation")]
    pub animation: bool,

    /// Animation duration in milliseconds
    #[serde(default = "default_animation_duration")]
    pub animation_duration: u64,
}

fn default_point_size() -> u8 { 6 }
fn default_point_size_min() -> u8 { 6 }
fn default_point_size_max() -> u8 { 30 }
fn default_opacity() -> u8 { 80 }
fn default_label_position() -> String { "inside".to_string() }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }
