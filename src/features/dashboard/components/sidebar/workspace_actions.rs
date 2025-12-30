use crate::context::ToastContext;
use crate::features::dashboard::{DashboardContext, ExportService, TemplateType};
use crate::ui::atoms::Icon;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;

/// Workspace actions component for Dashboard export/import
///
/// Provides card-based interface for saving and loading complete dashboards
/// (configuration + data snapshots).
///
/// # Example
/// ```rust
/// view! {
///     <WorkspaceActions />
/// }
/// ```
#[component]
pub fn WorkspaceActions() -> impl IntoView {
    let dashboard = DashboardContext::use_context();
    let toast = ToastContext::use_context();

    // Export dashboard (config + data)
    let on_export_dashboard = move |_: MouseEvent| {
        log::info!("Exporting complete dashboard");

        match ExportService::export_dashboard(&dashboard, TemplateType::Complete) {
            Ok(filename) => {
                log::info!("Export successful: {}", filename);
                toast.show_success(
                    "Dashboard Exported!",
                    &format!("Saved as {}", filename),
                );
            }
            Err(e) => {
                log::error!("Export failed: {}", e);
                toast.show_error(
                    "Export Failed",
                    &format!("Could not export: {}", e),
                );
            }
        }
    };

    // Import dashboard - trigger file input click
    let on_import_click = move |_: MouseEvent| {
        log::info!("Import clicked - opening file picker");
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(input) = document
                    .get_element_by_id("import-dashboard-file-input")
                    .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                } else {
                    log::error!("Failed to find import file input");
                    toast.show_error("Error", "File input not found");
                }
            }
        }
    };

    // File change handler - process selected file
    let on_file_change = move |ev: Event| {
        let target = ev.target().unwrap();
        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        if let Some(files) = input.files() && files.length() > 0 {
            let file = files.get(0).unwrap();
            let filename = file.name();

            log::info!("File selected for import: {}", filename);

            // Read file content using FileReader with Closure
            let reader = web_sys::FileReader::new().unwrap();
            let file_clone = file.clone();
            let dashboard_clone = dashboard;
            let toast_clone = toast;
            let filename_clone = filename.clone();

            // Create closure for onload event
            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |ev: Event| {
                if let Some(target) = ev.target() {
                    if let Some(reader) = target.dyn_ref::<web_sys::FileReader>() {
                        if let Some(content) = reader.result().ok().and_then(|v| v.as_string()) {
                            log::info!("File loaded, {} bytes", content.len());

                            // Import dashboard
                            match ExportService::import_dashboard(&dashboard_clone, &content, &filename_clone) {
                                Ok(()) => {
                                    log::info!("Import successful");
                                    toast_clone.show_success(
                                        "Dashboard Imported!",
                                        &format!("Loaded '{}'", filename_clone),
                                    );
                                }
                                Err(e) => {
                                    log::error!("Import failed: {}", e);
                                    toast_clone.show_error(
                                        "Import Failed",
                                        &format!("Could not import: {}", e),
                                    );
                                }
                            }
                        }
                    }
                }
            }) as Box<dyn Fn(_)>);

            // Add event listener
            reader
                .add_event_listener_with_callback("load", onload.as_ref().unchecked_ref())
                .unwrap();

            onload.forget(); // Keep listener alive
            reader.read_as_text(&file_clone).unwrap();
        }
    };

    view! {
        <div class="
            flex flex-col gap-3
            p-4 rounded-xl
            bg-gradient-to-br from-base-200/50 to-base-300/30
            border border-base-300
        ">
            // Section header
            <div class="flex items-center gap-2">
                <div class="
                    flex items-center justify-center
                    w-8 h-8 rounded-lg
                    bg-gradient-to-br from-secondary/20 to-accent/20
                ">
                    <Icon name=crate::ui::atoms::IconName::Copy class="w-4 h-4 text-secondary" />
                </div>
                <div class="flex flex-col">
                    <h3 class="text-xs font-bold text-base-content uppercase tracking-wide">
                        "Workspace"
                    </h3>
                    <p class="text-[10px] text-base-content/60">
                        "Save & load dashboards"
                    </p>
                </div>
            </div>

            // Action buttons
            <div class="flex flex-col gap-2">
                // Export Dashboard button
                <button
                    class="
                        btn btn-sm
                        gap-2
                        bg-secondary/10 hover:bg-secondary/20
                        text-secondary hover:text-secondary-focus
                        border-secondary/20 hover:border-secondary/30
                        rounded-lg transition-all
                    "
                    on:click=on_export_dashboard
                >
                    <Icon name=crate::ui::atoms::IconName::Download class="w-4 h-4" />
                    <span class="text-xs font-semibold">"Export Dashboard"</span>
                </button>

                // Import Dashboard button
                <button
                    class="
                        btn btn-sm
                        gap-2
                        bg-accent/10 hover:bg-accent/20
                        text-accent hover:text-accent-focus
                        border-accent/20 hover:border-accent/30
                        rounded-lg transition-all
                    "
                    on:click=on_import_click
                >
                    <Icon name=crate::ui::atoms::IconName::Upload class="w-4 h-4" />
                    <span class="text-xs font-semibold">"Import Dashboard"</span>
                </button>
            </div>

            // Helper text
            <p class="text-[10px] text-base-content/50 leading-relaxed">
                "Dashboards include configuration and data snapshots"
            </p>

            // Hidden file input for import
            <input
                type="file"
                id="import-dashboard-file-input"
                accept="application/json"
                class="hidden"
                on:change=on_file_change
            />
        </div>
    }
}
