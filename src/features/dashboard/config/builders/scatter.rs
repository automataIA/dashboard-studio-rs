//! Scatter widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::ScatterStyleOptions;
use crate::features::dashboard::config::theme_colors::{ChartColors, lighten_color};
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::transform::find_field_index;
use serde_json::{json, Value};

/// Scatter widget configuration builder
#[derive(Clone, Copy)]
pub struct ScatterConfig;

impl ScatterConfig {
    /// Transform dataset to scatter format
    ///
    /// Returns ECharts-compatible format with optional size and color dimensions:
    /// ```json
    /// [
    ///   [x_value, y_value, size_value, color_value],
    ///   ...
    /// ]
    /// ```
    fn dataset_to_scatter_format(
        dataset: &Dataset,
        mapping: &DataMapping,
    ) -> Result<Vec<Vec<Value>>, ConfigError> {
        // Find field indexes
        let x_idx = find_field_index(&dataset.fields, &mapping.x_axis)
            .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?;

        let y_idx = if !mapping.y_axis.is_empty() {
            find_field_index(&dataset.fields, &Some(mapping.y_axis[0].clone()))
                .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?
        } else {
            return Err(ConfigError::MissingField("Y-axis field is required".to_string()));
        };

        let size_idx = mapping.size.as_ref()
            .and_then(|s| find_field_index(&dataset.fields, &Some(s.clone())).ok());

        let color_idx = mapping.color.as_ref()
            .and_then(|c| find_field_index(&dataset.fields, &Some(c.clone())).ok());

        // Transform data rows
        let mut scatter_data: Vec<Vec<Value>> = Vec::new();

        for row in &dataset.data {
            if let (Some(x_val), Some(y_val)) = (row.get(x_idx), row.get(y_idx)) {
                let mut data_point = vec![x_val.clone(), y_val.clone()];

                // Add size dimension if specified
                if let Some(size_i) = size_idx {
                    if let Some(size_val) = row.get(size_i) {
                        data_point.push(size_val.clone());
                    }
                }

                // Add color dimension if specified
                if let Some(color_i) = color_idx {
                    if let Some(color_val) = row.get(color_i) {
                        data_point.push(color_val.clone());
                    }
                }

                scatter_data.push(data_point);
            }
        }

        Ok(scatter_data)
    }
}

impl WidgetConfigBuilder for ScatterConfig {
    type StyleOptions = ScatterStyleOptions;

    fn build_echarts_options(
        &self,
        dataset: &Dataset,
        mapping: &DataMapping,
        style: &Self::StyleOptions,
    ) -> Result<String, ConfigError> {
        // Transform dataset to scatter format
        let scatter_data = Self::dataset_to_scatter_format(dataset, mapping)?;

        // Read colors from active DaisyUI theme
        let colors = ChartColors::from_daisyui_theme();
        let primary_color = colors.to_palette()[0].clone();

        // Determine symbol size
        let symbol_size = if mapping.size.is_some() {
            // Use data-driven size
            json!([style.point_size_min as u32, style.point_size_max as u32])
        } else {
            // Fixed size
            json!(style.point_size_min as u32)
        };

        // Build series
        let series = json!({
            "type": "scatter",
            "data": scatter_data,
            "symbolSize": symbol_size,
            "itemStyle": {
                "color": primary_color.clone(),
                "opacity": style.opacity as f64 / 100.0,
            },
            "emphasis": {
                "focus": "self",
                "itemStyle": {
                    "color": lighten_color(&primary_color, 0.3),
                    "borderColor": colors.label_high_contrast.clone(),
                    "borderWidth": 2,
                    "shadowBlur": 10,
                    "shadowColor": "rgba(0, 0, 0, 0.3)",
                },
                "label": {
                    "show": true,
                    "fontSize": 12,
                    "fontWeight": "bold",
                    "color": colors.label_high_contrast.clone(),
                }
            },
            "label": if style.show_labels {
                Some(json!({
                    "show": true,
                    "position": "top",
                    "fontSize": 10,
                    "color": colors.label_high_contrast.clone(),
                }))
            } else {
                None::<serde_json::Value>
            },
        });

        // Build xAxis
        let mut x_axis = json!({
            "type": "value",
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
                "color": colors.label.clone(),
                "fontSize": 11
            }
        });
        if let Some(ref title) = style.x_axis_title {
            x_axis["name"] = json!(title);
            x_axis["nameLocation"] = json!("middle");
            x_axis["nameTextStyle"] = json!({
                "color": colors.label.clone(),
                "fontSize": 12,
                "fontWeight": 500
            });
            x_axis["nameGap"] = json!(30);
        }

        // Build yAxis
        let mut y_axis = json!({
            "type": "value",
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
                "color": colors.label.clone(),
                "fontSize": 11
            }
        });
        if let Some(ref title) = style.y_axis_title {
            y_axis["name"] = json!(title);
            y_axis["nameLocation"] = json!("middle");
            y_axis["nameRotate"] = json!(90);
            y_axis["nameTextStyle"] = json!({
                "color": colors.label.clone(),
                "fontSize": 12,
                "fontWeight": 500
            });
            y_axis["nameGap"] = json!(50);
        }

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

        // Adjust grid top if title is present
        let grid_top = if style.title.is_some() { "15%" } else { "10%" };

        // Build complete ECharts options
        let mut options = json!({
            "grid": {
                "left": if style.y_axis_title.is_some() { "8%" } else { "3%" },
                "right": "4%",
                "bottom": if style.x_axis_title.is_some() { "15%" } else { "10%" },
                "top": grid_top,
                "containLabel": true
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
            "xAxis": x_axis,
            "yAxis": y_axis,
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
        if mapping.x_axis.is_none() {
            return Err(ConfigError::MissingField("X-axis field is required for scatter plots".to_string()));
        }
        if mapping.y_axis.is_empty() {
            return Err(ConfigError::MissingField("Y-axis field is required for scatter plots".to_string()));
        }
        Ok(())
    }

    fn required_fields(&self) -> Vec<FieldRequirement> {
        vec![
            FieldRequirement::Single {
                name: "X-Axis",
                field_type: FieldType::Numeric,
                required: true,
            },
            FieldRequirement::Single {
                name: "Y-Axis",
                field_type: FieldType::Numeric,
                required: true,
            },
            FieldRequirement::Single {
                name: "Size (Optional)",
                field_type: FieldType::Numeric,
                required: false,
            },
            FieldRequirement::Single {
                name: "Color (Optional)",
                field_type: FieldType::Text,
                required: false,
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        ScatterStyleOptions::default()
    }
}
