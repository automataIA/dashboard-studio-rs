//! Style options for Area widget

use serde::{Deserialize, Serialize};

/// Style options for Area charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AreaStyleOptions {
    /// Enable smooth curves (Bezier interpolation)
    #[serde(default = "default_smooth")]
    pub smooth: bool,

    /// Area opacity (10-100%)
    #[serde(default = "default_opacity")]
    pub opacity: u8,

    /// Stack areas on top of each other
    #[serde(default)]
    pub stacked: bool,

    /// Show area border
    #[serde(default = "default_show_border")]
    pub show_border: bool,

    /// Border width (if show_border is true)
    #[serde(default = "default_border_width")]
    pub border_width: u8,

    /// Show data points
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
}

fn default_smooth() -> bool { true }
fn default_opacity() -> u8 { 60 }
fn default_show_border() -> bool { true }
fn default_border_width() -> u8 { 2 }
fn default_point_size() -> u8 { 4 }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }
