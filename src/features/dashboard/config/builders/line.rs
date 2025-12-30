//! Line widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::LineStyleOptions;
use crate::features::dashboard::config::theme_colors::{ChartColors, lighten_color};
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::{dataset_to_echarts_format, AggregationFunction};
use serde_json::json;

/// Line widget configuration builder
#[derive(Clone, Copy)]
pub struct LineConfig;

impl WidgetConfigBuilder for LineConfig {
    type StyleOptions = LineStyleOptions;

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
            AggregationFunction::Sum, // Default aggregation
        ).map_err(|e| ConfigError::DataTransformationError(e.to_string()))?;

        // Read colors from active DaisyUI theme
        let colors = ChartColors::from_daisyui_theme();
        let color_palette = colors.to_palette();

        // SVG patterns for accessibility (area fill patterns for colorblind support)
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
            let is_first = i == 0;
            // Use colors from palette, cycling if more series than colors
            let series_color = color_palette[i % color_palette.len()].clone();
            let pattern = patterns[i % patterns.len()];

            json!({
                "type": "line",
                "smooth": style.smooth,
                "lineStyle": {
                    "width": style.line_width as u32,
                    "color": series_color.clone()
                },
                "itemStyle": {
                    "color": series_color.clone()
                },
                // Emphasis state (hover)
                "emphasis": {
                    "focus": "series",  // Highlight only this series
                    "blurScope": "coordinateSystem",  // Blur other series
                    "lineStyle": {
                        "width": (style.line_width + 2) as u32,  // Thicker line on hover
                        "color": lighten_color(&series_color, 0.2),
                    },
                    "itemStyle": {
                        "color": lighten_color(&series_color, 0.2),
                        "borderColor": colors.label_high_contrast.clone(),
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
                    "lineStyle": {
                        "opacity": 0.3,
                    },
                    "itemStyle": {
                        "opacity": 0.3,
                    }
                },
                "areaStyle": if style.area_fill && is_first {
                    if style.enable_patterns {
                        // Pattern fill for area
                        Some(json!({
                            "color": {
                                "type": "pattern",
                                "image": pattern,
                                "repeat": "repeat",
                                "color": format!("{}33", series_color),  // Semi-transparent
                            }
                        }))
                    } else {
                        // Gradient fill for area
                        Some(json!({
                            "color": {
                                "type": "linear",
                                "x": 0, "y": 0, "x2": 0, "y2": 1,
                                "colorStops": [
                                    { "offset": 0, "color": format!("{}33", series_color) },
                                    { "offset": 1, "color": format!("{}00", series_color) }
                                ]
                            }
                        }))
                    }
                } else {
                    None::<serde_json::Value>
                },
                "showSymbol": style.show_points,
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
            "type": "category",
            "boundaryGap": false,
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
            return Err(ConfigError::MissingField("X-axis field is required for line charts".to_string()));
        }
        if mapping.y_axis.is_empty() {
            return Err(ConfigError::MissingField("At least one Y-axis field is required for line charts".to_string()));
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
        LineStyleOptions::default()
    }
}
