//! Widget configuration builder traits
//!
//! Defines the trait interface that all widget config builders must implement.

use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use serde::{Deserialize, Serialize};

/// Error types for widget configuration
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ConfigError {
    /// Required field is missing
    MissingField(String),

    /// Field has wrong type
    InvalidFieldType {
        field: String,
        expected: FieldType,
        found: FieldType,
    },

    /// Data transformation failed
    TransformationError(String),

    /// Invalid configuration value
    InvalidValue(String),

    /// Data transformation error
    DataTransformationError(String),

    /// JSON serialization error
    SerializationError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "Required field missing: {}", field),
            Self::InvalidFieldType { field, expected, found } => {
                write!(
                    f,
                    "Field '{}' has invalid type: expected {:?}, found {:?}",
                    field, expected, found
                )
            }
            Self::TransformationError(msg) => write!(f, "Data transformation failed: {}", msg),
            Self::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            Self::DataTransformationError(msg) => write!(f, "Data transformation error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Field requirement for widget configuration
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum FieldRequirement {
    /// Single field with specific type
    Single {
        name: &'static str,
        field_type: FieldType,
        required: bool,
    },

    /// Multiple fields with specific types
    Multiple {
        name: &'static str,
        field_types: Vec<FieldType>,
        min_count: usize,
        max_count: Option<usize>,
    },

    /// Specialized OHLC requirement for candlestick
    Ohlc, // Open, High, Low, Close (all Numeric)
}

impl FieldRequirement {
    /// Get the display name for this requirement
    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Single { name, .. } => name,
            Self::Multiple { name, .. } => name,
            Self::Ohlc => "OHLC (Open, High, Low, Close)",
        }
    }
}

/// Trait for building widget configurations
///
/// Each widget type implements this trait to provide:
/// - ECharts option generation from dataset
/// - Configuration validation
/// - Field requirement specification
pub trait WidgetConfigBuilder: Clone + Copy + Send + Sync {
    /// Style options type for this widget
    type StyleOptions: Clone + Serialize + for<'de> Deserialize<'de>;

    /// Build ECharts options JSON from dataset and configuration
    ///
    /// # Arguments
    /// * `dataset` - The active dataset with field definitions
    /// * `mapping` - Field mapping configuration
    /// * `style` - Widget-specific style options
    ///
    /// # Returns
    /// ECharts option JSON string, or error if configuration is invalid
    fn build_echarts_options(
        &self,
        dataset: &Dataset,
        mapping: &DataMapping,
        style: &Self::StyleOptions,
    ) -> Result<String, ConfigError>;

    /// Validate if the current data mapping is sufficient for this widget
    ///
    /// # Returns
    /// Ok(()) if valid, Err(ConfigError) with details if invalid
    #[allow(dead_code)]
    fn validate_config(&self, mapping: &DataMapping) -> Result<(), ConfigError>;

    /// Get the field requirements for this widget type
    ///
    /// # Returns
    /// Vector of field requirements needed for this widget
    #[allow(dead_code)]
    fn required_fields(&self) -> Vec<FieldRequirement>;

    /// Get default style options for this widget
    ///
    /// # Returns
    /// Default style options instance
    #[allow(dead_code)]
    fn default_style(&self) -> Self::StyleOptions;
}

