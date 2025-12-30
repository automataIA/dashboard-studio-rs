//! Widget config builders
//!
//! Each widget type has a builder that implements `WidgetConfigBuilder`
//! to generate ECharts options from datasets.

pub mod line;
pub mod bar;
pub mod pie;
pub mod candlestick;
pub mod area;
pub mod scatter;
pub mod radar;
pub mod heatmap;
pub mod treemap;

pub use line::LineConfig;
pub use bar::BarConfig;
pub use pie::PieConfig;
pub use candlestick::CandlestickConfig;
pub use area::AreaConfig;
pub use scatter::ScatterConfig;
pub use radar::RadarConfig;
pub use heatmap::HeatmapConfig;
pub use treemap::TreemapConfig;
