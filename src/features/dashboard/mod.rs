pub mod models;
pub mod context;
pub mod components;
pub mod echarts_bindings;
pub mod drag_drop;
pub mod csv_upload;
pub mod history;
pub mod kpi_aggregation;

// New configuration and data processing modules
pub mod config;
pub mod data;

// Template export/import modules
pub mod export;
pub mod io;

pub use models::*;
pub use context::DashboardContext;
pub use drag_drop::{DragDropManager, FieldDragDrop};
pub use kpi_aggregation::{calculate_kpi, analyze_dataset_for_kpis};
// Export/Import services
pub use export::{
    TemplateType,
    ExportService,
};

// Re-export commonly used types (unused imports removed)
// pub use config::{WidgetConfigBuilder, FieldRequirement, ConfigError};
// pub use data::{aggregate_data, dataset_to_echarts_format, TransformError, AggregationFunction};
