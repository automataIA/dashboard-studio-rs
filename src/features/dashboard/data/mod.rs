//! Data processing utilities
//!
//! This module provides data aggregation and transformation utilities
//! for converting CSV data into ECharts-compatible formats.

pub mod aggregation;
pub mod transform;

pub use aggregation::AggregationFunction;
pub use transform::dataset_to_echarts_format;
// Unused exports kept for future use
// pub use aggregation::aggregate_data;
// pub use transform::TransformError;
