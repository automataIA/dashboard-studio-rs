//! Style options for Candlestick widget

use serde::{Deserialize, Serialize};

/// Style options for Candlestick charts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CandlestickStyleOptions {
    /// Color when price went up (rise)
    #[serde(default = "default_rise_color")]
    pub rise_color: String,

    /// Color when price went down (fall)
    #[serde(default = "default_fall_color")]
    pub fall_color: String,

    /// Custom border colors
    #[serde(default)]
    pub custom_border_colors: bool,

    /// Border color for rise
    pub border_rise_color: Option<String>,

    /// Border color for fall
    pub border_fall_color: Option<String>,

    /// Candle width (1-20px)
    #[serde(default = "default_candle_width")]
    pub candle_width: u8,

    /// Show data labels
    #[serde(default)]
    pub show_labels: bool,

    /// Enable entry animation
    #[serde(default = "default_animation")]
    pub animation: bool,

    /// Animation duration in milliseconds
    #[serde(default = "default_animation_duration")]
    pub animation_duration: u64,
}

fn default_rise_color() -> String { "#00da3c".to_string() }
fn default_fall_color() -> String { "#ec0000".to_string() }
fn default_candle_width() -> u8 { 10 }
fn default_animation() -> bool { true }
fn default_animation_duration() -> u64 { 1000 }
