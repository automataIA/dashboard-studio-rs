//! Data transformation utilities

use super::aggregation::AggregationFunction;
use crate::features::dashboard::models::{DataMapping, Dataset, Field, FieldType};
use serde_json::Value;
use std::collections::HashMap;

/// Error types for data transformation
#[derive(Clone, Debug, PartialEq)]
pub enum TransformError {
    /// Field not found in dataset
    FieldNotFound(String),

    /// Invalid field type for operation
    #[allow(dead_code)]
    InvalidFieldType {
        field: String,
        expected: FieldType,
        found: FieldType,
    },

    /// Data transformation failed
    TransformationFailed(String),
}

impl std::fmt::Display for TransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldNotFound(field) => write!(f, "Field not found: {}", field),
            Self::InvalidFieldType {
                field,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Field '{}' has invalid type: expected {:?}, found {:?}",
                    field, expected, found
                )
            }
            Self::TransformationFailed(msg) => write!(f, "Transformation failed: {}", msg),
        }
    }
}

impl std::error::Error for TransformError {}

/// Find the index of a field by name in the dataset
pub fn find_field_index(
    fields: &[Field],
    field_name: &Option<String>,
) -> Result<usize, TransformError> {
    match field_name {
        Some(name) => fields
            .iter()
            .position(|f| f.name == *name)
            .ok_or_else(|| TransformError::FieldNotFound(name.clone())),
        None => Err(TransformError::FieldNotFound(
            "No field specified".to_string(),
        )),
    }
}

/// Find multiple field indexes by name
pub fn find_field_indexes(
    fields: &[Field],
    field_names: &[String],
) -> Result<Vec<usize>, TransformError> {
    field_names
        .iter()
        .map(|name| {
            fields
                .iter()
                .position(|f| &f.name == name)
                .ok_or_else(|| TransformError::FieldNotFound(name.clone()))
        })
        .collect()
}

/// Transform dataset data to ECharts 2D array format
///
/// # Arguments
/// * `dataset` - The dataset with fields and raw data
/// * `mapping` - Field mapping configuration
/// * `agg_fn` - Aggregation function to apply
///
/// # Returns
/// ECharts-compatible 2D array (header row + data rows)
///
/// # Example
/// For Line chart with x_axis="Month" and y_axis="Sales":
/// ```text
/// [
///   ["Month", "Sales"],
///   ["Jan", 1000],
///   ["Feb", 1500],
///   ...
/// ]
/// ```
pub fn dataset_to_echarts_format(
    dataset: &Dataset,
    mapping: &DataMapping,
    agg_fn: AggregationFunction,
) -> Result<Vec<Vec<Value>>, TransformError> {
    // Find field indexes
    let x_idx = find_field_index(&dataset.fields, &mapping.x_axis)?;
    let y_idxs = find_field_indexes(&dataset.fields, &mapping.y_axis)?;

    if y_idxs.is_empty() {
        return Err(TransformError::TransformationFailed(
            "No Y-axis fields specified".to_string(),
        ));
    }

    // Build header row
    let mut header = vec![Value::String(dataset.fields[x_idx].name.clone())];
    header.extend(
        y_idxs
            .iter()
            .map(|&idx| Value::String(dataset.fields[idx].name.clone())),
    );

    // Transform data rows
    let mut echarts_data = vec![header];

    // Group by x_axis value and aggregate y_axis values
    let mut grouped_data: HashMap<String, Vec<Vec<Value>>> = HashMap::new();

    for row in &dataset.data {
        if let Some(x_val) = row.get(x_idx) {
            let x_key = serde_json::to_string(x_val).unwrap_or_default();

            grouped_data.entry(x_key).or_default().push(row.clone());
        }
    }

    // Aggregate and build final data
    for (x_key, rows) in grouped_data {
        let x_val: Value = serde_json::from_str(&x_key).unwrap_or(Value::Null);

        let mut data_row = vec![x_val];

        // Aggregate each y field
        for &y_idx in &y_idxs {
            let values: Vec<f64> = rows
                .iter()
                .filter_map(|row| row.get(y_idx))
                .filter_map(|v| v.as_f64())
                .collect();

            let aggregated = if values.is_empty() {
                0.0
            } else {
                apply_aggregation(&values, agg_fn)
            };

            data_row.push(Value::Number(
                serde_json::Number::from_f64(aggregated).unwrap_or(serde_json::Number::from(0)),
            ));
        }

        echarts_data.push(data_row);
    }

    // Sort by x-axis value (for proper ordering)
    echarts_data.sort_by(|a, b| {
        if a.len() > 1 && b.len() > 1 {
            // Compare x-axis values (index 0)
            match (&a[0], &b[0]) {
                (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                (Value::Number(n1), Value::Number(n2)) => n1
                    .as_f64()
                    .partial_cmp(&n2.as_f64())
                    .unwrap_or(std::cmp::Ordering::Equal),
                _ => std::cmp::Ordering::Equal,
            }
        } else {
            std::cmp::Ordering::Equal
        }
    });

    Ok(echarts_data)
}

/// Apply aggregation function to values (internal helper)
fn apply_aggregation(values: &[f64], agg_fn: AggregationFunction) -> f64 {
    match agg_fn {
        AggregationFunction::Sum => values.iter().sum(),
        AggregationFunction::Avg => {
            if values.is_empty() {
                0.0
            } else {
                values.iter().sum::<f64>() / values.len() as f64
            }
        }
        AggregationFunction::Count => values.len() as f64,
        AggregationFunction::Min => values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
        AggregationFunction::Max => values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
        AggregationFunction::Median => {
            let mut sorted = values.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let len = sorted.len();
            if len == 0 {
                0.0
            } else if len.is_multiple_of(2) {
                (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
            } else {
                sorted[len / 2]
            }
        }
        AggregationFunction::None => values.first().copied().unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::dashboard::models::Field;

    #[test]
    fn test_find_field_index() {
        let fields = vec![
            Field {
                name: "Name".to_string(),
                field_type: FieldType::Text,
            },
            Field {
                name: "Value".to_string(),
                field_type: FieldType::Numeric,
            },
        ];

        let result = find_field_index(&fields, &Some("Value".to_string()));
        assert_eq!(result, Ok(1));

        let result = find_field_index(&fields, &Some("Missing".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_dataset_to_echarts_format() {
        let fields = vec![
            Field {
                name: "Month".to_string(),
                field_type: FieldType::Text,
            },
            Field {
                name: "Sales".to_string(),
                field_type: FieldType::Numeric,
            },
        ];

        let _dataset = Dataset {
            id: "test".to_string(),
            name: "Test".to_string(),
            size: "0 MB".to_string(),
            uploaded_at: "Today".to_string(),
            fields: fields.clone(),
            active: true,
            data: Vec::new(),
        };

        // Note: This test assumes dataset.data exists, but the struct doesn't have it yet
        // We'll need to add the `data` field to the Dataset struct
    }
}
