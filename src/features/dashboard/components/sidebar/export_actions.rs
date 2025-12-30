use crate::context::ToastContext;
use crate::features::dashboard::{DashboardContext, ExportService, TemplateType};
use crate::ui::atoms::Icon;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Template export action component
///
/// Provides a simple button to export current dashboard as a reusable template
/// (configuration only, no data).
///
/// # Example
/// ```rust
/// view! {
///     <ExportActions />
/// }
/// ```
#[component]
pub fn ExportActions() -> impl IntoView {
    let dashboard = DashboardContext::use_context();
    let toast = ToastContext::use_context();

    // Export as template (config only)
    let on_export_template = move |_: MouseEvent| {
        log::info!("Exporting dashboard as template (config only)");

        match ExportService::export_dashboard(&dashboard, TemplateType::Generic) {
            Ok(filename) => {
                log::info!("Export successful: {}", filename);
                toast.show_success(
                    "Template Exported!",
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

    view! {
        <button
            class="
                btn btn-sm
                w-full justify-center
                gap-2 h-9
                bg-primary/10 hover:bg-primary/20
                text-primary hover:text-primary-focus
                border-primary/20 hover:border-primary/30
                rounded-lg transition-all
            "
            on:click=on_export_template
        >
            <Icon name=crate::ui::atoms::IconName::Download class="w-4 h-4" />
            <span class="text-xs font-semibold">"Save as Template"</span>
        </button>
    }
}
