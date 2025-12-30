//! Pie widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::PieStyleOptions;
use crate::features::dashboard::config::theme_colors::ChartColors;
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::{dataset_to_echarts_format, AggregationFunction};
use serde_json::json;

/// Pie widget configuration builder
#[derive(Clone, Copy)]
pub struct PieConfig;

impl WidgetConfigBuilder for PieConfig {
    type StyleOptions = PieStyleOptions;

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

        // Use DaisyUI theme colors for pie slices instead of hardcoded palette
        // This ensures colors adapt to light/dark themes
        let color_palette = colors.to_palette();

        // Note: Pattern support for pie charts is reserved for future enhancement
        // Currently, pie charts use the extended 12-color palette for distinction
        // The enable_patterns flag is present in PieStyleOptions for consistency
        // but not yet implemented due to data structure complexity

        // Build series configuration
        let series = json!({
            "type": "pie",
            "radius": [style.inner_radius, "100%"],
            "center": ["50%", "50%"],
            "roseType": if style.rose_type { "area" } else { "radius" },
            "itemStyle": {
                "borderRadius": style.border_radius as u32,
                "borderColor": colors.background.clone(),
                "borderWidth": 2
            },
            "label": {
                "show": style.show_labels,
                "position": style.label_position,
                "color": colors.label_high_contrast.clone(),  // High-contrast labels
                "fontSize": 11,
                "formatter": "{b}: {d}%"
            },
            // Emphasis state (hover on slice)
            "emphasis": {
                "focus": "self",  // Highlight clicked slice
                "itemStyle": {
                    "shadowBlur": 10,
                    "shadowOffsetX": 0,
                    "shadowColor": "rgba(0, 0, 0, 0.5)",
                    "borderColor": colors.label_high_contrast.clone(),
                    "borderWidth": 3,
                },
                "label": {
                    "show": true,
                    "fontSize": 14,
                    "fontWeight": "bold",
                    "color": colors.label_high_contrast.clone(),
                },
                "scaleSize": 10,  // Slightly enlarge on hover
            },
            // Blur state (when another slice is hovered)
            "blur": {
                "itemStyle": {
                    "opacity": 0.4,
                },
                "label": {
                    "opacity": 0.4,
                }
            },
            "labelLine": {
                "show": style.show_labels && style.label_position == "outside",
                "lineStyle": {
                    "color": colors.grid.clone()
                }
            },
            "animation": style.animation,
            "animationDuration": style.animation_duration as i32,
            "animationEasing": "cubicOut"
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
            "color": color_palette,
            "dataset": {
                "source": echarts_data
            },
            "tooltip": {
                "trigger": "item",
                "backgroundColor": colors.background.clone(),
                "borderColor": colors.grid.clone(),
                "borderWidth": 1,
                "textStyle": {
                    "color": colors.label_high_contrast.clone(),  // High-contrast text
                    "fontSize": 12,
                    "fontWeight": "normal",
                },
                "extraCssText": "box-shadow: 0 4px 6px rgba(0,0,0,0.1); border-radius: 0.5rem;",
                "formatter": "{a} <br/>{b}: {c} ({d}%)"
            },
            "legend": {
                "top": if style.title.is_some() { "8%" } else { "0%" },
                "textStyle": {
                    "color": colors.text,
                    "fontSize": 12
                },
                "type": "scroll"
            },
            "series": [series]
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
            return Err(ConfigError::MissingField("At least one measure field is required for pie charts".to_string()));
        }
        Ok(())
    }

    fn required_fields(&self) -> Vec<FieldRequirement> {
        vec![
            FieldRequirement::Single {
                name: "Labels",
                field_type: FieldType::Text,
                required: true,
            },
            FieldRequirement::Multiple {
                name: "Values",
                field_types: vec![FieldType::Numeric],
                min_count: 1,
                max_count: Some(1),
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        PieStyleOptions::default()
    }
}
