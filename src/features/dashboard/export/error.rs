use std::fmt;

/// Errors that can occur during template export
#[derive(Debug, Clone)]
pub enum ExportError {
    /// Failed to serialize dashboard to JSON
    SerializationFailed {
        context: String,
        inner_error: String,
    },

    /// Failed to generate filename
    FilenameGenerationFailed {
        title: String,
        reason: String,
    },

    /// Browser file download failed
    DownloadFailed {
        filename: String,
        js_error: String,
    },

    /// No widgets to export
    EmptyDashboard,
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SerializationFailed { context, inner_error } => {
                write!(f, "Failed to serialize {}: {}", context, inner_error)
            }
            Self::FilenameGenerationFailed { title, reason } => {
                write!(f, "Cannot generate filename for '{}': {}", title, reason)
            }
            Self::DownloadFailed { filename, js_error } => {
                write!(f, "Failed to download file '{}': {}", filename, js_error)
            }
            Self::EmptyDashboard => {
                write!(f, "Cannot export empty dashboard (no widgets configured)")
            }
        }
    }
}

/// Errors that can occur during template import
#[derive(Debug, Clone)]
pub enum ImportError {
    /// Failed to parse JSON file
    ParseFailed {
        filename: String,
        line: Option<u32>,
        inner_error: String,
    },

    /// Unsupported schema version
    UnsupportedVersion {
        found: String,
        supported: Vec<String>,
    },

    /// Data validation failed
    ValidationFailed {
        errors: Vec<ValidationError>,
    },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseFailed { filename, line, inner_error } => {
                if let Some(line_num) = line {
                    write!(f, "Failed to parse '{}' at line {}: {}", filename, line_num, inner_error)
                } else {
                    write!(f, "Failed to parse '{}': {}", filename, inner_error)
                }
            }
            Self::UnsupportedVersion { found, supported } => {
                write!(
                    f,
                    "Template version '{}' is not supported. Supported versions: {}",
                    found,
                    supported.join(", ")
                )
            }
            Self::ValidationFailed { errors } => {
                write!(f, "Template validation failed ({} errors)", errors.len())
            }
        }
    }
}

/// Specific validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub severity: ValidationSeverity,
    pub field_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationSeverity {
    Error,   // Blocking - cannot import
    Warning, // Non-blocking - can import with fixes
}

impl ValidationError {
    pub fn error(field_path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Error,
            field_path: field_path.into(),
            message: message.into(),
        }
    }

    pub fn warning(field_path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Warning,
            field_path: field_path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.severity {
            ValidationSeverity::Error => "ERROR",
            ValidationSeverity::Warning => "WARNING",
        };
        write!(f, "[{}] {}: {}", prefix, self.field_path, self.message)
    }
}

/// User-friendly error messages
#[allow(dead_code)]
impl ExportError {
    pub fn user_message(&self) -> String {
        match self {
            Self::SerializationFailed { .. } => {
                "Unable to prepare dashboard for export. Please try again.".to_string()
            }
            Self::FilenameGenerationFailed { .. } => {
                "Cannot create download filename. Please rename your dashboard.".to_string()
            }
            Self::DownloadFailed { .. } => {
                "Download failed. Please check your browser settings and try again.".to_string()
            }
            Self::EmptyDashboard => {
                "Cannot export an empty dashboard. Please add at least one widget.".to_string()
            }
        }
    }
}

#[allow(dead_code)]
impl ImportError {
    pub fn user_message(&self) -> String {
        match self {
            Self::ParseFailed { .. } => {
                "Invalid file format. Please select a valid dashboard template (.json).".to_string()
            }
            Self::UnsupportedVersion { found, .. } => {
                format!(
                    "This template (version {}) is not compatible with the current version. Please export a new template.",
                    found
                )
            }
            Self::ValidationFailed { errors } => {
                let error_count = errors.iter()
                    .filter(|e| e.severity == ValidationSeverity::Error)
                    .count();
                format!(
                    "Template validation failed with {} error(s). Check console for details.",
                    error_count
                )
            }
        }
    }
}
