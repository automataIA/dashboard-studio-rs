use crate::features::dashboard::FieldType;

/// Sample size for type detection (first N rows)
const SAMPLE_SIZE: usize = 100;

/// Threshold for type classification (90% of values must match)
const TYPE_THRESHOLD: f64 = 0.9;

/// Detect field type from sample data
///
/// # Arguments
/// * `column_name` - Name of the column being analyzed
/// * `values` - Sample values from the column
///
/// # Returns
/// Inferred field type
pub fn detect_column_type(column_name: &str, values: &[String]) -> Result<FieldType, String> {
    if values.is_empty() {
        return Err(format!("Column '{}' has no data", column_name));
    }

    // Filter out null/empty values
    let non_null_values: Vec<&String> = values
        .iter()
        .filter(|v| {
            let trimmed = v.trim();
            !trimmed.is_empty()
                && trimmed.to_lowercase() != "null"
                && trimmed.to_lowercase() != "n/a"
                && trimmed.to_lowercase() != "na"
        })
        .collect();

    // If all null/empty, default to Text
    if non_null_values.is_empty() {
        return Ok(FieldType::Text);
    }

    // Check for Boolean (true/false, yes/no, 1/0)
    let boolean_count = non_null_values
        .iter()
        .filter(|v| is_boolean(v))
        .count();

    if boolean_count as f64 / non_null_values.len() as f64 >= TYPE_THRESHOLD {
        return Ok(FieldType::Boolean);
    }

    // Check for Numeric
    let numeric_count = non_null_values
        .iter()
        .filter(|v| is_numeric(v))
        .count();

    if numeric_count as f64 / non_null_values.len() as f64 >= TYPE_THRESHOLD {
        return Ok(FieldType::Numeric);
    }

    // Check for Date
    let date_count = non_null_values
        .iter()
        .filter(|v| is_date(v))
        .count();

    if date_count as f64 / non_null_values.len() as f64 >= TYPE_THRESHOLD {
        return Ok(FieldType::Date);
    }

    // Default to Text
    Ok(FieldType::Text)
}

/// Check if a value represents a boolean
fn is_boolean(value: &str) -> bool {
    let lower = value.trim().to_lowercase();
    matches!(
        lower.as_str(),
        "true" | "false" | "yes" | "no" | "1" | "0" | "y" | "n"
    )
}

/// Check if a value represents a number
fn is_numeric(value: &str) -> bool {
    value.trim().parse::<f64>().is_ok()
}

/// Check if a value represents a date
fn is_date(value: &str) -> bool {
    let trimmed = value.trim();

    // Try ISO 8601 format
    if chrono::DateTime::parse_from_rfc3339(trimmed).is_ok() {
        return true;
    }

    // Try common date formats
    let formats = [
        "%Y-%m-%d",
        "%m/%d/%Y",
        "%d/%m/%Y",
        "%Y-%m-%d %H:%M:%S",
        "%m/%d/%Y %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
        "%Y/%m/%d",
        "%m-%d-%Y",
        "%d-%m-%Y",
    ];

    for format in formats {
        if chrono::NaiveDateTime::parse_from_str(trimmed, format).is_ok()
            || chrono::NaiveDate::parse_from_str(trimmed, format).is_ok()
        {
            return true;
        }
    }

    false
}

/// Detect types for all columns
///
/// # Arguments
/// * `headers` - Column headers
/// * `rows` - Sample rows from the CSV
///
/// # Returns
/// Vector of (column_name, field_type) tuples
pub fn detect_types(
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
) -> Result<Vec<(String, FieldType)>, String> {
    if headers.is_empty() {
        return Err("No headers found".to_string());
    }

    let mut results = Vec::new();

    for (idx, header) in headers.iter().enumerate() {
        // Collect column values from sample rows
        let column_values: Vec<String> = rows
            .iter()
            .filter_map(|row| row.get(idx).cloned())
            .take(SAMPLE_SIZE)
            .collect();

        // Detect type for this column
        let field_type = detect_column_type(header, &column_values)?;
        results.push((header.clone(), field_type));
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_boolean() {
        assert!(is_boolean("true"));
        assert!(is_boolean("false"));
        assert!(is_boolean("yes"));
        assert!(is_boolean("no"));
        assert!(is_boolean("1"));
        assert!(is_boolean("0"));
        assert!(!is_boolean("maybe"));
    }

    #[test]
    fn test_detect_numeric() {
        assert!(is_numeric("123"));
        assert!(is_numeric("123.45"));
        assert!(is_numeric("-123"));
        assert!(is_numeric("0.45"));
        assert!(!is_numeric("abc"));
        assert!(!is_numeric("12a"));
    }

    #[test]
    fn test_detect_column_type_boolean() {
        let values = vec![
            "true".to_string(),
            "false".to_string(),
            "yes".to_string(),
            "no".to_string(),
            "true".to_string(),
            "false".to_string(),
            "yes".to_string(),
            "no".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];

        let result = detect_column_type("test_col", &values).unwrap();
        assert_eq!(result, FieldType::Boolean);
    }

    #[test]
    fn test_detect_column_type_numeric() {
        let values = vec![
            "123.45".to_string(),
            "67.89".to_string(),
            "100".to_string(),
            "200.5".to_string(),
            "300.75".to_string(),
        ];

        let result = detect_column_type("revenue", &values).unwrap();
        assert_eq!(result, FieldType::Numeric);
    }

    #[test]
    fn test_detect_column_type_text() {
        let values = vec![
            "Electronics".to_string(),
            "Clothing".to_string(),
            "Books".to_string(),
            "Toys".to_string(),
            "Sports".to_string(),
        ];

        let result = detect_column_type("category", &values).unwrap();
        assert_eq!(result, FieldType::Text);
    }
}
