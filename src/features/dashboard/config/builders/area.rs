//! Area widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::AreaStyleOptions;
use crate::features::dashboard::config::theme_colors::{ChartColors, lighten_color};
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::{dataset_to_echarts_format, AggregationFunction};
use serde_json::json;

/// Area widget configuration builder
#[derive(Clone, Copy)]
pub struct AreaConfig;

impl WidgetConfigBuilder for AreaConfig {
    type StyleOptions = AreaStyleOptions;

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

        // Build series array based on y_axis fields
        let num_series = mapping.y_axis.len();
        let series: Vec<serde_json::Value> = (0..num_series).map(|i| {
            let series_color = color_palette[i % color_palette.len()].clone();

            // Calculate opacity for area fill
            let opacity = style.opacity as f64 / 100.0;

            json!({
                "type": "line",
                "smooth": style.smooth,
                "stack": if style.stacked { "total" } else { "" },
                "lineStyle": {
                    "width": style.border_width as u32,
                    "color": series_color.clone()
                },
                "itemStyle": {
                    "color": series_color.clone()
                },
                // Area fill (always present for area charts)
                "areaStyle": {
                    "color": {
                        "type": "linear",
                        "x": 0, "y": 0, "x2": 0, "y2": 1,
                        "colorStops": [
                            { "offset": 0, "color": format!("{}{:02x}", series_color, (opacity * 255.0) as u8) },
                            { "offset": 1, "color": format!("{}00", series_color) }
                        ]
                    }
                },
                // Emphasis state (hover)
                "emphasis": {
                    "focus": "series",
                    "blurScope": "coordinateSystem",
                    "lineStyle": {
                        "width": (style.border_width + 2) as u32,
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
                // Blur state
                "blur": {
                    "lineStyle": {
                        "opacity": 0.2,
                    },
                    "areaStyle": {
                        "opacity": 0.1,
                    },
                    "itemStyle": {
                        "opacity": 0.2,
                    }
                },
                "showSymbol": style.show_points,
                "symbol": if style.show_points { "circle" } else { "none" },
                "symbolSize": style.point_size as u32,
                "animation": style.animation,
                "animationDuration": style.animation_duration,
                "animationEasing": "cubicOut"
            })
        }).collect();

        // Build xAxis
        let x_axis = json!({
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

        // Build yAxis
        let y_axis = json!({
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

        // Build complete ECharts options
        let options = json!({
            "dataset": {
                "source": echarts_data
            },
            "grid": {
                "left": "3%",
                "right": "4%",
                "bottom": "10%",
                "top": "15%",
                "containLabel": true
            },
            "tooltip": {
                "trigger": "axis",
                "backgroundColor": colors.background.clone(),
                "borderColor": colors.grid.clone(),
                "borderWidth": 1,
                "textStyle": {
                    "color": colors.label_high_contrast.clone(),
                    "fontSize": 12,
                    "fontWeight": "normal",
                },
                "extraCssText": "box-shadow: 0 4px 6px rgba(0,0,0,0.1); border-radius: 0.5rem;",
            },
            "legend": {
                "top": "0%",
                "textStyle": {
                    "color": colors.text,
                    "fontSize": 12
                }
            },
            "xAxis": x_axis,
            "yAxis": y_axis,
            "series": series
        });

        serde_json::to_string(&options)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))
    }

    fn validate_config(&self, mapping: &DataMapping) -> Result<(), ConfigError> {
        if mapping.x_axis.is_none() {
            return Err(ConfigError::MissingField("X-axis field is required for area charts".to_string()));
        }
        if mapping.y_axis.is_empty() {
            return Err(ConfigError::MissingField("At least one Y-axis field is required for area charts".to_string()));
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
                max_count: Some(5),
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        AreaStyleOptions::default()
    }
}
