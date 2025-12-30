//! Style options for Treemap widget

use serde::{Deserialize, Serialize};

/// Style options for Treemap charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TreemapStyleOptions {
    /// Visual mapping mode (value/squarifying)
    #[serde(default = "default_visual_mode")]
    pub visual_mode: String,

    /// Show node labels
    #[serde(default)]
    pub show_labels: bool,

    /// Label position (inside/outside)
    #[serde(default = "default_label_position")]
    pub label_position: String,

    /// Label size
    #[serde(default = "default_label_size")]
    pub label_size: u8,

    /// Gap between nodes (0-10px)
    #[serde(default = "default_gap")]
    pub gap: u8,

    /// Node border radius (0-10px)
    #[serde(default = "default_border_radius")]
    pub border_radius: u8,

    /// Show breadcrumbs for navigation
    #[serde(default)]
    pub show_breadcrumbs: bool,

    /// Show breadcrumb
    #[serde(default)]
    pub show_breadcrumb: bool,

    /// Leaf depth
    #[serde(default = "default_leaf_depth")]
    pub leaf_depth: u8,

    /// Color depth for hierarchy (0-10)
    #[serde(default = "default_color_depth")]
    pub color_depth: u8,

    /// Chart title
    pub title: Option<String>,

    /// Enable entry animation
    #[serde(default = "default_animation")]
    pub animation: bool,

    /// Animation duration in milliseconds
    #[serde(default = "default_animation_duration")]
    pub animation_duration: u64,
}

fn default_visual_mode() -> String { "squarifying".to_string() }
fn default_label_position() -> String { "inside".to_string() }
fn default_label_size() -> u8 { 12 }
fn default_leaf_depth() -> u8 { 1 }
fn default_gap() -> u8 { 2 }
fn default_border_radius() -> u8 { 4 }
fn default_color_depth() -> u8 { 0 }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }
