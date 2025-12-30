//! KPI aggregation functions
//!
//! Provides functions to calculate aggregations (SUM, AVG, COUNT, etc.)
//! from dataset fields for KPI widgets.

use crate::features::dashboard::models::{Dataset, Field, FieldType, KpiAggregation};

/// Result of a KPI aggregation
#[derive(Clone, Debug, PartialEq)]
pub struct KpiValue {
    pub value: f64,
    pub formatted: String,
    pub aggregation: KpiAggregation,
}

/// Calculate KPI value from a dataset field
pub fn calculate_kpi(
    dataset: &Dataset,
    field_name: &str,
    aggregation: KpiAggregation,
) -> Option<KpiValue> {
    // Find field index
    let field_index = dataset.fields.iter().position(|f| f.name == field_name)?;
    let field = &dataset.fields[field_index];

    // Extract values from data rows
    let values: Vec<f64> = dataset
        .data
        .iter()
        .filter_map(|row| {
            row.get(field_index)
                .and_then(|val| val.as_f64())
                .or_else(|| {
                    // Try parsing as string first
                    row.get(field_index)
                        .and_then(|val| val.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                })
        })
        .collect();

    if values.is_empty() {
        return None;
    }

    // Calculate aggregation
    let (value, formatted) = match aggregation {
        KpiAggregation::Sum => {
            let sum: f64 = values.iter().sum();
            (sum, format_number(sum, field))
        }
        KpiAggregation::Average => {
            let avg: f64 = values.iter().sum::<f64>() / values.len() as f64;
            (avg, format_number(avg, field))
        }
        KpiAggregation::Count => {
            let count = values.len() as f64;
            (count, format!("{:.0}", count))
        }
        KpiAggregation::Min => {
            let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            (min, format_number(min, field))
        }
        KpiAggregation::Max => {
            let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            (max, format_number(max, field))
        }
        KpiAggregation::Last => {
            let last = *values.last()?;
            (last, format_number(last, field))
        }
        KpiAggregation::First => {
            let first = *values.first()?;
            (first, format_number(first, field))
        }
    };

    Some(KpiValue {
        value,
        formatted,
        aggregation,
    })
}

/// Format a number based on field type
fn format_number(value: f64, field: &Field) -> String {
    match field.field_type {
        FieldType::Numeric => {
            // Format with commas for thousands
            if value.fract() == 0.0 && value.abs() < 1_000_000.0 {
                // Integer under 1M
                format_with_commas(value)
            } else if value.abs() >= 1_000_000.0 {
                // Large numbers: use M/K suffix
                format_large_number(value)
            } else {
                // Decimal
                format!("{:.2}", value)
            }
        }
        FieldType::Date => format_date(value),
        _ => format!("{:.0}", value),
    }
}

/// Format number with thousand separators
fn format_with_commas(value: f64) -> String {
    let formatted = format!("{:.0}", value);
    let chars: Vec<char> = formatted.chars().collect();
    let mut result = String::new();
    let mut count = 0;

    for c in chars.iter() {
        if c == &'-' {
            result.push(*c);
        } else if c.is_ascii_digit() {
            if count > 0 && count % 3 == 0 {
                result.push(',');
            }
            result.push(*c);
            count += 1;
        }
    }

    result
}

/// Format large numbers with M/K suffixes
fn format_large_number(value: f64) -> String {
    let abs = value.abs();

    if abs >= 1_000_000_000.0 {
        format!("{:.1}B", value / 1_000_000_000.0)
    } else if abs >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else {
        format_with_commas(value)
    }
}

/// Format Unix timestamp as date (simple placeholder)
fn format_date(_value: f64) -> String {
    // This is a placeholder - proper date formatting would require chrono
    "Date".to_string()
}

/// Suggest KPI aggregations for a numeric field
pub fn suggest_aggregations(field: &Field) -> Vec<(KpiAggregation, String)> {
    match field.field_type {
        FieldType::Numeric => vec![
            (KpiAggregation::Sum, format!("Total {}", field.name)),
            (KpiAggregation::Average, format!("Average {}", field.name)),
            (KpiAggregation::Min, format!("Min {}", field.name)),
            (KpiAggregation::Max, format!("Max {}", field.name)),
        ],
        FieldType::Text => vec![
            (KpiAggregation::Count, format!("Count of {}", field.name)),
        ],
        _ => vec![],
    }
}

/// Analyze dataset and suggest KPIs for all numeric fields
pub fn analyze_dataset_for_kpis(dataset: &Dataset) -> Vec<(String, Vec<(KpiAggregation, String)>)> {
    dataset
        .fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Numeric | FieldType::Text))
        .map(|field| {
            let suggestions = suggest_aggregations(field);
            (field.name.clone(), suggestions)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let dataset = create_test_dataset();
        let result = calculate_kpi(&dataset, "revenue", KpiAggregation::Sum);

        assert!(result.is_some());
        assert_eq!(result.unwrap().value, 600.0); // 100 + 200 + 300
    }

    #[test]
    fn test_calculate_avg() {
        let dataset = create_test_dataset();
        let result = calculate_kpi(&dataset, "revenue", KpiAggregation::Average);

        assert!(result.is_some());
        assert_eq!(result.unwrap().value, 200.0); // (100 + 200 + 300) / 3
    }

    fn create_test_dataset() -> Dataset {
        Dataset {
            id: "test".into(),
            name: "Test".into(),
            size: "1KB".into(),
            uploaded_at: "2024-01-01".into(),
            fields: vec![
                Field {
                    name: "revenue".into(),
                    field_type: FieldType::Numeric,
                },
            ],
            active: true,
            data: vec![
                vec![Value::Number(serde_json::Number::from(100))],
                vec![Value::Number(serde_json::Number::from(200))],
                vec![Value::Number(serde_json::Number::from(300))],
            ],
        }
    }
}
