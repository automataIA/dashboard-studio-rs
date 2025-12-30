//! Style options for KPI widget

use serde::{Deserialize, Serialize};

/// Style options for KPI widgets
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiStyleOptions {
    /// Number format (currency, number, percentage)
    #[serde(default = "default_value_format")]
    pub value_format: KpiValueFormat,

    /// Show trend indicator
    #[serde(default = "default_show_trend")]
    pub show_trend: bool,

    /// Show progress bar
    #[serde(default = "default_show_progress")]
    pub show_progress: bool,

    /// Decimal places for numbers
    #[serde(default = "default_decimals")]
    pub decimals: u8,

    /// Enable comparison text
    #[serde(default)]
    pub show_comparison: bool,
}

/// Number format for KPI value
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum KpiValueFormat {
    #[default]
    Number,
    Currency,
    Percentage,
    Custom,
}

fn default_value_format() -> KpiValueFormat { KpiValueFormat::default() }
fn default_show_trend() -> bool { true }
fn default_show_progress() -> bool { true }
fn default_decimals() -> u8 { 0 }

impl Default for KpiStyleOptions {
    fn default() -> Self {
        Self {
            value_format: default_value_format(),
            show_trend: default_show_trend(),
            show_progress: default_show_progress(),
            decimals: default_decimals(),
            show_comparison: true,
        }
    }
}
