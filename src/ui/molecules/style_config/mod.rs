//! Style configuration UI system for widgets
//!
//! Provides a trait-based abstraction for rendering style configuration
//! controls for different widget types. Each widget type implements the
//! `StyleConfigUI` trait to provide its own customization interface.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Trait for widget style configuration UI
///
/// Each widget type implements this trait to provide:
/// - Type-safe style options
/// - UI rendering for style controls
/// - Default values
///
/// # Example
/// ```rust
/// use crate::features::dashboard::config::style::LineStyleOptions;
///
/// #[derive(Clone, Copy)]
/// pub struct LineStyleConfig;
///
/// impl StyleConfigUI for LineStyleConfig {
///     type Options = LineStyleOptions;
///
///     fn render_controls(
///         options: &StyleOptionsSignal<Self::Options>,
///         on_change: &Callback<Self::Options>,
///     ) -> impl IntoView {
///         // Render style controls
///     }
/// }
/// ```
pub trait StyleConfigUI: Clone + Copy + Send + Sync + 'static {
    /// The style options type for this widget
    ///
    /// Must implement Clone, Serialize, Deserialize, Default, and be Send + Sync
    type Options: Clone + Serialize + for<'de> Deserialize<'de> + Default + Send + Sync + PartialEq;

    /// Render configuration controls for this widget type
    ///
    /// # Arguments
    /// * `options` - Signal providing the current style options
    /// * `on_change` - Callback to invoke when style options change
    ///
    /// # Returns
    /// A view containing the style configuration controls
    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView;
}

// Widget-specific style config implementations
pub mod line;
pub mod bar;
pub mod pie;
pub mod kpi;
pub mod table;
pub mod scatter;
pub mod area;
pub mod radar;
pub mod candlestick;
pub mod heatmap;
pub mod treemap;

// Re-exports for convenience
pub use line::LineStyleConfig;
pub use bar::BarStyleConfig;
pub use pie::PieStyleConfig;
pub use kpi::KpiStyleConfig;
pub use table::TableStyleConfig;
pub use scatter::ScatterStyleConfig;
pub use area::AreaStyleConfig;
pub use radar::RadarStyleConfig;
pub use candlestick::CandlestickStyleConfig;
pub use heatmap::HeatmapStyleConfig;
pub use treemap::TreemapStyleConfig;
