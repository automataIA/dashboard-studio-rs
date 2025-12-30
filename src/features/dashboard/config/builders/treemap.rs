//! Treemap widget config builder

use crate::features::dashboard::config::traits::{WidgetConfigBuilder, FieldRequirement, ConfigError};
use crate::features::dashboard::config::style::TreemapStyleOptions;
use crate::features::dashboard::config::theme_colors::ChartColors;
use crate::features::dashboard::models::{Dataset, DataMapping, FieldType};
use crate::features::dashboard::data::transform::find_field_index;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Treemap widget configuration builder
#[derive(Clone, Copy)]
pub struct TreemapConfig;

impl TreemapConfig {
    /// Transform dataset to treemap hierarchical format
    fn dataset_to_treemap_format(
        dataset: &Dataset,
        mapping: &DataMapping,
    ) -> Result<Value, ConfigError> {
        if mapping.hierarchy.is_empty() {
            return Err(ConfigError::MissingField("Hierarchy fields are required for treemap".to_string()));
        }

        // Find field indexes for hierarchy levels
        let hierarchy_indexes: Vec<usize> = mapping.hierarchy.iter()
            .map(|field_name| find_field_index(&dataset.fields, &Some(field_name.clone())))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?;

        // Value field (optional - use count if not specified)
        let value_idx = if !mapping.y_axis.is_empty() {
            Some(find_field_index(&dataset.fields, &Some(mapping.y_axis[0].clone()))
                .map_err(|e| ConfigError::DataTransformationError(e.to_string()))?)
        } else {
            None
        };

        // Build hierarchical structure
        let mut root_children: HashMap<String, HashMap<String, Vec<Value>>> = HashMap::new();

        for row in &dataset.data {
            // Extract hierarchy values
            let hierarchy_values: Vec<String> = hierarchy_indexes.iter()
                .filter_map(|&idx| {
                    row.get(idx).and_then(|v| v.as_str()).map(|s| s.to_string())
                })
                .collect();

            if hierarchy_values.len() < 2 {
                continue;  // Need at least 2 levels
            }

            // Get value (or default to 1 for counting)
            let value = if let Some(v_idx) = value_idx {
                row.get(v_idx)
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0)
            } else {
                1.0
            };

            // Build hierarchy: level0 -> level1 -> level2...
            let level0 = &hierarchy_values[0];
            let level1 = &hierarchy_values[1];

            let level0_map = root_children.entry(level0.clone()).or_insert_with(HashMap::new);
            let level1_items = level0_map.entry(level1.clone()).or_insert_with(Vec::new);

            // Add leaf node
            if hierarchy_values.len() > 2 {
                level1_items.push(json!({
                    "name": hierarchy_values[2],
                    "value": value
                }));
            } else {
                level1_items.push(json!({
                    "name": level1,
                    "value": value
                }));
            }
        }

        // Convert to ECharts tree structure
        let children: Vec<Value> = root_children.into_iter().map(|(level0_name, level1_map)| {
            let level1_children: Vec<Value> = level1_map.into_iter().map(|(level1_name, items)| {
                json!({
                    "name": level1_name,
                    "children": items
                })
            }).collect();

            json!({
                "name": level0_name,
                "children": level1_children
            })
        }).collect();

        Ok(json!({
            "name": "Root",
            "children": children
        }))
    }
}

impl WidgetConfigBuilder for TreemapConfig {
    type StyleOptions = TreemapStyleOptions;

    fn build_echarts_options(
        &self,
        dataset: &Dataset,
        mapping: &DataMapping,
        style: &Self::StyleOptions,
    ) -> Result<String, ConfigError> {
        // Transform dataset to treemap format
        let tree_data = Self::dataset_to_treemap_format(dataset, mapping)?;

        // Read colors from active DaisyUI theme
        let colors = ChartColors::from_daisyui_theme();
        let _color_palette = colors.to_palette();

        // Build series
        let series = json!({
            "type": "treemap",
            "data": [tree_data],
            "leafDepth": style.leaf_depth as u32,
            "roam": false,
            "breadcrumb": {
                "show": style.show_breadcrumb,
                "itemStyle": {
                    "color": colors.grid.clone(),
                    "textStyle": {
                        "color": colors.label.clone()
                    }
                }
            },
            "label": {
                "show": true,
                "formatter": "{b}",
                "fontSize": style.label_size as u32,
                "color": "#fff",  // White text on colored rectangles
            },
            "upperLabel": {
                "show": true,
                "height": 30,
                "color": "#fff"
            },
            "itemStyle": {
                "borderColor": colors.background.clone(),
                "borderWidth": 2,
                "gapWidth": 2
            },
            "levels": [
                {
                    "itemStyle": {
                        "borderColor": colors.grid.clone(),
                        "borderWidth": 4,
                        "gapWidth": 4
                    }
                },
                {
                    "colorSaturation": [0.35, 0.5],
                    "itemStyle": {
                        "borderWidth": 3,
                        "gapWidth": 3,
                        "borderColorSaturation": 0.6
                    }
                },
                {
                    "colorSaturation": [0.25, 0.4],
                    "itemStyle": {
                        "borderWidth": 2,
                        "gapWidth": 2,
                        "borderColorSaturation": 0.7
                    }
                }
            ],
            "emphasis": {
                "itemStyle": {
                    "shadowBlur": 20,
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
                "trigger": "item",
                "backgroundColor": colors.background.clone(),
                "borderColor": colors.grid.clone(),
                "borderWidth": 1,
                "textStyle": {
                    "color": colors.label_high_contrast.clone(),
                    "fontSize": 12,
                },
                "formatter": "{b}: {c}"
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
        if mapping.hierarchy.len() < 2 {
            return Err(ConfigError::MissingField("At least 2 hierarchy levels are required for treemaps".to_string()));
        }
        Ok(())
    }

    fn required_fields(&self) -> Vec<FieldRequirement> {
        vec![
            FieldRequirement::Multiple {
                name: "Hierarchy Levels",
                field_types: vec![FieldType::Text],
                min_count: 2,
                max_count: Some(4),
            },
            FieldRequirement::Single {
                name: "Value (Optional)",
                field_type: FieldType::Numeric,
                required: false,
            },
        ]
    }

    fn default_style(&self) -> Self::StyleOptions {
        TreemapStyleOptions::default()
    }
}
