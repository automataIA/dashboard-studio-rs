use crate::features::dashboard::CsvError;
use std::collections::HashSet;

/// Validation configuration for CSV uploads
#[derive(Clone, Debug)]
pub struct ValidationConfig {
    /// Maximum file size in megabytes
    pub max_file_size_mb: u64,
    /// Minimum number of columns required
    pub min_columns: usize,
    /// Maximum number of columns allowed
    pub max_columns: usize,
    /// Whether headers are required
    pub require_headers: bool,
    /// Allowed CSV delimiters
    #[allow(dead_code)]
    pub allowed_delimiters: Vec<char>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_file_size_mb: 100, // 100MB max
            min_columns: 1,
            max_columns: 1000,
            require_headers: true,
            allowed_delimiters: vec![',', '\t', ';', '|'],
        }
    }
}

/// Validate CSV file before parsing
///
/// # Arguments
/// * `file_size` - Size of the file in bytes
/// * `filename` - Name of the file
/// * `config` - Validation configuration
///
/// # Returns
/// Ok(()) if valid, Err(CsvError) if validation fails
pub fn validate_file(
    file_size: u64,
    filename: &str,
    config: &ValidationConfig,
) -> Result<(), CsvError> {
    // Check file size
    let size_mb = file_size / (1024 * 1024);
    if size_mb > config.max_file_size_mb {
        return Err(CsvError::FileTooLarge {
            max_mb: config.max_file_size_mb,
            actual_mb: size_mb,
        });
    }

    // Check file extension (warning only, not an error)
    let lower_name = filename.to_lowercase();
    if !lower_name.ends_with(".csv")
        && !lower_name.ends_with(".txt")
        && !lower_name.ends_with(".tsv")
    {
        log::warn!(
            "File '{}' doesn't have a standard CSV extension (.csv, .txt, .tsv)",
            filename
        );
    }

    Ok(())
}

/// Validate CSV structure after parsing headers
///
/// # Arguments
/// * `headers` - Column headers from CSV
/// * `row_count` - Number of data rows (for logging)
/// * `config` - Validation configuration
///
/// # Returns
/// Ok(()) if valid, Err(CsvError) if validation fails
pub fn validate_structure(
    headers: &[String],
    _row_count: usize,
    config: &ValidationConfig,
) -> Result<(), CsvError> {
    // Check headers exist
    if config.require_headers && headers.is_empty() {
        return Err(CsvError::NoHeaders);
    }

    // Check column count
    if headers.len() < config.min_columns {
        return Err(CsvError::InvalidCsvFormat(format!(
            "Too few columns: {} (minimum: {})",
            headers.len(),
            config.min_columns
        )));
    }

    if headers.len() > config.max_columns {
        return Err(CsvError::InvalidCsvFormat(format!(
            "Too many columns: {} (maximum: {})",
            headers.len(),
            config.max_columns
        )));
    }

    // Check for empty headers
    for (idx, header) in headers.iter().enumerate() {
        if header.trim().is_empty() {
            return Err(CsvError::InvalidCsvFormat(format!(
                "Empty header at column {}",
                idx + 1
            )));
        }
    }

    // Check for duplicate headers
    let mut seen = HashSet::new();
    for header in headers {
        if !seen.insert(header.clone()) {
            return Err(CsvError::InvalidCsvFormat(format!(
                "Duplicate header: '{}'",
                header
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_file_too_large() {
        let config = ValidationConfig {
            max_file_size_mb: 10,
            ..Default::default()
        };

        let result = validate_file(15 * 1024 * 1024, "test.csv", &config);
        assert!(result.is_err());
        match result.unwrap_err() {
            CsvError::FileTooLarge { max_mb, actual_mb } => {
                assert_eq!(max_mb, 10);
                assert_eq!(actual_mb, 15);
            }
            _ => panic!("Expected FileTooLarge error"),
        }
    }

    #[test]
    fn test_validate_file_success() {
        let config = ValidationConfig::default();
        let result = validate_file(1024 * 1024, "test.csv", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_structure_no_headers() {
        let config = ValidationConfig {
            require_headers: true,
            ..Default::default()
        };

        let result = validate_structure(&[], 0, &config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CsvError::NoHeaders);
    }

    #[test]
    fn test_validate_structure_duplicate_headers() {
        let config = ValidationConfig::default();
        let headers = vec!["name".into(), "name".into(), "age".into()];

        let result = validate_structure(&headers, 0, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_structure_empty_header() {
        let config = ValidationConfig::default();
        let headers = vec!["name".into(), "".into(), "age".into()];

        let result = validate_structure(&headers, 0, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_structure_success() {
        let config = ValidationConfig::default();
        let headers = vec!["name".into(), "age".into(), "city".into()];

        let result = validate_structure(&headers, 100, &config);
        assert!(result.is_ok());
    }
}
