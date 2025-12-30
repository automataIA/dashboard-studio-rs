use super::template::{DashboardTemplate, TemplateType};
use super::error::{ExportError, ImportError};
use super::validation;
use crate::features::dashboard::context::DashboardContext;
use crate::features::dashboard::io;
use leptos::logging::*;

/// Export service - handles all export/import business logic
pub struct ExportService;

impl ExportService {
    /// Export dashboard to JSON file with comprehensive logging
    pub fn export_dashboard(
        context: &DashboardContext,
        template_type: TemplateType,
    ) -> Result<String, ExportError> {
        log!("=== Export Dashboard Started ===");
        log!("Template type: {:?}", template_type);

        // Step 1: Pre-flight checks
        debug_log!("Step 1: Validating dashboard state");
        let widgets = context.get_widgets();
        if widgets.is_empty() {
            error!("Export failed: Dashboard has no widgets");
            return Err(ExportError::EmptyDashboard);
        }
        log!("Dashboard has {} widgets", widgets.len());

        // Step 2: Create template
        debug_log!("Step 2: Creating template from context");
        let template = context.export_template(template_type);
        log!(
            "Template created: {} widgets, {} datasets, {} layers",
            template.widgets.len(),
            template.datasets.len(),
            template.layers.len()
        );

        // Step 3: Generate filename
        debug_log!("Step 3: Generating filename");
        let filename = Self::generate_filename(&template)?;
        log!("Generated filename: {}", filename);

        // Step 4: Serialize to JSON
        debug_log!("Step 4: Serializing template to JSON");
        let _json = template.to_json().map_err(|e| {
            error!("Serialization failed: {}", e);
            ExportError::SerializationFailed {
                context: "dashboard template".to_string(),
                inner_error: e.to_string(),
            }
        })?;
        log!("Serialization successful");

        // Step 5: Download file
        debug_log!("Step 5: Initiating browser download");
        io::download_json(&template, &filename).map_err(|e| {
            error!("Download failed: {:?}", e);
            ExportError::DownloadFailed {
                filename: filename.clone(),
                js_error: format!("{:?}", e),
            }
        })?;

        log!("=== Export Dashboard Completed Successfully ===");
        Ok(filename)
    }

    /// Import dashboard from template with comprehensive validation
    pub fn import_dashboard(
        context: &DashboardContext,
        json_content: &str,
        filename: &str,
    ) -> Result<(), ImportError> {
        log!("=== Import Dashboard Started ===");
        log!("Filename: {}", filename);
        log!("Content size: {} bytes", json_content.len());

        // Step 1: Parse JSON
        debug_log!("Step 1: Parsing JSON");
        let mut template = DashboardTemplate::from_json(json_content).map_err(|e| {
            error!("JSON parse failed: {}", e);

            // Try to extract line number from error message
            let line = Self::extract_line_number(&e.to_string());

            ImportError::ParseFailed {
                filename: filename.to_string(),
                line,
                inner_error: e.to_string(),
            }
        })?;
        log!("JSON parsed successfully");
        log!(
            "Template version: {}, title: '{}'",
            template.version,
            template.metadata.title
        );

        // Step 2: Validate schema version
        debug_log!("Step 2: Validating schema version");
        if !validation::SUPPORTED_VERSIONS.contains(&template.version.as_str()) {
            error!("Unsupported version: {}", template.version);
            return Err(ImportError::UnsupportedVersion {
                found: template.version.clone(),
                supported: validation::SUPPORTED_VERSIONS
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            });
        }
        log!("Version {} is supported", template.version);

        // Step 3: Comprehensive validation
        debug_log!("Step 3: Running comprehensive validation");
        match validation::validate_template(&template) {
            Ok(warnings) => {
                // Non-blocking warnings
                if !warnings.is_empty() {
                    warn!("Template has {} warnings:", warnings.len());
                    for warning in &warnings {
                        warn!("  - {}", warning);
                    }
                }
            }
            Err(errors) => {
                // Blocking errors
                error!("Template validation failed with {} errors:", errors.len());
                for error in &errors {
                    error!("  - {}", error);
                }
                return Err(ImportError::ValidationFailed { errors });
            }
        }
        log!("Validation passed");

        // Step 4: Auto-sanitization
        debug_log!("Step 4: Applying automatic fixes");
        let fixes = validation::sanitize_template(&mut template);
        if !fixes.is_empty() {
            log!("Applied {} automatic fixes:", fixes.len());
            for fix in &fixes {
                log!("  - {}", fix);
            }
        }

        // Step 5: Import into context
        debug_log!("Step 5: Importing into dashboard context");
        context.import_template(template);
        log!("Template imported into context");

        log!("=== Import Dashboard Completed Successfully ===");
        Ok(())
    }

    /// Generate safe filename for export
    fn generate_filename(template: &DashboardTemplate) -> Result<String, ExportError> {
        let title = &template.metadata.title;

        if title.trim().is_empty() {
            return Err(ExportError::FilenameGenerationFailed {
                title: title.clone(),
                reason: "Title is empty".to_string(),
            });
        }

        // Sanitize title: keep only alphanumeric, replace others with underscore
        let safe_title: String = title
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();

        // Generate timestamp
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");

        // Add suffix based on template type
        let suffix = match template.metadata.template_type {
            TemplateType::Generic => "",
            TemplateType::Complete => "_complete",
        };

        Ok(format!("{}{}_{}.json", safe_title, suffix, timestamp))
    }

    /// Extract line number from serde_json error message
    fn extract_line_number(error_msg: &str) -> Option<u32> {
        // serde_json errors typically include "at line X column Y"
        error_msg
            .split("line ")
            .nth(1)
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse::<u32>().ok())
    }
}
