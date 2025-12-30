//! Bar widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::BarStyleOptions;
use crate::features::dashboard::config::theme_colors::{ChartColors, lighten_color};
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::{dataset_to_echarts_format, AggregationFunction};
use serde_json::json;

/// Bar widget configuration builder
#[derive(Clone, Copy)]
pub struct BarConfig;

impl WidgetConfigBuilder for BarConfig {
    type StyleOptions = BarStyleOptions;

    fn build_echarts_options(
        &self,
        dataset: &Dataset,
        mapping: &DataMapping,
        style: &Self::StyleOptions,
    ) -> Result<String, ConfigError> {
        // Transform dataset to ECharts format
        let echarts_data = dataset_to_echarts_format(
            dataset,
            mapping,
            AggregationFunction::Sum,
        ).map_err(|e| ConfigError::DataTransformationError(e.to_string()))?;

        // Read colors from active DaisyUI theme
        let colors = ChartColors::from_daisyui_theme();
        let color_palette = colors.to_palette();

        // Determine axis orientation
        let x_axis_type = if style.horizontal { "value" } else { "category" };
        let y_axis_type = if style.horizontal { "category" } else { "value" };

        // SVG patterns for accessibility (colorblind support)
        let patterns = [
            "path://M0,0 L10,10 M10,0 L0,10",    // 1. Diagonal cross-hatch
            "path://M0,5 L10,5",                   // 2. Horizontal lines
            "path://M5,0 L5,10",                   // 3. Vertical lines
            "path://M0,0 L10,10",                  // 4. Diagonal right
            "path://M0,10 L10,0",                  // 5. Diagonal left
            "path://M0,0 L5,5 L10,0 L5,10 Z",     // 6. Diamond
            "path://M5,0 L10,5 L5,10 L0,5 Z",     // 7. Square
            "circle",                               // 8. Dots
        ];

        // Build series array based on y_axis fields
        let num_series = mapping.y_axis.len();
        let series: Vec<serde_json::Value> = (0..num_series).map(|i| {
            // Use colors from palette, cycling if more series than colors
            let series_color = color_palette[i % color_palette.len()].clone();
            let pattern = patterns[i % patterns.len()];

            // Color configuration with optional pattern
            let color_config = if style.enable_patterns {
                json!({
                    "type": "pattern",
                    "image": pattern,
                    "repeat": "repeat",
                    "color": series_color.clone(),
                })
            } else {
                json!(series_color.clone())
            };

            json!({
                "type": "bar",
                "stack": if style.stacked { serde_json::json!("total") } else { serde_json::Value::Null },
                "barMinWidth": if style.horizontal { Some(0) } else { None::<i32> },
                "barMaxWidth": if style.horizontal { None::<i32> } else { Some(style.bar_width as i32) },
                "itemStyle": {
                    "color": color_config,
                    "borderRadius": if style.horizontal {
                        [0, style.border_radius as u32, style.border_radius as u32, 0]
                    } else {
                        [0, 0, style.border_radius as u32, style.border_radius as u32]
                    }
                },
                // Emphasis state (hover)
                "emphasis": {
                    "focus": "series",  // Highlight only this series
                    "blurScope": "coordinateSystem",  // Blur other series
                    "itemStyle": {
                        "color": lighten_color(&series_color, 0.2),  // 20% lighter on hover
                        "borderColor": colors.label_high_contrast.clone(),  // High-contrast border
                        "borderWidth": 2,
                        "shadowBlur": 10,
                        "shadowColor": "rgba(0, 0, 0, 0.3)",
                    },
                    "label": {
                        "show": true,
                        "fontSize": 13,
                        "fontWeight": "bold",
                        "color": colors.label_high_contrast.clone(),
                    }
                },
                // Blur state (when another series is hovered)
                "blur": {
                    "itemStyle": {
                        "opacity": 0.3,  // Fade out non-focused series
                    }
                },
                "label": if style.show_labels {
                    Some(json!({
                        "show": true,
                        "position": "top",
                        "fontSize": 11,
                        "color": colors.label_high_contrast.clone(),
                    }))
                } else {
                    None::<serde_json::Value>
                },
                "animation": true,
                "animationDuration": 1000,
                "animationEasing": "cubicOut"
            })
        }).collect();

        // Build xAxis with optional title
        let mut x_axis = json!({
            "type": x_axis_type,
            "boundaryGap": !style.horizontal,
            "axisLine": {
                "lineStyle": { "color": colors.grid }
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

        // Build yAxis with optional title
        let mut y_axis = json!({
            "type": y_axis_type,
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
        let grid_top = if style.title.is_some() { "20%" } else { "15%" };

        // Build complete ECharts options
        let mut options = json!({
            "dataset": {
                "source": echarts_data
            },
            "grid": {
                "left": if style.y_axis_title.is_some() { "8%" } else { "3%" },
                "right": "4%",
                "bottom": if style.x_axis_title.is_some() { "15%" } else { "10%" },
                "top": grid_top,
                "containLabel": true
            },
            "tooltip": {
                "trigger": "axis",
                "backgroundColor": colors.background.clone(),
                "borderColor": colors.grid.clone(),
                "borderWidth": 1,
                "textStyle": {
                    "color": colors.label_high_contrast.clone(),  // High-contrast text
                    "fontSize": 12,
                    "fontWeight": "normal",
                },
                "extraCssText": "box-shadow: 0 4px 6px rgba(0,0,0,0.1); border-radius: 0.5rem;",
            },
            "legend": {
                "top": if style.title.is_some() { "8%" } else { "0%" },
                "textStyle": {
                    "color": colors.text,
                    "fontSize": 12
                }
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
            return Err(ConfigError::MissingField("X-axis field is required for bar charts".to_string()));
        }
        if mapping.y_axis.is_empty() {
            return Err(ConfigError::MissingField("At least one Y-axis field is required for bar charts".to_string()));
        }
        Ok(())
    }

    fn required_fields(&self) -> Vec<FieldRequirement> {
        vec![
            FieldRequirement::Single {
                name: "X-Axis",
                field_type: FieldType::Text,
                required: true,
            },
            FieldRequirement::Multiple {
                name: "Y-Axis",
                field_types: vec![FieldType::Numeric],
                min_count: 1,
                max_count: Some(3),
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        BarStyleOptions::default()
    }
}
