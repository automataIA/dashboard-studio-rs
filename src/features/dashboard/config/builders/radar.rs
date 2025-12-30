//! Radar widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::RadarStyleOptions;
use crate::features::dashboard::config::theme_colors::ChartColors;
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::transform::find_field_index;
use serde_json::{json, Value};

/// Radar widget configuration builder
#[derive(Clone, Copy)]
pub struct RadarConfig;

impl RadarConfig {
    /// Transform dataset to radar format
    fn dataset_to_radar_format(
        dataset: &Dataset,
        mapping: &DataMapping,
    ) -> Result<(Vec<Value>, Vec<Value>), ConfigError> {
        if mapping.y_axis.is_empty() {
            return Err(ConfigError::MissingField("At least one indicator field is required".to_string()));
        }

        // Find field indexes for all indicators
        let indicator_indexes: Vec<usize> = mapping.y_axis.iter()
            .map(|field_name| find_field_index(&dataset.fields, &Some(field_name.clone())))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?;

        // Build radar indicator configuration
        let indicators: Vec<Value> = mapping.y_axis.iter()
            .map(|name| json!({ "name": name }))
            .collect();

        // Get first row of data for demo (or aggregate multiple rows)
        let data_values: Vec<Value> = if let Some(row) = dataset.data.first() {
            indicator_indexes.iter()
                .filter_map(|&idx| row.get(idx).cloned())
                .collect()
        } else {
            vec![]
        };

        Ok((indicators, data_values))
    }
}

impl WidgetConfigBuilder for RadarConfig {
    type StyleOptions = RadarStyleOptions;

    fn build_echarts_options(
        &self,
        dataset: &Dataset,
        mapping: &DataMapping,
        style: &Self::StyleOptions,
    ) -> Result<String, ConfigError> {
        // Transform dataset to radar format
        let (indicators, data_values) = Self::dataset_to_radar_format(dataset, mapping)?;

        // Read colors from active DaisyUI theme
        let colors = ChartColors::from_daisyui_theme();
        let primary_color = colors.to_palette()[0].clone();

        // Build series
        let series = json!({
            "type": "radar",
            "data": [{
                "value": data_values,
                "name": "Values",
                "areaStyle": if style.filled {
                    Some(json!({
                        "color": format!("{}66", primary_color),  // 40% opacity
                    }))
                } else {
                    None::<Value>
                },
                "lineStyle": {
                    "color": primary_color.clone(),
                    "width": style.line_width as u32,
                },
                "itemStyle": {
                    "color": primary_color.clone(),
                },
                "label": if style.show_labels {
                    Some(json!({
                        "show": true,
                        "fontSize": 11,
                        "color": colors.label_high_contrast.clone(),
                    }))
                } else {
                    None::<Value>
                },
            }],
        });

        // Build title option
        let title_option = style.title.as_ref().map(|t| json!({
            "text": t,
            "left": "center",
            "top": "0%",
            "textStyle": {
                "color": colors.text.clone(),
                "fontSize": 16,
                "fontWeight": 600
            }
        }));

        // Build complete ECharts options
        let mut options = json!({
            "radar": {
                "indicator": indicators,
                "shape": if style.circular { "circle" } else { "polygon" },
                "splitNumber": 5,
                "axisName": {
                    "color": colors.label.clone(),
                    "fontSize": 12,
                },
                "splitLine": {
                    "lineStyle": {
                        "color": colors.grid.clone(),
                    }
                },
                "splitArea": {
                    "show": true,
                    "areaStyle": {
                        "color": [
                            format!("{}0d", colors.grid),  // 5% opacity
                            format!("{}1a", colors.grid),  // 10% opacity
                        ]
                    }
                },
                "axisLine": {
                    "lineStyle": {
                        "color": colors.grid.clone(),
                    }
                }
            },
            "tooltip": {
                "trigger": "item",
                "backgroundColor": colors.background.clone(),
                "borderColor": colors.grid.clone(),
                "borderWidth": 1,
                "textStyle": {
                    "color": colors.label_high_contrast.clone(),
                    "fontSize": 12,
                },
                "extraCssText": "box-shadow: 0 4px 6px rgba(0,0,0,0.1); border-radius: 0.5rem;",
            },
            "series": series
        });

        // Add title if present
        if let Some(title_obj) = title_option {
            options["title"] = title_obj;
        }

        serde_json::to_string(&options)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))
    }

    fn validate_config(&self, mapping: &DataMapping) -> Result<(), ConfigError> {
        if mapping.y_axis.is_empty() {
            return Err(ConfigError::MissingField("At least one indicator field is required for radar charts".to_string()));
        }
        Ok(())
    }

    fn required_fields(&self) -> Vec<FieldRequirement> {
        vec![
            FieldRequirement::Multiple {
                name: "Indicators",
                field_types: vec![FieldType::Numeric],
                min_count: 3,
                max_count: Some(8),
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        RadarStyleOptions::default()
    }
}
