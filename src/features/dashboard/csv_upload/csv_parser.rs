use crate::features::dashboard::{Field, Dataset, CsvError};
use crate::features::dashboard::csv_upload::{
    type_detector,
    schema_validator::{self, ValidationConfig},
};
use uuid::Uuid;
use serde_json::Value;

/// Parse CSV and create Dataset with Fields
///
/// # Arguments
/// * `csv_text` - The CSV file content as text
/// * `filename` - Name of the CSV file
/// * `file_size` - Size of the file in bytes
///
/// # Returns
/// A tuple of (Dataset, Vec<Field>) or a CsvError
pub fn parse_csv_to_dataset(
    csv_text: &str,
    filename: &str,
    file_size: u64,
) -> Result<(Dataset, Vec<Field>), CsvError> {
    // Parse CSV using csv crate
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(csv_text.as_bytes());

    // Get headers
    let headers = rdr
        .headers()
        .map_err(|e| CsvError::InvalidCsvFormat(format!("Failed to read headers: {}", e)))?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // Validate structure
    schema_validator::validate_structure(&headers, 0, &ValidationConfig::default())?;

    // Read all rows for type detection and data storage
    let all_rows: Vec<Vec<String>> = rdr
        .records()
        .map(|result| {
            result
                .map(|record| record.iter().map(|s| s.to_string()).collect())
                .map_err(|e| CsvError::ParseError {
                    row: 0,
                    message: e.to_string(),
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if all_rows.is_empty() {
        return Err(CsvError::EmptyFile);
    }

    // Sample first 100 rows for type detection
    let sample_rows = all_rows.iter().take(100).cloned().collect::<Vec<_>>();

    // Detect column types
    let field_types = type_detector::detect_types(headers.clone(), sample_rows)
        .map_err(CsvError::TypeInferenceFailed)?;

    // Create Field objects
    let fields: Vec<Field> = field_types
        .iter()
        .map(|(name, field_type)| Field {
            name: name.clone(),
            field_type: *field_type,
        })
        .collect();

    // Convert CSV data to JSON values
    let data: Vec<Vec<Value>> = all_rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|val| {
                    // Try to parse as number first
                    if let Ok(num) = val.parse::<f64>() {
                        Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)))
                    } else if let Ok(bool_val) = val.parse::<bool>() {
                        Value::Bool(bool_val)
                    } else {
                        Value::String(val.clone())
                    }
                })
                .collect()
        })
        .collect();

    // Create Dataset
    let dataset = Dataset {
        id: format!("ds_{}", Uuid::new_v4()),
        name: filename.to_string(),
        size: format_size(file_size),
        uploaded_at: "Today".to_string(),
        active: false, // Will be activated by context
        fields: fields.clone(),
        data,
    };

    Ok((dataset, fields))
}

/// Format file size for display
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_size(5 * 1024 * 1024), "5.0 MB");
    }
}
