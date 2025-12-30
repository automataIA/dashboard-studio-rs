//! Style options for Pie widget

use serde::{Deserialize, Serialize};

/// Style options for Pie charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PieStyleOptions {
    /// Chart title
    #[serde(default)]
    pub title: Option<String>,

    /// Inner radius for donut chart (0-100%, "0%" = pie, "50%" = donut)
    #[serde(default = "default_inner_radius")]
    pub inner_radius: String,

    /// Enable rose/Nightingale chart (radius varies by value)
    #[serde(default)]
    pub rose_type: bool,

    /// Show data labels on slices
    #[serde(default = "default_show_labels")]
    pub show_labels: bool,

    /// Label position ("inside" or "outside")
    #[serde(default = "default_label_position")]
    pub label_position: String,

    /// Border radius for slices (0-20 pixels)
    #[serde(default = "default_border_radius")]
    pub border_radius: u8,

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

fn default_inner_radius() -> String { "0%".to_string() }
fn default_show_labels() -> bool { true }
fn default_label_position() -> String { "outside".to_string() }
fn default_border_radius() -> u8 { 4 }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }

impl Default for PieStyleOptions {
    fn default() -> Self {
        Self {
            title: None,
            inner_radius: default_inner_radius(),
            rose_type: false,
            show_labels: default_show_labels(),
            label_position: default_label_position(),
            border_radius: default_border_radius(),
            animation: default_animation(),
            animation_duration: default_animation_duration(),
            enable_patterns: false,
        }
    }
}
