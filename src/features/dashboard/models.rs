use serde::{Deserialize, Serialize};

/// Unique identifier for widgets
pub type WidgetId = String;

/// Unique identifier for datasets
pub type DatasetId = String;

/// Unique identifier for layers
pub type LayerId = String;

/// Field data type enumeration
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    Text,      // String/categorical data
    Numeric,   // Numbers (revenue, units)
    Date,      // Temporal data
    Boolean,   // True/false
}

impl FieldType {
    /// Get CSS color class for field type
    pub fn icon_color(&self) -> &'static str {
        match self {
            Self::Text => "text-base-content/60",
            Self::Numeric => "text-success",
            Self::Date => "text-base-content/60",
            Self::Boolean => "text-info",
        }
    }
}

/// Field within a dataset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

/// Widget type enumeration (unified system)
///
/// Replaces the old ChartType enum. Supports all 10 widget types with
/// contextual configurations in Data/Style/AI tabs.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum WidgetType {
    // Basic Widget Families (5 families, 15+ variants)
    Line,      // basic, smooth, step, stacked, area
    Bar,       // basic, stacked, grouped, race, waterfall
    Pie,       // basic, doughnut, rose
    Scatter,   // basic, bubble
    Area,      // basic, stacked

    // Advanced Widget Types (4 types)
    Radar,
    Candlestick,
    Heatmap,
    Treemap,

    // Non-ECharts Widget Types (2 types)
    Kpi,
    Table,
}

impl WidgetType {
    /// Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Line => "Line Chart",
            Self::Bar => "Bar Chart",
            Self::Pie => "Pie Chart",
            Self::Scatter => "Scatter Plot",
            Self::Area => "Area Chart",
            Self::Radar => "Radar Chart",
            Self::Candlestick => "Candlestick",
            Self::Heatmap => "Heatmap",
            Self::Treemap => "Treemap",
            Self::Kpi => "KPI",
            Self::Table => "Table",
        }
    }

    /// Check if this is an ECharts-based widget
    pub fn is_echarts(&self) -> bool {
        !matches!(self, Self::Kpi | Self::Table)
    }

    /// Get icon name for layer display
    pub fn icon_name(self) -> String {
        match self {
            Self::Line => "show-chart".into(),
            Self::Bar => "bar-chart".into(),
            Self::Pie => "pie-chart".into(),
            Self::Scatter => "scatter-plot".into(),
            Self::Area => "area-chart".into(),
            Self::Radar => "radar".into(),
            Self::Candlestick => "candlestick-chart".into(),
            Self::Heatmap => "grid-on".into(),
            Self::Treemap => "account-tree".into(),
            Self::Kpi => "monitoring".into(),
            Self::Table => "table-chart".into(),
        }
    }
}

/// Dataset representing uploaded data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    pub id: DatasetId,
    pub name: String,           // e.g., "Q3_Sales_Data.csv"
    pub size: String,           // e.g., "2.4 MB"
    pub uploaded_at: String,    // e.g., "Today"
    pub fields: Vec<Field>,
    pub active: bool,
    /// Raw data rows (each row is a Vec of JSON values)
    /// This is the parsed CSV data
    #[serde(default)]
    pub data: Vec<Vec<serde_json::Value>>,
}

/// Chart type enumeration
///
/// # Deprecated
/// This enum is deprecated in favor of `WidgetType`. Maintained for backward compatibility.
/// Use `WidgetType` instead for all new code.
#[deprecated(since = "0.2.0", note = "Use `WidgetType` instead")]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum ChartType {
    LineChart,
    BarChart,
    PieChart,
    #[allow(non_camel_case_types)]
    Kpi,
    Table,
}

/// Data mapping for widget axes and fields
///
/// Supports mapping from dataset fields to widget-specific configurations.
/// Different widget types use different subsets of these fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DataMapping {
    // Basic axes (Line, Bar, Area)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis: Option<String>,   // Field name for X-axis (dimension)

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub y_axis: Vec<String>,      // Field names for Y-axis (measures, can be multiple)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>, // Field for grouping/categorization (stacking)

    // Advanced fields for specialized widgets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,         // For Scatter: bubble size measure

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,        // For Scatter: color dimension

    // Candlestick-specific fields (OHLC)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,         // For Candlestick: open price field

    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,        // For Candlestick: close price field

    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,         // For Candlestick: high price field

    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,          // For Candlestick: low price field

    // Hierarchy for Treemap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy: Vec<String>,       // For Treemap: hierarchical fields

    // Table-specific fields
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,         // For Table: selected columns to display

    // KPI-specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_field: Option<String>,           // For KPI: field to aggregate

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_aggregation: Option<KpiAggregation>, // For KPI: how to aggregate
}

/// KPI aggregation type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum KpiAggregation {
    Sum,        // Sum of all values
    Average,    // Average (mean)
    Count,      // Count of rows
    Min,        // Minimum value
    Max,        // Maximum value
    Last,       // Last (most recent) value
    First,      // First value
}

impl KpiAggregation {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Sum => "Total",
            Self::Average => "Average",
            Self::Count => "Count",
            Self::Min => "Minimum",
            Self::Max => "Maximum",
            Self::Last => "Latest",
            Self::First => "First",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Sum => "sum", // Σ
            Self::Average => "average",
            Self::Count => "counter",
            Self::Min => "arrow-downward",
            Self::Max => "arrow-upward",
            Self::Last => "schedule",
            Self::First => "history",
        }
    }
}

/// ECharts configuration (stored as JSON string)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChartConfig {
    #[serde(default)]
    pub chart_type: Option<WidgetType>,
    pub data_mapping: DataMapping,
    pub style_options: String,    // JSON string of ECharts options
}

impl ChartConfig {
    pub fn default_with_type(widget_type: WidgetType) -> Self {
        Self {
            chart_type: Some(widget_type),
            data_mapping: DataMapping::default(),
            style_options: "{}".into(),
        }
    }
}

/// Grid position and size
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridPosition {
    pub x: u32,      // Grid column (0-11)
    pub y: u32,      // Grid row
    pub width: u32,  // Columns span (1-12)
    pub height: u32, // Rows span
}

impl Default for GridPosition {
    fn default() -> Self {
        Self { x: 0, y: 0, width: 4, height: 4 }
    }
}

impl Default for WidgetType {
    fn default() -> Self {
        Self::Line
    }
}

/// Widget on the dashboard canvas
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Widget {
    pub id: WidgetId,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,

    pub widget_type: WidgetType,
    pub chart_config: ChartConfig,
    pub grid_position: GridPosition,

    /// Runtime-only field - not persisted in exports
    #[serde(skip, default)]
    pub editing: bool,
}

/// Layer in the layer panel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: LayerId,
    pub widget_id: WidgetId,  // Associated widget
    pub label: String,
    pub icon: String,         // Material icon name

    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub visible: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub locked: bool,
}

// Helper functions for Layer defaults
fn default_true() -> bool { true }
fn is_true(v: &bool) -> bool { *v }
fn is_false(v: &bool) -> bool { !*v }

/// Template category for organizing template gallery
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TemplateCategory {
    Generic,
    Business,
    Sales,
    Finance,
}

/// Widget definition within a template layout
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateWidget {
    pub widget_type: WidgetType,
    pub title: String,
    pub grid_position: GridPosition,
    pub chart_config: ChartConfig,
}

/// Template for quick widget creation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub title: String,
    pub category: TemplateCategory,
    pub preview_type: WidgetType,
    pub widgets: Vec<TemplateWidget>, // Multi-widget layout
    pub default_config: ChartConfig,  // Legacy: for single widget templates
}

/// CSV upload and parsing errors
#[derive(Clone, Debug, PartialEq)]
pub enum CsvError {
    /// File size exceeds limit (max MB)
    FileTooLarge {
        max_mb: u64,
        actual_mb: u64,
    },
    /// Invalid CSV format
    InvalidCsvFormat(String),
    /// Empty file
    EmptyFile,
    /// No headers found
    NoHeaders,
    /// Inconsistent row lengths
    #[allow(dead_code)]
    InconsistentRowLength {
        expected: usize,
        found: usize,
    },
    /// File read error
    #[allow(dead_code)]
    FileReadError(String),
    /// Parsing error at specific row
    ParseError {
        row: usize,
        message: String,
    },
    /// Type inference failed
    TypeInferenceFailed(String),
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileTooLarge { max_mb, actual_mb } => {
                write!(
                    f,
                    "File too large: {} MB exceeds maximum of {} MB",
                    actual_mb, max_mb
                )
            }
            Self::InvalidCsvFormat(msg) => write!(f, "Invalid CSV format: {}", msg),
            Self::EmptyFile => write!(f, "File is empty"),
            Self::NoHeaders => write!(f, "CSV has no header row"),
            Self::InconsistentRowLength { expected, found } => write!(
                f,
                "Row length inconsistent: expected {} columns, found {}",
                expected, found
            ),
            Self::FileReadError(msg) => write!(f, "Failed to read file: {}", msg),
            Self::ParseError { row, message } => {
                write!(f, "Parse error at row {}: {}", row, message)
            }
            Self::TypeInferenceFailed(msg) => write!(f, "Type inference failed: {}", msg),
        }
    }
}

impl std::error::Error for CsvError {}
