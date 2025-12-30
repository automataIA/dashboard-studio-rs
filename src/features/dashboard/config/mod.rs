//! Widget configuration system
//!
//! This module provides the trait-based architecture for building widget configurations.
//! Each widget type implements `WidgetConfigBuilder` to generate ECharts options.

pub mod accessibility;
pub mod builders;
pub mod style;
pub mod theme_colors;
pub mod traits;

// Unused exports kept for future use
// pub use traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
// pub use theme_colors::ChartColors;
