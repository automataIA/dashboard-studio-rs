use super::models::*;
use super::history::{Command, HistoryManager};
use leptos::prelude::*;

/// Dashboard context managing all dashboard state
///
/// This context follows Leptos 0.8 patterns:
/// - Uses signals for reactive state
/// - Provides CRUD methods for widgets, datasets, layers
/// - Manages active selections
///
/// # Example
/// ```rust
/// // In dashboard page
/// let dashboard = DashboardContext::provide();
///
/// // In child components
/// let dashboard = DashboardContext::use_context();
/// dashboard.add_widget(widget);
/// ```
#[derive(Clone, Copy)]
pub struct DashboardContext {
    // State signals
    /// Public signal for datasets (needed for export/import tracking)
    pub datasets: ReadSignal<Vec<Dataset>>,
    set_datasets: WriteSignal<Vec<Dataset>>,

    /// Public signal for widgets (needed for auto-save tracking)
    pub widgets: ReadSignal<Vec<Widget>>,
    set_widgets: WriteSignal<Vec<Widget>>,

    /// Public signal for layers (needed for auto-save tracking)
    pub layers: ReadSignal<Vec<Layer>>,
    set_layers: WriteSignal<Vec<Layer>>,

    templates: ReadSignal<Vec<Template>>,
    set_templates: WriteSignal<Vec<Template>>,

    // Active selections
    active_dataset_id: ReadSignal<Option<DatasetId>>,
    set_active_dataset_id: WriteSignal<Option<DatasetId>>,

    /// Public signal for accessing selected widget ID
    pub selected_widget_id: ReadSignal<Option<WidgetId>>,
    set_selected_widget_id: WriteSignal<Option<WidgetId>>,

    /// Widget type selected for creation (from WidgetSelector)
    pub pending_widget_type: ReadSignal<WidgetType>,
    pub set_pending_widget_type: WriteSignal<WidgetType>,

    /// Whether the canvas grid pattern is visible
    pub grid_view_active: ReadSignal<bool>,
    pub set_grid_view_active: WriteSignal<bool>,

    /// Dashboard title
    pub title: ReadSignal<String>,
    pub set_title: WriteSignal<String>,

    /// Last edited timestamp text
    pub last_edited: ReadSignal<String>,
    pub set_last_edited: WriteSignal<String>,

    /// Whether dashboard is auto-saved
    pub auto_saved: ReadSignal<bool>,
    pub set_auto_saved: WriteSignal<bool>,

    /// Current dashboard's storage key (for saving/loading)
    pub storage_key: ReadSignal<String>,
    pub set_storage_key: WriteSignal<String>,

    // ===== History Management (Undo/Redo) =====

    /// History manager for undo/redo operations
    history: ReadSignal<HistoryManager>,
    set_history: WriteSignal<HistoryManager>,

    /// Whether undo is available (for button state)
    pub can_undo: ReadSignal<bool>,
    set_can_undo: WriteSignal<bool>,

    /// Whether redo is available (for button state)
    pub can_redo: ReadSignal<bool>,
    set_can_redo: WriteSignal<bool>,
}

impl Default for DashboardContext {
    fn default() -> Self {
        Self::new()
    }
}

impl DashboardContext {
    /// Create new dashboard context with empty state
    pub fn new() -> Self {
        let (datasets, set_datasets) = signal(Vec::new());
        let (widgets, set_widgets) = signal(Vec::new());
        let (layers, set_layers) = signal(Vec::new());
        let (templates, set_templates) = signal(Vec::new());
        let (active_dataset_id, set_active_dataset_id) = signal(None);
        let (selected_widget_id, set_selected_widget_id) = signal(None);
        let (pending_widget_type, set_pending_widget_type) = signal(WidgetType::Line);
        let (grid_view_active, set_grid_view_active) = signal(true); // Default to true
        let (title, set_title) = signal(String::from("Untitled Dashboard"));
        let (last_edited, set_last_edited) = signal(String::from("Last edited just now"));
        let (auto_saved, set_auto_saved) = signal(true);

        // Storage management
        let (storage_key, set_storage_key) = signal(String::new());

        // History management
        let (history, set_history) = signal(HistoryManager::new(50));
        let (can_undo, set_can_undo) = signal(false);
        let (can_redo, set_can_redo) = signal(false);

        Self {
            datasets,
            set_datasets,
            widgets,
            set_widgets,
            layers,
            set_layers,
            templates,
            set_templates,
            active_dataset_id,
            set_active_dataset_id,
            selected_widget_id,
            set_selected_widget_id,
            pending_widget_type,
            set_pending_widget_type,
            grid_view_active,
            set_grid_view_active,
            title,
            set_title,
            last_edited,
            set_last_edited,
            auto_saved,
            set_auto_saved,
            storage_key,
            set_storage_key,
            history,
            set_history,
            can_undo,
            set_can_undo,
            can_redo,
            set_can_redo,
        }
    }

    /// Provide context at dashboard page root
    pub fn provide() -> Self {
        let context = Self::new();
        provide_context(context);
        context
    }

    /// Get context in child components
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    // ===== Dataset Methods =====

    /// Get all datasets
    pub fn get_datasets(&self) -> Vec<Dataset> {
        self.datasets.get()
    }

    /// Add a new dataset
    pub fn add_dataset(&self, dataset: Dataset) {
        self.set_datasets.update(|datasets| {
            datasets.push(dataset);
        });
    }

    /// Remove dataset by ID
    pub fn remove_dataset(&self, id: &str) {
        self.set_datasets.update(|datasets| {
            datasets.retain(|d| d.id != id);
        });
    }

    /// Set active dataset
    pub fn set_active_dataset(&self, id: Option<DatasetId>) {
        // Deactivate all datasets
        self.set_datasets.update(|datasets| {
            for dataset in datasets.iter_mut() {
                dataset.active = false;
            }
        });

        // Activate selected dataset
        if let Some(dataset_id) = &id {
            self.set_datasets.update(|datasets| {
                if let Some(dataset) = datasets.iter_mut().find(|d| &d.id == dataset_id) {
                    dataset.active = true;
                }
            });
        }

        self.set_active_dataset_id.set(id);
    }

    /// Get active dataset
    pub fn get_active_dataset(&self) -> Option<Dataset> {
        let dataset_id = self.active_dataset_id.get()?;
        self.datasets.get().into_iter().find(|d| d.id == dataset_id)
    }

    // ===== Widget Methods =====

    /// Get all widgets
    pub fn get_widgets(&self) -> Vec<Widget> {
        self.widgets.get()
    }

    /// Add a new widget
    pub fn add_widget(&self, widget: Widget) {
        let widget_id = widget.id.clone();
        let widget_title = widget.title.clone();
        let widget_type = widget.widget_type;

        // CAPTURE STATE BEFORE MUTATION
        let widget_clone = widget.clone();

        // EXECUTE MUTATION
        self.set_widgets.update(|widgets| {
            widgets.push(widget);
        });

        // Automatically create a layer for the widget
        let layer = Layer {
            id: format!("layer_{}", widget_id),
            widget_id: widget_id.clone(),
            label: widget_title,
            icon: widget_type.icon_name(),
            visible: true,
            locked: false,
        };
        let layer_clone = layer.clone();
        self.add_layer(layer);

        // RECORD COMMAND
        let command = Command::AddWidget {
            widget: Box::new(widget_clone),
            layer: layer_clone,
        };
        self.set_history.update(|history| {
            history.execute(command);
        });
        self.update_history_availability();

        log::info!("Created layer for widget: {}", widget_id);
        self.mark_as_edited();
    }

    /// Update an existing widget
    pub fn update_widget(&self, id: &str, updater: impl FnOnce(&mut Widget)) {
        // CAPTURE STATE BEFORE MUTATION
        let previous_state = self
            .widgets
            .get()
            .iter()
            .find(|w| w.id == id)
            .cloned();

        if previous_state.is_none() {
            log::warn!("Attempted to update non-existent widget: {}", id);
            return;
        }

        // EXECUTE MUTATION
        self.set_widgets.update(|widgets| {
            if let Some(widget) = widgets.iter_mut().find(|w| w.id == id) {
                updater(widget);
            }
        });

        // CAPTURE NEW STATE
        let new_state = self
            .widgets
            .get()
            .iter()
            .find(|w| w.id == id)
            .cloned()
            .unwrap();

        // RECORD COMMAND (only if state changed)
        if previous_state.as_ref() != Some(&new_state) {
            let command = Command::UpdateWidget {
                widget_id: id.to_string(),
                previous_state: Box::new(previous_state.unwrap()),
                new_state: Box::new(new_state),
            };
            self.set_history.update(|history| {
                history.execute(command);
            });
            self.update_history_availability();
        }

        self.mark_as_edited();
    }

    /// Remove widget by ID
    pub fn remove_widget(&self, id: &str) {
        // CAPTURE STATE BEFORE MUTATION
        let (removed_widget, removed_layer) = (
            self.widgets.get().into_iter().find(|w| w.id == id),
            self.layers.get().into_iter().find(|l| l.widget_id == id),
        );

        let (widget, layer) = match (removed_widget, removed_layer) {
            (Some(w), Some(l)) => (w, l),
            _ => {
                log::warn!("Attempted to remove non-existent widget: {}", id);
                return;
            }
        };

        // EXECUTE MUTATION
        self.set_widgets.update(|widgets| {
            widgets.retain(|w| w.id != id);
        });

        // Also remove associated layer
        self.set_layers.update(|layers| {
            layers.retain(|l| l.widget_id != id);
        });

        // RECORD COMMAND
        let command = Command::RemoveWidget {
            widget: Box::new(widget.clone()),
            layer: layer.clone(),
        };
        self.set_history.update(|history| {
            history.execute(command);
        });
        self.update_history_availability();

        self.mark_as_edited();
    }

    /// Set widget editing state
    pub fn set_widget_editing(&self, id: &str, editing: bool) {
        self.update_widget(id, |widget| {
            widget.editing = editing;
        });
    }

    /// Update widget data mapping
    pub fn update_widget_mapping(&self, id: &str, mapping: DataMapping) {
        // CAPTURE STATE BEFORE MUTATION
        let previous_mapping = self
            .widgets
            .get()
            .into_iter()
            .find(|w| w.id == id)
            .map(|w| w.chart_config.data_mapping.clone());

        if previous_mapping.is_none() {
            log::warn!("Attempted to update mapping for non-existent widget: {}", id);
            return;
        }

        // Skip if mapping unchanged
        if previous_mapping.as_ref() == Some(&mapping) {
            return;
        }

        // EXECUTE MUTATION
        self.set_widgets.update(|widgets| {
            if let Some(widget) = widgets.iter_mut().find(|w| w.id == id) {
                widget.chart_config.data_mapping = mapping.clone();
            }
        });

        // RECORD COMMAND
        let command = Command::UpdateDataMapping {
            widget_id: id.to_string(),
            previous_mapping: Box::new(previous_mapping.unwrap()),
            new_mapping: Box::new(mapping),
        };
        self.set_history.update(|history| {
            history.execute(command);
        });
        self.update_history_availability();

        self.mark_as_edited();
    }

    /// Update widget grid position
    pub fn update_widget_position(&self, id: &str, position: GridPosition) {
        // CAPTURE STATE BEFORE MUTATION
        let previous_position = self
            .widgets
            .get()
            .into_iter()
            .find(|w| w.id == id)
            .map(|w| w.grid_position);

        if previous_position.is_none() {
            log::warn!("Attempted to move non-existent widget: {}", id);
            return;
        }

        // Skip if position unchanged
        if previous_position == Some(position) {
            return;
        }

        // EXECUTE MUTATION (direct, not via update_widget to avoid double-recording)
        self.set_widgets.update(|widgets| {
            if let Some(widget) = widgets.iter_mut().find(|w| w.id == id) {
                widget.grid_position = position;
            }
        });

        // RECORD COMMAND
        let command = Command::MoveWidget {
            widget_id: id.to_string(),
            previous_position: previous_position.unwrap(),
            new_position: position,
        };
        self.set_history.update(|history| {
            history.execute(command);
        });
        self.update_history_availability();

        self.mark_as_edited();
    }

    /// Get selected widget
    pub fn get_selected_widget(&self) -> Option<Widget> {
        let widget_id = self.selected_widget_id.get()?;
        self.widgets.get().into_iter().find(|w| w.id == widget_id)
    }

    /// Set selected widget
    pub fn set_selected_widget(&self, id: Option<WidgetId>) {
        self.set_selected_widget_id.set(id);
    }

    // ===== Layer Methods =====

    /// Get all layers
    pub fn get_layers(&self) -> Vec<Layer> {
        self.layers.get()
    }

    /// Add layer (automatically created when widget is added)
    pub fn add_layer(&self, layer: Layer) {
        self.set_layers.update(|layers| {
            layers.push(layer);
        });
    }

    /// Toggle layer visibility
    pub fn toggle_layer_visibility(&self, id: &str) {
        self.set_layers.update(|layers| {
            if let Some(layer) = layers.iter_mut().find(|l| l.id == id) {
                layer.visible = !layer.visible;
            }
        });
    }

    /// Toggle layer lock
    pub fn toggle_layer_lock(&self, id: &str) {
        self.set_layers.update(|layers| {
            if let Some(layer) = layers.iter_mut().find(|l| l.id == id) {
                layer.locked = !layer.locked;
            }
        });
    }

    // ===== Template Methods =====

    /// Get all templates
    pub fn get_templates(&self) -> Vec<Template> {
        self.templates.get()
    }

    /// Initialize templates (called on context creation)
    pub fn init_templates(&self, templates: Vec<Template>) {
        self.set_templates.set(templates);
    }

    /// Create widgets from template (supports multi-widget layouts)
    /// Returns the number of widgets created
    ///
    /// # Arguments
    /// * `template_id` - ID of the template to use
    /// * `kpi_field` - Optional KPI field name for KPI widgets
    /// * `kpi_aggregation` - Optional KPI aggregation type for KPI widgets
    pub fn create_widgets_from_template(
        &self,
        template_id: &str,
        kpi_field: Option<String>,
        kpi_aggregation: Option<crate::features::dashboard::KpiAggregation>,
    ) -> usize {
        let template = self
            .templates
            .get_untracked()
            .into_iter()
            .find(|t| t.id == template_id);

        let template = match template {
            Some(t) => t,
            None => {
                log::error!("Template not found: {}", template_id);
                return 0;
            }
        };

        // If template has predefined widgets, create them
        if !template.widgets.is_empty() {
            let mut created_count = 0;

            for template_widget in &template.widgets {
                let widget_id = format!("widget_{}", uuid::Uuid::new_v4());

                // Apply KPI configuration if this is a KPI widget and KPI config was provided
                let chart_config = if template_widget.widget_type == WidgetType::Kpi {
                    if let (Some(field), Some(aggr)) = (&kpi_field, &kpi_aggregation) {
                        // Clone and modify the chart_config with KPI settings
                        let mut config = template_widget.chart_config.clone();
                        config.data_mapping.kpi_field = Some(field.clone());
                        config.data_mapping.kpi_aggregation = Some(*aggr);
                        config
                    } else {
                        template_widget.chart_config.clone()
                    }
                } else {
                    template_widget.chart_config.clone()
                };

                let widget = Widget {
                    id: widget_id.clone(),
                    title: template_widget.title.clone(),
                    subtitle: None,
                    widget_type: template_widget.widget_type,
                    chart_config,
                    grid_position: template_widget.grid_position,
                    editing: false,
                };

                // Add widget
                self.set_widgets.update(|widgets| widgets.push(widget.clone()));

                // Create corresponding layer
                let layer = Layer {
                    id: format!("layer_{}", widget_id),
                    widget_id: widget_id.clone(),
                    label: template_widget.title.clone(),
                    icon: template_widget.widget_type.icon_name(),
                    visible: true,
                    locked: false,
                };

                self.set_layers.update(|layers| layers.push(layer));

                created_count += 1;
            }

            log::info!("Created {} widgets from template: {}", created_count, template_id);
            created_count
        } else {
            // Fallback: create single widget from default_config (legacy)
            let widget_id = format!("widget_{}", uuid::Uuid::new_v4());

            // Apply KPI configuration if this is a KPI widget and KPI config was provided
            let chart_config = if template.preview_type == WidgetType::Kpi {
                if let (Some(field), Some(aggr)) = (&kpi_field, &kpi_aggregation) {
                    let mut config = template.default_config.clone();
                    config.data_mapping.kpi_field = Some(field.clone());
                    config.data_mapping.kpi_aggregation = Some(*aggr);
                    config
                } else {
                    template.default_config.clone()
                }
            } else {
                template.default_config.clone()
            };

            let widget = Widget {
                id: widget_id.clone(),
                title: template.title.clone(),
                subtitle: None,
                widget_type: template.preview_type,
                chart_config,
                grid_position: GridPosition::default(),
                editing: false,
            };

            self.set_widgets.update(|widgets| widgets.push(widget.clone()));

            let layer = Layer {
                id: format!("layer_{}", widget_id),
                widget_id: widget_id.clone(),
                label: template.title.clone(),
                icon: template.preview_type.icon_name(),
                visible: true,
                locked: false,
            };

            self.set_layers.update(|layers| layers.push(layer));

            log::info!("Created 1 widget from template (legacy): {}", template_id);
            1
        }
    }

    // ===== Dashboard Metadata Methods =====

    /// Update the dashboard title
    pub fn set_dashboard_title(&self, title: String) {
        self.set_title.set(title);
        self.mark_as_edited();
    }

    /// Mark dashboard as edited (updates timestamp)
    pub fn mark_as_edited(&self) {
        // Simple format for now - can be enhanced with proper date formatting
        self.set_last_edited.set(String::from("Last edited just now"));

        // Note: Auto-save indicator simulation removed to avoid adding gloo-timers dependency
        // In production, you would implement actual auto-save logic here
    }

    // ===== Storage Methods =====

    /// Generate storage key: {timestamp}_{title_slug}
    fn generate_storage_key(&self) -> String {
        let title = self.title.get_untracked();
        let timestamp = chrono::Utc::now().timestamp_millis();
        let title_slug = title.to_lowercase().replace(&[' ', '/', '\\'][..], "_");
        format!("{}_{}", timestamp, title_slug)
    }

    /// Save to localStorage (Generic template, no CSV data)
    pub fn save_to_storage(&self) {
        let template = self.export_template(crate::features::dashboard::export::TemplateType::Generic);

        // Reuse existing key if available, otherwise generate new one
        let key = {
            let existing_key = self.storage_key.get_untracked();
            if existing_key.is_empty() {
                self.generate_storage_key()
            } else {
                existing_key
            }
        };

        match crate::features::dashboard::io::storage::save_template(&key, &template) {
            Ok(_) => {
                log::info!("Dashboard auto-saved: {}", key);
                self.set_storage_key.set(key.clone());
                self.set_auto_saved.set(true);
            }
            Err(e) => {
                log::error!("Failed to auto-save: {:?}", e);
                self.set_auto_saved.set(false);
            }
        }
    }

    /// Load dashboard from localStorage by key
    pub fn load_from_storage(&self, key: &str) {
        match crate::features::dashboard::io::storage::load_template(key) {
            Ok(template) => {
                self.import_template(template);
                self.set_storage_key.set(key.to_string());
                log::info!("Dashboard loaded: {}", key);
            }
            Err(e) => log::error!("Failed to load: {}", e),
        }
    }

    // ===== Template Export/Import Methods =====

    /// Export dashboard as template
    pub fn export_template(&self, template_type: crate::features::dashboard::export::TemplateType) -> crate::features::dashboard::export::DashboardTemplate {
        log::debug!("DashboardContext::export_template called with type {:?}", template_type);
        crate::features::dashboard::export::DashboardTemplate::from_context(self, template_type)
    }

    /// Import template and restore dashboard state
    pub fn import_template(&self, template: crate::features::dashboard::export::DashboardTemplate) {
        log::info!("DashboardContext::import_template - starting import");

        use crate::features::dashboard::models::Dataset;

        // Convert DatasetExport → Dataset
        let datasets = template.datasets.into_iter().map(|ds_export| {
            log::debug!("Converting dataset: {} ({} fields)", ds_export.name, ds_export.fields.len());

            Dataset {
                id: ds_export.id,
                name: ds_export.name.clone(),
                size: "Imported".into(),
                uploaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
                fields: ds_export.fields,
                active: false,
                data: ds_export.data.unwrap_or_default(),
            }
        }).collect::<Vec<_>>();

        log::info!(
            "Importing {} widgets, {} datasets, {} layers",
            template.widgets.len(),
            datasets.len(),
            template.layers.len()
        );

        // Apply state (bypassing history to avoid recording import as undoable)
        self.set_widgets.set(template.widgets);
        self.set_layers.set(template.layers);
        self.set_datasets.set(datasets);
        self.set_title.set(template.metadata.title);

        // Clear history and reset to fresh state
        self.set_history.update(|history| {
            *history = crate::features::dashboard::history::HistoryManager::new(50);
        });
        self.update_history_availability();

        self.mark_as_edited();

        log::info!("DashboardContext::import_template - completed successfully");
    }

    // ===== Undo/Redo Methods =====

    /// Undo the last operation
    pub fn undo(&self) {
        let command = self.set_history.try_update(|history| {
            history.undo().cloned()
        });

        if let Some(command) = command.flatten() {
            self.execute_undo_command(command);
            self.update_history_availability();
            self.mark_as_edited();
        }
    }

    /// Redo the last undone operation
    pub fn redo(&self) {
        let command = self.set_history.try_update(|history| {
            history.redo().cloned()
        });

        if let Some(command) = command.flatten() {
            self.execute_redo_command(command);
            self.update_history_availability();
            self.mark_as_edited();
        }
    }

    /// Update undo/redo button availability signals
    fn update_history_availability(&self) {
        let history = self.history.get_untracked();
        self.set_can_undo.set(history.can_undo());
        self.set_can_redo.set(history.can_redo());
    }

    /// Execute undo logic for each command type
    fn execute_undo_command(&self, command: Command) {
        match command {
            Command::AddWidget { widget, layer } => {
                // Inverse: Remove widget and layer
                self.set_widgets.update(|widgets| {
                    widgets.retain(|w| w.id != widget.id);
                });
                self.set_layers.update(|layers| {
                    layers.retain(|l| l.id != layer.id);
                });
                log::info!("Undo: Removed widget {}", widget.id);
            }

            Command::RemoveWidget { widget, layer } => {
                // Inverse: Add widget and layer back
                self.set_widgets.update(|widgets| {
                    widgets.push((*widget).clone());
                });
                self.set_layers.update(|layers| {
                    layers.push(layer.clone());
                });
                log::info!("Undo: Restored widget {}", widget.id);
            }

            Command::UpdateWidget { widget_id, previous_state, .. } => {
                // Inverse: Restore previous state
                self.set_widgets.update(|widgets| {
                    if let Some(widget) = widgets.iter_mut().find(|w| w.id == widget_id) {
                        *widget = (*previous_state).clone();
                    }
                });
                log::info!("Undo: Restored widget {}", widget_id);
            }

            Command::MoveWidget { widget_id, previous_position, .. } => {
                // Inverse: Restore previous position
                self.set_widgets.update(|widgets| {
                    if let Some(widget) = widgets.iter_mut().find(|w| w.id == widget_id) {
                        widget.grid_position = previous_position;
                    }
                });
                log::info!("Undo: Moved widget {} back", widget_id);
            }

            Command::UpdateDataMapping { widget_id, previous_mapping, .. } => {
                // Inverse: Restore previous mapping
                self.set_widgets.update(|widgets| {
                    if let Some(widget) = widgets.iter_mut().find(|w| w.id == widget_id) {
                        widget.chart_config.data_mapping = previous_mapping.as_ref().clone();
                    }
                });
                log::info!("Undo: Restored mapping for widget {}", widget_id);
            }

            Command::Batch { commands, .. } => {
                // Undo batch in reverse order
                for cmd in commands.into_iter().rev() {
                    self.execute_undo_command(cmd);
                }
            }
        }
    }

    /// Execute redo logic for each command type
    fn execute_redo_command(&self, command: Command) {
        match command {
            Command::AddWidget { widget, layer } => {
                // Redo: Add widget and layer again
                self.set_widgets.update(|widgets| {
                    widgets.push((*widget).clone());
                });
                self.set_layers.update(|layers| {
                    layers.push(layer.clone());
                });
                log::info!("Redo: Added widget {}", widget.id);
            }

            Command::RemoveWidget { widget, layer: _ } => {
                // Redo: Remove widget and layer again
                self.set_widgets.update(|widgets| {
                    widgets.retain(|w| w.id != widget.id);
                });
                self.set_layers.update(|layers| {
                    layers.retain(|l| l.widget_id != widget.id);
                });
                log::info!("Redo: Removed widget {}", widget.id);
            }

            Command::UpdateWidget { widget_id, new_state, .. } => {
                // Redo: Apply new state
                self.set_widgets.update(|widgets| {
                    if let Some(widget) = widgets.iter_mut().find(|w| w.id == widget_id) {
                        *widget = (*new_state).clone();
                    }
                });
                log::info!("Redo: Updated widget {}", widget_id);
            }

            Command::MoveWidget { widget_id, new_position, .. } => {
                // Redo: Apply new position
                self.set_widgets.update(|widgets| {
                    if let Some(widget) = widgets.iter_mut().find(|w| w.id == widget_id) {
                        widget.grid_position = new_position;
                    }
                });
                log::info!("Redo: Moved widget {}", widget_id);
            }

            Command::UpdateDataMapping { widget_id, new_mapping, .. } => {
                // Redo: Apply new mapping
                self.set_widgets.update(|widgets| {
                    if let Some(widget) = widgets.iter_mut().find(|w| w.id == widget_id) {
                        widget.chart_config.data_mapping = new_mapping.as_ref().clone();
                    }
                });
                log::info!("Redo: Updated mapping for widget {}", widget_id);
            }

            Command::Batch { commands, .. } => {
                // Redo batch in original order
                for cmd in commands {
                    self.execute_redo_command(cmd);
                }
            }
        }
    }

    // ===== Initialization =====

    /// Initialize context with mock data (for development/testing)
    pub fn init_with_mock_data(&self) {
        // Set title for mock dashboard
        self.set_title.set(String::from("Q3 Financial Analysis"));
        self.set_last_edited.set(String::from("Last edited 5 mins ago"));

        // Mock datasets
        let datasets = vec![
            Dataset {
                id: "ds_q3_sales".into(),
                name: "Q3_Sales_Data.csv".into(),
                size: "2.4 MB".into(),
                uploaded_at: "Today".into(),
                active: true,
                fields: vec![
                    Field {
                        name: "Product Category".into(),
                        field_type: FieldType::Text,
                    },
                    Field {
                        name: "Total Revenue".into(),
                        field_type: FieldType::Numeric,
                    },
                    Field {
                        name: "Units Sold".into(),
                        field_type: FieldType::Numeric,
                    },
                    Field {
                        name: "Date".into(),
                        field_type: FieldType::Date,
                    },
                ],
                data: Vec::new(),
            },
            Dataset {
                id: "ds_churn".into(),
                name: "Customer_Churn.csv".into(),
                size: "1.1 MB".into(),
                uploaded_at: "Yesterday".into(),
                active: false,
                fields: vec![],
                data: Vec::new(),
            },
        ];

        // Mock widgets
        let widgets = vec![
            Widget {
                id: "widget_revenue_trend".into(),
                title: "Revenue Trend".into(),
                subtitle: Some("Monthly performance vs Target".into()),
                widget_type: WidgetType::Line,
                chart_config: ChartConfig {
                    chart_type: Some(WidgetType::Line),
                    data_mapping: DataMapping {
                        x_axis: Some("Date".into()),
                        y_axis: vec!["Total Revenue".into()],
                        category: None,
                        size: None,
                        color: None,
                        open: None,
                        close: None,
                        high: None,
                        low: None,
                        hierarchy: Vec::new(),
                        columns: Vec::new(),
                        kpi_field: None,
                        kpi_aggregation: None,
                    },
                    style_options: "{}".into(),
                },
                grid_position: GridPosition {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 4,
                },
                editing: false,
            },
            Widget {
                id: "widget_total_profit".into(),
                title: "Total Profit".into(),
                subtitle: Some("vs $75,000 last month".into()),
                widget_type: WidgetType::Kpi,
                chart_config: ChartConfig {
                    chart_type: Some(WidgetType::Kpi),
                    data_mapping: DataMapping::default(),
                    style_options: "{}".into(),
                },
                grid_position: GridPosition {
                    x: 8,
                    y: 0,
                    width: 4,
                    height: 4,
                },
                editing: false,
            },
            Widget {
                id: "widget_market_share".into(),
                title: "Market Share".into(),
                subtitle: None,
                widget_type: WidgetType::Pie,
                chart_config: ChartConfig {
                    chart_type: Some(WidgetType::Pie),
                    data_mapping: DataMapping {
                        x_axis: Some("Product Category".into()),
                        y_axis: vec!["Total Revenue".into()],
                        category: None,
                        size: None,
                        color: None,
                        open: None,
                        close: None,
                        high: None,
                        low: None,
                        hierarchy: Vec::new(),
                        columns: Vec::new(),
                        kpi_field: None,
                        kpi_aggregation: None,
                    },
                    style_options: "{}".into(),
                },
                grid_position: GridPosition {
                    x: 0,
                    y: 4,
                    width: 4,
                    height: 5,
                },
                editing: true,
            },
        ];

        // Mock layers
        let layers = vec![
            Layer {
                id: "layer_revenue".into(),
                widget_id: "widget_revenue_trend".into(),
                label: "Revenue Trend".into(),
                icon: "show-chart".into(),
                visible: true,
                locked: false,
            },
            Layer {
                id: "layer_profit".into(),
                widget_id: "widget_total_profit".into(),
                label: "Total Profit".into(),
                icon: "monitoring".into(),
                visible: true,
                locked: false,
            },
            Layer {
                id: "layer_market".into(),
                widget_id: "widget_market_share".into(),
                label: "Market Share".into(),
                icon: "pie-chart".into(),
                visible: true,
                locked: false,
            },
        ];

        // Mock templates (legacy method, for development only)
        use crate::features::dashboard::models::TemplateCategory;

        let templates = vec![
            Template {
                id: "tpl_revenue_growth".into(),
                title: "Revenue Growth".into(),
                category: TemplateCategory::Business,
                preview_type: WidgetType::Bar,
                widgets: vec![],
                default_config: ChartConfig {
                    chart_type: Some(WidgetType::Bar),
                    data_mapping: DataMapping::default(),
                    style_options: "{}".into(),
                },
            },
            Template {
                id: "tpl_kpi_dashboard".into(),
                title: "KPI Dashboard".into(),
                category: TemplateCategory::Business,
                preview_type: WidgetType::Kpi,
                widgets: vec![],
                default_config: ChartConfig {
                    chart_type: Some(WidgetType::Kpi),
                    data_mapping: DataMapping::default(),
                    style_options: "{}".into(),
                },
            },
        ];

        // Set all data
        self.set_datasets.set(datasets);
        self.set_widgets.set(widgets);
        self.set_layers.set(layers);
        self.set_templates.set(templates);
        self.set_active_dataset_id.set(Some("ds_q3_sales".into()));
    }

    /// Initialize context with empty state (no mock data)
    ///
    /// This is the production-ready initialization that starts with a clean slate.
    /// Users must upload their own CSV files.
    ///
    /// Auto-loads the most recent dashboard from localStorage if available.
    pub fn init_empty(&self) {
        // Try to load the most recent dashboard from localStorage
        if let Ok(templates_list) = crate::features::dashboard::io::storage::list_templates()
            && let Some(most_recent_key) = templates_list.into_iter().max()
        {
            log::info!("Found saved dashboard: {}", most_recent_key);
            self.load_from_storage(&most_recent_key);
            return;
        }

        log::info!("No saved dashboard found, initializing empty");

        // Set default title for new dashboard
        self.set_title.set(String::from("Untitled Dashboard"));
        self.set_last_edited.set(String::from("Last edited just now"));

        // Empty datasets - users will upload their own
        let datasets = vec![];

        // Empty widgets - users will create their own
        let widgets = vec![];

        // Empty layers - no widgets yet
        let layers = vec![];

        // Templates with categories and multi-widget layouts
        use crate::features::dashboard::models::{TemplateCategory, TemplateWidget};

        let templates = vec![
            // ===== GENERIC TEMPLATES (Single widget types) =====
            Template {
                id: "tpl_bar_chart".into(),
                title: "Bar Chart".into(),
                category: TemplateCategory::Generic,
                preview_type: WidgetType::Bar,
                widgets: vec![],
                default_config: ChartConfig {
                    chart_type: Some(WidgetType::Bar),
                    data_mapping: DataMapping::default(),
                    style_options: "{}".into(),
                },
            },
            Template {
                id: "tpl_line_chart".into(),
                title: "Line Chart".into(),
                category: TemplateCategory::Generic,
                preview_type: WidgetType::Line,
                widgets: vec![],
                default_config: ChartConfig {
                    chart_type: Some(WidgetType::Line),
                    data_mapping: DataMapping::default(),
                    style_options: "{}".into(),
                },
            },
            Template {
                id: "tpl_pie_chart".into(),
                title: "Pie Chart".into(),
                category: TemplateCategory::Generic,
                preview_type: WidgetType::Pie,
                widgets: vec![],
                default_config: ChartConfig {
                    chart_type: Some(WidgetType::Pie),
                    data_mapping: DataMapping::default(),
                    style_options: "{}".into(),
                },
            },
            Template {
                id: "tpl_kpi_card".into(),
                title: "KPI Card".into(),
                category: TemplateCategory::Generic,
                preview_type: WidgetType::Kpi,
                widgets: vec![],
                default_config: ChartConfig {
                    chart_type: Some(WidgetType::Kpi),
                    data_mapping: DataMapping::default(),
                    style_options: "{}".into(),
                },
            },

            // ===== BUSINESS TEMPLATES (Multi-widget layouts) =====
            Template {
                id: "tpl_business_overview".into(),
                title: "Business Overview".into(),
                category: TemplateCategory::Business,
                preview_type: WidgetType::Kpi,
                widgets: vec![
                    // KPI Cards Row (top)
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Total Revenue".into(),
                        grid_position: GridPosition { x: 0, y: 0, width: 3, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Total Orders".into(),
                        grid_position: GridPosition { x: 3, y: 0, width: 3, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Avg Order Value".into(),
                        grid_position: GridPosition { x: 6, y: 0, width: 3, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Growth Rate".into(),
                        grid_position: GridPosition { x: 9, y: 0, width: 3, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    // Charts Row (middle)
                    TemplateWidget {
                        widget_type: WidgetType::Bar,
                        title: "Revenue by Category".into(),
                        grid_position: GridPosition { x: 0, y: 2, width: 6, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Bar),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Pie,
                        title: "Sales Distribution".into(),
                        grid_position: GridPosition { x: 6, y: 2, width: 6, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Pie),
                    },
                    // Trend Chart (bottom)
                    TemplateWidget {
                        widget_type: WidgetType::Line,
                        title: "Revenue Trend".into(),
                        grid_position: GridPosition { x: 0, y: 6, width: 12, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Line),
                    },
                ],
                default_config: ChartConfig::default(),
            },

            // ===== SALES TEMPLATES =====
            Template {
                id: "tpl_sales_dashboard".into(),
                title: "Sales Performance".into(),
                category: TemplateCategory::Sales,
                preview_type: WidgetType::Bar,
                widgets: vec![
                    // Sales KPIs
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Monthly Sales".into(),
                        grid_position: GridPosition { x: 0, y: 0, width: 4, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Deals Closed".into(),
                        grid_position: GridPosition { x: 4, y: 0, width: 4, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Win Rate".into(),
                        grid_position: GridPosition { x: 8, y: 0, width: 4, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    // Sales Charts
                    TemplateWidget {
                        widget_type: WidgetType::Bar,
                        title: "Sales by Rep".into(),
                        grid_position: GridPosition { x: 0, y: 2, width: 6, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Bar),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Line,
                        title: "Sales Pipeline".into(),
                        grid_position: GridPosition { x: 6, y: 2, width: 6, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Line),
                    },
                    // Top Products
                    TemplateWidget {
                        widget_type: WidgetType::Bar,
                        title: "Top Products".into(),
                        grid_position: GridPosition { x: 0, y: 6, width: 12, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Bar),
                    },
                ],
                default_config: ChartConfig::default(),
            },

            // ===== FINANCE TEMPLATES =====
            Template {
                id: "tpl_finance_dashboard".into(),
                title: "Financial Overview".into(),
                category: TemplateCategory::Finance,
                preview_type: WidgetType::Line,
                widgets: vec![
                    // Finance KPIs
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Total Revenue".into(),
                        grid_position: GridPosition { x: 0, y: 0, width: 4, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Total Expenses".into(),
                        grid_position: GridPosition { x: 4, y: 0, width: 4, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Kpi,
                        title: "Net Profit".into(),
                        grid_position: GridPosition { x: 8, y: 0, width: 4, height: 2 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Kpi),
                    },
                    // Financial Charts
                    TemplateWidget {
                        widget_type: WidgetType::Line,
                        title: "Revenue vs Expenses".into(),
                        grid_position: GridPosition { x: 0, y: 2, width: 12, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Line),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Pie,
                        title: "Expense Breakdown".into(),
                        grid_position: GridPosition { x: 0, y: 6, width: 6, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Pie),
                    },
                    TemplateWidget {
                        widget_type: WidgetType::Bar,
                        title: "Budget vs Actual".into(),
                        grid_position: GridPosition { x: 6, y: 6, width: 6, height: 4 },
                        chart_config: ChartConfig::default_with_type(WidgetType::Bar),
                    },
                ],
                default_config: ChartConfig::default(),
            },
        ];

        // Set all data
        self.set_datasets.set(datasets);
        self.set_widgets.set(widgets);
        self.set_layers.set(layers);
        self.set_templates.set(templates);
        self.set_active_dataset_id.set(None);
    }
}
