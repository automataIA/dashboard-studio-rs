use serde::{Deserialize, Serialize};
use super::super::models::*;
use super::super::context::DashboardContext;
use leptos::prelude::GetUntracked;

/// Dashboard template for export/import
/// Inspired by Grafana/DataDog JSON dashboard format
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardTemplate {
    /// Template schema version (for future compatibility)
    pub version: String,

    /// Dashboard metadata
    pub metadata: DashboardMetadata,

    /// Widgets (chart configurations + layout)
    pub widgets: Vec<Widget>,

    /// Datasets (fields schema + optional data)
    pub datasets: Vec<DatasetExport>,

    /// Layers (widget visibility/lock state)
    pub layers: Vec<Layer>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardMetadata {
    pub title: String,
    pub created_at: String,  // ISO 8601 timestamp
    pub exported_at: String,
    pub template_type: TemplateType,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TemplateType {
    /// Generic template - configuration only, no actual data
    Generic,
    /// Complete template - includes data snapshots
    Complete,
}

/// Dataset export format
/// Conditionally includes data based on template_type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatasetExport {
    pub id: DatasetId,
    pub name: String,
    pub fields: Vec<Field>,

    /// Optional data rows (only for Complete templates)
    /// For Generic templates, this is None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Vec<serde_json::Value>>>,

    /// For Complete templates with external CSV files
    /// Relative path in .zip bundle: "./data/{filename}"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_path: Option<String>,
}

/// Current schema version (SchemaVer format: MODEL-REVISION-ADDITION)
pub const SCHEMA_VERSION: &str = "1-0-0";

impl DashboardTemplate {
    /// Create template from DashboardContext
    pub fn from_context(
        ctx: &DashboardContext,
        template_type: TemplateType,
    ) -> Self {
        // Use get_untracked() for non-reactive export context
        let datasets = ctx.datasets.get_untracked();
        let widgets = ctx.widgets.get_untracked();
        let layers = ctx.layers.get_untracked();
        let title = ctx.title.get_untracked();

        let datasets_export = datasets.into_iter().map(|ds| {
            DatasetExport {
                id: ds.id,
                name: ds.name.clone(),
                fields: ds.fields,
                data: match template_type {
                    TemplateType::Generic => None,  // No data for generic templates
                    TemplateType::Complete => Some(ds.data),
                },
                csv_path: None,
            }
        }).collect();

        let now = chrono::Utc::now().to_rfc3339();

        Self {
            version: SCHEMA_VERSION.into(),
            metadata: DashboardMetadata {
                title,
                created_at: now.clone(),
                exported_at: now,
                template_type,
            },
            widgets,
            datasets: datasets_export,
            layers,
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string with automatic version migration
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        // First, try to parse to detect version
        let mut template: Self = serde_json::from_str(json)?;

        // Auto-migrate legacy versions
        if template.version == "1.0" {
            log::info!("Migrating template from version '1.0' to '{}'", SCHEMA_VERSION);
            template.version = SCHEMA_VERSION.into();
        }

        Ok(template)
    }

    /// Check if this template uses a legacy version format
    pub fn is_legacy_version(&self) -> bool {
        self.version == "1.0"
    }

    /// Get the schema version this template uses
    pub fn schema_version(&self) -> &str {
        &self.version
    }
}
