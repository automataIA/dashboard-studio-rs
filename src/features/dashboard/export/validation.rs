use super::template::{DashboardTemplate, DatasetExport};
use super::error::ValidationError;
use crate::features::dashboard::models::{Widget, WidgetType, FieldType, Field};
use leptos::logging::*;

/// Supported template schema versions (includes legacy versions for backward compatibility)
pub const SUPPORTED_VERSIONS: &[&str] = &[
    "1.0",      // Legacy format
    "1-0-0",    // Current SchemaVer format
];

/// Validation result
pub type ValidationResult = Result<Vec<ValidationError>, Vec<ValidationError>>;

/// Validate a dashboard template comprehensively
pub fn validate_template(template: &DashboardTemplate) -> ValidationResult {
    log!("Starting template validation");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 1. Version check
    if let Err(e) = validate_version(&template.version) {
        errors.push(e);
        // Fatal error - return immediately
        return Err(errors);
    }

    // 2. Metadata validation
    validate_metadata(template, &mut errors, &mut warnings);

    // 3. Widgets validation
    validate_widgets(&template.widgets, &mut errors, &mut warnings);

    // 4. Datasets validation
    validate_datasets(&template.datasets, &mut errors, &mut warnings);

    // 5. Cross-references validation
    validate_cross_references(template, &mut errors, &mut warnings);

    log!("Validation complete: {} errors, {} warnings", errors.len(), warnings.len());

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(warnings)
    }
}

/// Validate schema version
fn validate_version(version: &str) -> Result<(), ValidationError> {
    debug_log!("Validating version: {}", version);

    if !SUPPORTED_VERSIONS.contains(&version) {
        return Err(ValidationError::error(
            "version",
            format!(
                "Unsupported version '{}'. Supported: {}",
                version,
                SUPPORTED_VERSIONS.join(", ")
            ),
        ));
    }

    Ok(())
}

/// Validate metadata fields
fn validate_metadata(
    template: &DashboardTemplate,
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationError>,
) {
    debug_log!("Validating metadata");

    let metadata = &template.metadata;

    // Title should not be empty
    if metadata.title.trim().is_empty() {
        warnings.push(ValidationError::warning(
            "metadata.title",
            "Dashboard title is empty",
        ));
    }

    // Validate timestamps (ISO 8601 format)
    if let Err(e) = validate_timestamp(&metadata.created_at) {
        errors.push(ValidationError::error(
            "metadata.created_at",
            format!("Invalid timestamp: {}", e),
        ));
    }

    if let Err(e) = validate_timestamp(&metadata.exported_at) {
        errors.push(ValidationError::error(
            "metadata.exported_at",
            format!("Invalid timestamp: {}", e),
        ));
    }
}

/// Validate timestamp format
fn validate_timestamp(timestamp: &str) -> Result<(), String> {
    chrono::DateTime::parse_from_rfc3339(timestamp)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Validate widgets array
fn validate_widgets(
    widgets: &[Widget],
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationError>,
) {
    debug_log!("Validating {} widgets", widgets.len());

    if widgets.is_empty() {
        warnings.push(ValidationError::warning(
            "widgets",
            "No widgets in template",
        ));
        return;
    }

    for (idx, widget) in widgets.iter().enumerate() {
        let path = format!("widgets[{}]", idx);

        // Validate widget ID
        if widget.id.trim().is_empty() {
            errors.push(ValidationError::error(
                format!("{}.id", path),
                "Widget ID cannot be empty",
            ));
        }

        // Validate position and size
        if widget.grid_position.width == 0 || widget.grid_position.height == 0 {
            errors.push(ValidationError::error(
                format!("{}.grid_position", path),
                "Grid size (width/height) must be greater than zero",
            ));
        }

        // Validate data mapping
        validate_widget_data_mapping(widget, &path, errors, warnings);
    }

    // Check for duplicate IDs
    let mut seen_ids = std::collections::HashSet::new();
    for (idx, widget) in widgets.iter().enumerate() {
        if !seen_ids.insert(&widget.id) {
            errors.push(ValidationError::error(
                format!("widgets[{}].id", idx),
                format!("Duplicate widget ID: '{}'", widget.id),
            ));
        }
    }
}

/// Validate widget data mapping based on widget type
fn validate_widget_data_mapping(
    widget: &Widget,
    path: &str,
    _errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationError>,
) {
    let mapping = &widget.chart_config.data_mapping;

    match widget.widget_type {
        WidgetType::Bar | WidgetType::Line | WidgetType::Area => {
            // Require x_axis and at least one y_axis
            if mapping.x_axis.is_none() {
                warnings.push(ValidationError::warning(
                    format!("{}.data_mapping.x_axis", path),
                    "No X-axis field configured",
                ));
            }

            if mapping.y_axis.is_empty() {
                warnings.push(ValidationError::warning(
                    format!("{}.data_mapping.y_axis", path),
                    "No Y-axis fields configured",
                ));
            }
        }

        WidgetType::Pie => {
            // Require x_axis (dimension) and y_axis (value)
            if mapping.x_axis.is_none() {
                warnings.push(ValidationError::warning(
                    format!("{}.data_mapping.x_axis", path),
                    "No dimension field (x_axis) configured for Pie chart",
                ));
            }

            if mapping.y_axis.is_empty() {
                warnings.push(ValidationError::warning(
                    format!("{}.data_mapping.y_axis", path),
                    "No value field (y_axis) configured for Pie chart",
                ));
            }
        }

        WidgetType::Scatter => {
            // Require x_axis, y_axis
            if mapping.x_axis.is_none() || mapping.y_axis.is_empty() {
                warnings.push(ValidationError::warning(
                    format!("{}.data_mapping", path),
                    "Scatter plot requires X and Y axes",
                ));
            }
        }

        WidgetType::Table | WidgetType::Kpi => {
            // Less strict requirements
            if mapping.columns.is_empty() {
                warnings.push(ValidationError::warning(
                    format!("{}.data_mapping.columns", path),
                    "No columns configured",
                ));
            }
        }

        _ => {
            // Other widget types
        }
    }
}

/// Validate datasets array
fn validate_datasets(
    datasets: &[DatasetExport],
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationError>,
) {
    debug_log!("Validating {} datasets", datasets.len());

    if datasets.is_empty() {
        warnings.push(ValidationError::warning(
            "datasets",
            "No datasets in template",
        ));
        return;
    }

    for (idx, dataset) in datasets.iter().enumerate() {
        let path = format!("datasets[{}]", idx);

        // Validate dataset ID
        if dataset.id.trim().is_empty() {
            errors.push(ValidationError::error(
                format!("{}.id", path),
                "Dataset ID cannot be empty",
            ));
        }

        // Validate dataset name
        if dataset.name.trim().is_empty() {
            errors.push(ValidationError::error(
                format!("{}.name", path),
                "Dataset name cannot be empty",
            ));
        }

        // Validate fields
        if dataset.fields.is_empty() {
            errors.push(ValidationError::error(
                format!("{}.fields", path),
                "Dataset must have at least one field",
            ));
        }

        validate_fields(&dataset.fields, &path, errors, warnings);

        // Validate data if present
        if let Some(data) = &dataset.data {
            validate_dataset_data(data, &dataset.fields, &path, errors, warnings);
        }
    }

    // Check for duplicate dataset IDs
    let mut seen_ids = std::collections::HashSet::new();
    for (idx, dataset) in datasets.iter().enumerate() {
        if !seen_ids.insert(&dataset.id) {
            errors.push(ValidationError::error(
                format!("datasets[{}].id", idx),
                format!("Duplicate dataset ID: '{}'", dataset.id),
            ));
        }
    }
}

/// Validate field definitions
fn validate_fields(
    fields: &[Field],
    base_path: &str,
    errors: &mut Vec<ValidationError>,
    _warnings: &mut Vec<ValidationError>,
) {
    let mut seen_names = std::collections::HashSet::new();

    for (idx, field) in fields.iter().enumerate() {
        let path = format!("{}.fields[{}]", base_path, idx);

        // Field name cannot be empty
        if field.name.trim().is_empty() {
            errors.push(ValidationError::error(
                format!("{}.name", path),
                "Field name cannot be empty",
            ));
        }

        // Check for duplicate field names
        if !seen_names.insert(&field.name) {
            errors.push(ValidationError::error(
                format!("{}.name", path),
                format!("Duplicate field name: '{}'", field.name),
            ));
        }
    }
}

/// Validate dataset data rows
fn validate_dataset_data(
    data: &[Vec<serde_json::Value>],
    fields: &[Field],
    base_path: &str,
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationError>,
) {
    debug_log!("Validating dataset data: {} rows, {} fields", data.len(), fields.len());

    if data.is_empty() {
        warnings.push(ValidationError::warning(
            format!("{}.data", base_path),
            "Dataset has no data rows",
        ));
        return;
    }

    // Check each row has correct number of columns
    for (row_idx, row) in data.iter().enumerate() {
        if row.len() != fields.len() {
            errors.push(ValidationError::error(
                format!("{}.data[{}]", base_path, row_idx),
                format!(
                    "Row has {} columns but {} fields are defined",
                    row.len(),
                    fields.len()
                ),
            ));
            continue;
        }

        // Validate field types (sample first 10 rows for performance)
        if row_idx < 10 {
            for (col_idx, (value, field)) in row.iter().zip(fields.iter()).enumerate() {
                if !value.is_null() {
                    validate_field_value(
                        value,
                        field,
                        &format!("{}.data[{}][{}]", base_path, row_idx, col_idx),
                        warnings,
                    );
                }
            }
        }
    }
}

/// Validate a single field value matches expected type
fn validate_field_value(
    value: &serde_json::Value,
    field: &Field,
    path: &str,
    warnings: &mut Vec<ValidationError>,
) {
    let type_matches = match field.field_type {
        FieldType::Numeric => value.is_number(),
        FieldType::Text => value.is_string(),
        FieldType::Boolean => value.is_boolean(),
        FieldType::Date => {
            // Check if string can be parsed as date
            if let Some(s) = value.as_str() {
                chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok()
                    || chrono::DateTime::parse_from_rfc3339(s).is_ok()
            } else {
                false
            }
        }
    };

    if !type_matches {
        warnings.push(ValidationError::warning(
            path,
            format!(
                "Field '{}' expects {:?} but got {:?}",
                field.name,
                field.field_type,
                value
            ),
        ));
    }
}

/// Validate cross-references between widgets, datasets, and layers
fn validate_cross_references(
    template: &DashboardTemplate,
    errors: &mut Vec<ValidationError>,
    _warnings: &mut Vec<ValidationError>,
) {
    debug_log!("Validating cross-references");

    // Build widget ID set
    let widget_ids: std::collections::HashSet<_> =
        template.widgets.iter().map(|w| &w.id).collect();

    // Check layer widget references
    for (idx, layer) in template.layers.iter().enumerate() {
        if !widget_ids.contains(&layer.widget_id) {
            errors.push(ValidationError::error(
                format!("layers[{}].widget_id", idx),
                format!("References non-existent widget: '{}'", layer.widget_id),
            ));
        }
    }
}

/// Auto-fix common issues (non-destructive)
pub fn sanitize_template(template: &mut DashboardTemplate) -> Vec<String> {
    log!("Sanitizing template");

    let mut fixes = Vec::new();

    // Fix empty title
    if template.metadata.title.trim().is_empty() {
        template.metadata.title = "Untitled Dashboard".to_string();
        fixes.push("Set empty title to 'Untitled Dashboard'".to_string());
    }

    // Remove widgets with invalid positions
    let original_count = template.widgets.len();
    template.widgets.retain(|w| w.grid_position.width > 0 && w.grid_position.height > 0);
    let removed = original_count - template.widgets.len();
    if removed > 0 {
        fixes.push(format!("Removed {} widgets with invalid dimensions", removed));
    }

    // Remove orphaned layers
    let widget_ids: std::collections::HashSet<_> =
        template.widgets.iter().map(|w| &w.id).collect();

    let original_layer_count = template.layers.len();
    template.layers.retain(|l| widget_ids.contains(&l.widget_id));
    let removed_layers = original_layer_count - template.layers.len();
    if removed_layers > 0 {
        fixes.push(format!("Removed {} orphaned layers", removed_layers));
    }

    log!("Applied {} automatic fixes", fixes.len());
    fixes
}
