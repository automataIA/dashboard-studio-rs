//! Style options for Table widget

use serde::{Deserialize, Serialize};

/// Style options for Table widgets
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableStyleOptions {
    /// Show pagination
    #[serde(default = "default_show_pagination")]
    pub show_pagination: bool,

    /// Page size for pagination
    #[serde(default = "default_page_size")]
    pub page_size: u16,

    /// Show column sorting
    #[serde(default = "default_show_sorting")]
    pub show_sorting: bool,

    /// Enable striped rows
    #[serde(default)]
    pub striped: bool,

    /// Enable hover effects
    #[serde(default = "default_hover")]
    pub hover: bool,

    /// Row height (compact, normal, comfortable)
    #[serde(default = "default_row_height")]
    pub row_height: TableRowHeight,

    /// Show borders
    #[serde(default)]
    pub show_borders: bool,
}

/// Row height options
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum TableRowHeight {
    #[default]
    Normal,
    Compact,
    Comfortable,
}

fn default_show_pagination() -> bool {
    false
}
fn default_page_size() -> u16 {
    10
}
fn default_show_sorting() -> bool {
    true
}
fn default_hover() -> bool {
    true
}
fn default_row_height() -> TableRowHeight {
    TableRowHeight::default()
}

impl Default for TableStyleOptions {
    fn default() -> Self {
        Self {
            show_pagination: default_show_pagination(),
            page_size: default_page_size(),
            show_sorting: default_show_sorting(),
            striped: false,
            hover: default_hover(),
            row_height: default_row_height(),
            show_borders: true,
        }
    }
}
