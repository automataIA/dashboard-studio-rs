//! Candlestick widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::CandlestickStyleOptions;
use crate::features::dashboard::config::theme_colors::ChartColors;
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::transform::find_field_index;
use serde_json::{json, Value};

/// Candlestick widget configuration builder
#[derive(Clone, Copy)]
pub struct CandlestickConfig;

impl CandlestickConfig {
    /// Transform dataset to candlestick format
    ///
    /// Returns ECharts-compatible 2D array:
    /// ```json
    /// [
    ///   ["date", "open", "close", "low", "high"],
    ///   ["2024-01-01", 100, 105, 98, 110],
    ///   ...
    /// ]
    /// ```
    fn dataset_to_candlestick_format(
        dataset: &Dataset,
        mapping: &DataMapping,
    ) -> Result<Vec<Vec<Value>>, ConfigError> {
        // Find field indexes for OHLC data
        let date_idx = find_field_index(&dataset.fields, &mapping.x_axis)
            .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?;

        let open_idx = find_field_index(&dataset.fields, &mapping.open)
            .map_err(|e| ConfigError::DataTransformationError(format!("Open field not found: {}", e)))?;

        let close_idx = find_field_index(&dataset.fields, &mapping.close)
            .map_err(|e| ConfigError::DataTransformationError(format!("Close field not found: {}", e)))?;

        let low_idx = find_field_index(&dataset.fields, &mapping.low)
            .map_err(|e| ConfigError::DataTransformationError(format!("Low field not found: {}", e)))?;

        let high_idx = find_field_index(&dataset.fields, &mapping.high)
            .map_err(|e| ConfigError::DataTransformationError(format!("High field not found: {}", e)))?;

        // Build header row
        let header = vec![
            Value::String(dataset.fields[date_idx].name.clone()),
            Value::String("Open".to_string()),
            Value::String("Close".to_string()),
            Value::String("Low".to_string()),
            Value::String("High".to_string()),
        ];

        let mut echarts_data = vec![header];

        // Transform data rows
        for row in &dataset.data {
            if let (Some(date), Some(open), Some(close), Some(low), Some(high)) = (
                row.get(date_idx),
                row.get(open_idx),
                row.get(close_idx),
                row.get(low_idx),
                row.get(high_idx),
            ) {
                echarts_data.push(vec![
                    date.clone(),
                    open.clone(),
                    close.clone(),
                    low.clone(),
                    high.clone(),
                ]);
            }
        }

        Ok(echarts_data)
    }
}

impl WidgetConfigBuilder for CandlestickConfig {
    type StyleOptions = CandlestickStyleOptions;

    fn build_echarts_options(
        &self,
        dataset: &Dataset,
        mapping: &DataMapping,
        style: &Self::StyleOptions,
    ) -> Result<String, ConfigError> {
        // Transform dataset to candlestick format
        let echarts_data = Self::dataset_to_candlestick_format(dataset, mapping)?;

        // Read colors from active DaisyUI theme
        let colors = ChartColors::from_daisyui_theme();

        // Determine border colors
        let border_rise = style.border_rise_color.clone()
            .or_else(|| if style.custom_border_colors { None } else { Some(style.rise_color.clone()) })
            .unwrap_or(style.rise_color.clone());

        let border_fall = style.border_fall_color.clone()
            .or_else(|| if style.custom_border_colors { None } else { Some(style.fall_color.clone()) })
            .unwrap_or(style.fall_color.clone());

        // Build candlestick series
        let series = json!({
            "type": "candlestick",
            "barWidth": style.candle_width,
            "itemStyle": {
                "color": style.rise_color,
                "color0": style.fall_color,
                "borderColor": border_rise,
                "borderColor0": border_fall,
            },
            "label": if style.show_labels {
                Some(json!({
                    "show": true,
                    "fontSize": 10,
                    "position": "top"
                }))
            } else {
                None::<Value>
            },
            "animation": style.animation,
            "animationDuration": style.animation_duration,
            "animationEasing": "cubicOut"
        });

        // Build xAxis (date/category axis)
        let x_axis = json!({
            "type": "category",
            "boundaryGap": true,
            "axisLine": {
                "lineStyle": { "color": colors.grid }
            },
            "axisLabel": {
                "color": colors.label.clone(),
                "fontSize": 11,
                "rotate": 45
            },
            "splitLine": {
                "show": false
            }
        });

        // Build yAxis (value axis for prices)
        let y_axis = json!({
            "type": "value",
            "scale": true,
            "splitArea": {
                "show": true
            },
            "axisLine": {
                "lineStyle": { "color": colors.grid }
            },
            "splitLine": {
                "lineStyle": {
                    "color": colors.grid,
                    "type": "dashed"
                }
            },
            "axisLabel": {
                "color": colors.label,
                "fontSize": 11
            }
        });

        // Build complete ECharts options
        let options = json!({
            "dataset": {
                "source": echarts_data
            },
            "grid": {
                "left": "5%",
                "right": "5%",
                "bottom": "15%",
                "top": "10%",
                "containLabel": true
            },
            "tooltip": {
                "trigger": "axis",
                "axisPointer": {
                    "type": "cross"
                },
                "backgroundColor": colors.background.clone(),
                "borderColor": colors.grid,
                "textStyle": {
                    "color": colors.text
                }
            },
            "xAxis": x_axis,
            "yAxis": y_axis,
            "series": [series]
        });

        serde_json::to_string(&options)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))
    }

    fn validate_config(&self, mapping: &DataMapping) -> Result<(), ConfigError> {
        if mapping.x_axis.is_none() {
            return Err(ConfigError::MissingField("Date field (X-axis) is required for candlestick charts".to_string()));
        }
        if mapping.open.is_none() {
            return Err(ConfigError::MissingField("Open field is required for candlestick charts".to_string()));
        }
        if mapping.close.is_none() {
            return Err(ConfigError::MissingField("Close field is required for candlestick charts".to_string()));
        }
        if mapping.low.is_none() {
            return Err(ConfigError::MissingField("Low field is required for candlestick charts".to_string()));
        }
        if mapping.high.is_none() {
            return Err(ConfigError::MissingField("High field is required for candlestick charts".to_string()));
        }
        Ok(())
    }

    fn required_fields(&self) -> Vec<FieldRequirement> {
        vec![
            FieldRequirement::Single {
                name: "Date",
                field_type: FieldType::Date,
                required: true,
            },
            FieldRequirement::Single {
                name: "Open",
                field_type: FieldType::Numeric,
                required: true,
            },
            FieldRequirement::Single {
                name: "Close",
                field_type: FieldType::Numeric,
                required: true,
            },
            FieldRequirement::Single {
                name: "Low",
                field_type: FieldType::Numeric,
                required: true,
            },
            FieldRequirement::Single {
                name: "High",
                field_type: FieldType::Numeric,
                required: true,
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        CandlestickStyleOptions::default()
    }
}
