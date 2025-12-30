//! Data aggregation utilities

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Aggregation function for measure fields
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationFunction {
    Sum,
    Avg,
    Count,
    Min,
    Max,
    Median,
    None, // No aggregation (use raw values)
}

impl AggregationFunction {
    /// Get all aggregation functions
    #[allow(dead_code)]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Sum,
            Self::Avg,
            Self::Count,
            Self::Min,
            Self::Max,
            Self::Median,
            Self::None,
        ]
    }

    /// Get display name
    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Sum => "SUM",
            Self::Avg => "AVG",
            Self::Count => "COUNT",
            Self::Min => "MIN",
            Self::Max => "MAX",
            Self::Median => "MEDIAN",
            Self::None => "NONE",
        }
    }
}

impl Default for AggregationFunction {
    fn default() -> Self {
        Self::Sum
    }
}

/// Aggregate data for a set of measure fields
///
/// # Arguments
/// * `data` - Raw data rows (each row is a Vec of JSON values)
/// * `field_indexes` - Indexes of the measure fields to aggregate
/// * `agg_fn` - Aggregation function to apply
///
/// # Returns
/// Vector of aggregated values (one per field)
///
/// # Example
/// ```rust
/// let data = vec![
///     vec![json!("A"), json!(10), json!(20)],
///     vec![json!("B"), json!(15), json!(25)],
/// ];
/// let result = aggregate_data(&data, vec![1, 2], AggregationFunction::Sum);
/// assert_eq!(result, vec![25.0, 45.0]);
/// ```
#[allow(dead_code)]
pub fn aggregate_data(
    data: &[Vec<Value>],
    field_indexes: Vec<usize>,
    agg_fn: AggregationFunction,
) -> Vec<f64> {
    if field_indexes.is_empty() || data.is_empty() {
        return Vec::new();
    }

    field_indexes
        .iter()
        .map(|&idx| {
            let values: Vec<f64> = data
                .iter()
                .filter_map(|row| row.get(idx))
                .filter_map(|v| v.as_f64())
                .collect();

            if values.is_empty() {
                return 0.0;
            }

            apply_aggregation(&values, agg_fn)
        })
        .collect()
}

/// Apply aggregation function to a set of values
#[allow(dead_code)]
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
        AggregationFunction::Min => {
            values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
        }
        AggregationFunction::Max => {
            values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
        }
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
        AggregationFunction::None => {
            // Return first value or 0 if empty
            values.first().copied().unwrap_or(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sum_aggregation() {
        let data = vec![
            vec![json!("A"), json!(10), json!(20)],
            vec![json!("B"), json!(15), json!(25)],
            vec![json!("C"), json!(20), json!(30)],
        ];

        let result = aggregate_data(&data, vec![1, 2], AggregationFunction::Sum);
        assert_eq!(result, vec![45.0, 75.0]);
    }

    #[test]
    fn test_avg_aggregation() {
        let data = vec![
            vec![json!(10)],
            vec![json!(20)],
            vec![json!(30)],
        ];

        let result = aggregate_data(&data, vec![0], AggregationFunction::Avg);
        assert_eq!(result, vec![20.0]);
    }

    #[test]
    fn test_count_aggregation() {
        let data = vec![
            vec![json!(10)],
            vec![json!(20)],
            vec![json!(30)],
        ];

        let result = aggregate_data(&data, vec![0], AggregationFunction::Count);
        assert_eq!(result, vec![3.0]);
    }

    #[test]
    fn test_min_max_aggregation() {
        let data = vec![
            vec![json!(10)],
            vec![json!(5)],
            vec![json!(20)],
        ];

        let min_result = aggregate_data(&data, vec![0], AggregationFunction::Min);
        let max_result = aggregate_data(&data, vec![0], AggregationFunction::Max);

        assert_eq!(min_result, vec![5.0]);
        assert_eq!(max_result, vec![20.0]);
    }

    #[test]
    fn test_median_aggregation() {
        let data = vec![
            vec![json!(10)],
            vec![json!(20)],
            vec![json!(30)],
        ];

        let result = aggregate_data(&data, vec![0], AggregationFunction::Median);
        assert_eq!(result, vec![20.0]);
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<Vec<Value>> = vec![];
        let result = aggregate_data(&data, vec![0], AggregationFunction::Sum);
        assert_eq!(result, Vec::<f64>::new());
    }
}
