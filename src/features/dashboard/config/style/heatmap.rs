//! Style options for Heatmap widget

use serde::{Deserialize, Serialize};

/// Style options for Heatmap charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeatmapStyleOptions {
    /// Color scale (gradient/ordinal)
    #[serde(default = "default_color_scale")]
    pub color_scale: String,

    /// Show cell values
    #[serde(default)]
    pub show_labels: bool,

    /// Show values in cells
    #[serde(default)]
    pub show_values: bool,

    /// Label position (inside/outside)
    #[serde(default = "default_label_position")]
    pub label_position: String,

    /// Gap between cells (0-10px)
    #[serde(default = "default_gap")]
    pub gap: u8,

    /// Cell border radius (0-10px)
    #[serde(default = "default_border_radius")]
    pub border_radius: u8,

    /// Interactive hover effects
    #[serde(default = "default_interactive")]
    pub interactive: bool,

    /// Minimum color value
    pub color_min: Option<String>,

    /// Maximum color value
    pub color_max: Option<String>,

    /// Chart title
    pub title: Option<String>,

    /// Enable entry animation
    #[serde(default = "default_animation")]
    pub animation: bool,

    /// Animation duration in milliseconds
    #[serde(default = "default_animation_duration")]
    pub animation_duration: u64,
}

fn default_color_scale() -> String { "gradient".to_string() }
fn default_label_position() -> String { "inside".to_string() }
fn default_gap() -> u8 { 2 }
fn default_border_radius() -> u8 { 0 }
fn default_interactive() -> bool { true }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }
