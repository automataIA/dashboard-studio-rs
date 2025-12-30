//! Heatmap widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::HeatmapStyleOptions;
use crate::features::dashboard::config::theme_colors::ChartColors;
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::transform::find_field_index;
use serde_json::{json, Value};

/// Heatmap widget configuration builder
#[derive(Clone, Copy)]
pub struct HeatmapConfig;

impl HeatmapConfig {
    /// Transform dataset to heatmap format
    ///
    /// Returns (data, x_categories, y_categories)
    fn dataset_to_heatmap_format(
        dataset: &Dataset,
        mapping: &DataMapping,
    ) -> Result<(Vec<Vec<Value>>, Vec<String>, Vec<String>), ConfigError> {
        // For heatmap, we need x_axis (x categories), category (y categories), and y_axis[0] (values)
        let x_idx = find_field_index(&dataset.fields, &mapping.x_axis)
            .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?;

        let y_category_idx = find_field_index(&dataset.fields, &mapping.category)
            .map_err(|_e| ConfigError::DataTransformationError("Category field required for Y-axis grouping".to_string()))?;

        let value_idx = if !mapping.y_axis.is_empty() {
            find_field_index(&dataset.fields, &Some(mapping.y_axis[0].clone()))
                .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?
        } else {
            return Err(ConfigError::MissingField("Value field is required".to_string()));
        };

        // Collect unique categories for both axes
        let mut x_categories: Vec<String> = Vec::new();
        let mut y_categories: Vec<String> = Vec::new();

        for row in &dataset.data {
            if let Some(x_val) = row.get(x_idx) {
                let x_str = x_val.as_str().unwrap_or("").to_string();
                if !x_categories.contains(&x_str) {
                    x_categories.push(x_str);
                }
            }
            if let Some(y_val) = row.get(y_category_idx) {
                let y_str = y_val.as_str().unwrap_or("").to_string();
                if !y_categories.contains(&y_str) {
                    y_categories.push(y_str);
                }
            }
        }

        // Build heatmap data: [[x_index, y_index, value], ...]
        let mut heatmap_data: Vec<Vec<Value>> = Vec::new();

        for row in &dataset.data {
            if let (Some(x_val), Some(y_val), Some(value)) =
                (row.get(x_idx), row.get(y_category_idx), row.get(value_idx)) {
                let x_str = x_val.as_str().unwrap_or("");
                let y_str = y_val.as_str().unwrap_or("");

                if let (Some(x_i), Some(y_i)) = (
                    x_categories.iter().position(|c| c == x_str),
                    y_categories.iter().position(|c| c == y_str),
                ) {
                    heatmap_data.push(vec![
                        json!(x_i),
                        json!(y_i),
                        value.clone(),
                    ]);
                }
            }
        }

        Ok((heatmap_data, x_categories, y_categories))
    }
}

impl WidgetConfigBuilder for HeatmapConfig {
    type StyleOptions = HeatmapStyleOptions;

    fn build_echarts_options(
        &self,
        dataset: &Dataset,
        mapping: &DataMapping,
        style: &Self::StyleOptions,
    ) -> Result<String, ConfigError> {
        // Transform dataset to heatmap format
        let (heatmap_data, x_categories, y_categories) =
            Self::dataset_to_heatmap_format(dataset, mapping)?;

        // Read colors from active DaisyUI theme
        let colors = ChartColors::from_daisyui_theme();

        // Color scale based on style
        let color_scale = if let (Some(min), Some(max)) = (&style.color_min, &style.color_max) {
            vec![min.clone(), max.clone()]
        } else {
            vec!["#313695".to_string(), "#a50026".to_string()]  // Blue to Red
        };

        // Build series
        let series = json!({
            "type": "heatmap",
            "data": heatmap_data,
            "label": if style.show_values {
                Some(json!({
                    "show": true,
                    "fontSize": 10,
                    "color": colors.label_high_contrast.clone(),
                }))
            } else {
                None::<Value>
            },
            "emphasis": {
                "itemStyle": {
                    "shadowBlur": 10,
                    "shadowColor": "rgba(0, 0, 0, 0.5)"
                }
            }
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
            "tooltip": {
                "position": "top",
                "backgroundColor": colors.background.clone(),
                "borderColor": colors.grid.clone(),
                "borderWidth": 1,
                "textStyle": {
                    "color": colors.label_high_contrast.clone(),
                    "fontSize": 12,
                },
            },
            "grid": {
                "height": "70%",
                "top": if style.title.is_some() { "15%" } else { "10%" }
            },
            "xAxis": {
                "type": "category",
                "data": x_categories,
                "splitArea": {
                    "show": true
                },
                "axisLabel": {
                    "color": colors.label.clone(),
                    "fontSize": 11
                }
            },
            "yAxis": {
                "type": "category",
                "data": y_categories,
                "splitArea": {
                    "show": true
                },
                "axisLabel": {
                    "color": colors.label.clone(),
                    "fontSize": 11
                }
            },
            "visualMap": {
                "min": 0,
                "max": 10,
                "calculable": true,
                "orient": "horizontal",
                "left": "center",
                "bottom": "5%",
                "inRange": {
                    "color": color_scale
                },
                "textStyle": {
                    "color": colors.label.clone()
                }
            },
            "series": series
        });

        if let Some(title_obj) = title_option {
            options["title"] = title_obj;
        }

        serde_json::to_string(&options)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))
    }

    fn validate_config(&self, mapping: &DataMapping) -> Result<(), ConfigError> {
        if mapping.x_axis.is_none() {
            return Err(ConfigError::MissingField("X-axis field is required for heatmaps".to_string()));
        }
        if mapping.category.is_none() {
            return Err(ConfigError::MissingField("Category field is required for Y-axis grouping".to_string()));
        }
        if mapping.y_axis.is_empty() {
            return Err(ConfigError::MissingField("Value field is required for heatmaps".to_string()));
        }
        Ok(())
    }

    fn required_fields(&self) -> Vec<FieldRequirement> {
        vec![
            FieldRequirement::Single {
                name: "X-Axis (Categories)",
                field_type: FieldType::Text,
                required: true,
            },
            FieldRequirement::Single {
                name: "Y-Axis (Categories)",
                field_type: FieldType::Text,
                required: true,
            },
            FieldRequirement::Single {
                name: "Value",
                field_type: FieldType::Numeric,
                required: true,
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        HeatmapStyleOptions::default()
    }
}
