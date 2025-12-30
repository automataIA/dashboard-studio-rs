//! Style options for all widget types
//!
//! Each widget type has its own style options struct that defines
//! the configurable visual properties.

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

pub use line::LineStyleOptions;
pub use bar::BarStyleOptions;
pub use pie::PieStyleOptions;
pub use kpi::{KpiStyleOptions, KpiValueFormat};
pub use table::{TableStyleOptions, TableRowHeight};
pub use scatter::ScatterStyleOptions;
pub use area::AreaStyleOptions;
pub use radar::RadarStyleOptions;
pub use candlestick::CandlestickStyleOptions;
pub use heatmap::HeatmapStyleOptions;
pub use treemap::TreemapStyleOptions;
