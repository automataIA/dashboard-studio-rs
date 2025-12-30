use crate::features::dashboard::csv_upload::CsvUploadManager;
use crate::ui::atoms::{Button, ButtonSize, ButtonVariant, Divider, Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

use super::dataset_section::DatasetSection;
use super::export_actions::ExportActions;
use super::template_section::TemplateSection;
use super::workspace_actions::WorkspaceActions;
use crate::ui::organisms::data::DatasetItemData;
use crate::ui::organisms::data::TemplateData;

/// Left sidebar organism component
///
/// Main left panel containing datasets and templates sections.
/// Composes DatasetSection, TemplateSection, ExportActions, and documentation footer.
///
/// # Example
/// ```rust
/// let datasets = signal(vec![...]);
/// let templates = signal(vec![...]);
///
/// view! {
///     <LeftSidebar
///         datasets=datasets
///         templates=templates
///         upload_manager=Some(upload_manager)
///     />
/// }
/// ```
#[component]
pub fn LeftSidebar(
    /// List of datasets to display (reactive)
    #[prop(into)]
    datasets: Signal<Vec<DatasetItemData>>,
    /// List of templates to display (reactive)
    #[prop(into)]
    templates: Signal<Vec<TemplateData>>,
    /// CSV Upload Manager for handling uploads
    #[prop(optional)]
    upload_manager: Option<CsvUploadManager>,
) -> impl IntoView {
    // Template click is handled internally by TemplateSection
    // No need to expose it to parent

    // Documentation button handler
    let on_doc_click = Callback::new(|_: MouseEvent| {
        // TODO: Open documentation
        log::info!("Documentation clicked");
    });

    view! {
        <div class="p-4 flex flex-col gap-6 h-full">
            // Datasets Section
            <DatasetSection
                datasets=datasets
                upload_manager=upload_manager
            />

            // Divider
            <Divider class="h-px bg-base-300 w-full" />

            // Templates Section - Card-based design
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
                        bg-gradient-to-br from-primary/20 to-accent/20
                    ">
                        <Icon name=IconName::GridView class="w-4 h-4 text-primary" />
                    </div>
                    <div class="flex flex-col flex-1">
                        <h3 class="text-xs font-bold text-base-content uppercase tracking-wide">
                            "Templates"
                        </h3>
                        <p class="text-[10px] text-base-content/60">
                            "Browse & apply templates"
                        </p>
                    </div>
                </div>

                // Template library with categories and KPI Quick Actions
                <TemplateSection templates=templates />

                // Template export action - positioned after browsing
                <div class="pt-2 border-t border-base-300">
                    <ExportActions />
                </div>
            </div>

            // Divider between sections
            <Divider class="h-px bg-base-300 w-full" />

            // Workspace Section - Dashboard export/import
            <WorkspaceActions />

            // Spacer to push footer down
            <div class="flex-1"></div>

            // Footer
            <div class="mt-auto">
                <Button
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::Medium
                    on_click=on_doc_click
                    class="w-full items-center gap-3 justify-start"
                >
                    <Icon name=IconName::MenuBook class="w-5 h-5" />
                    <span class="text-sm font-medium">"Documentation"</span>
                </Button>
            </div>
        </div>
    }
}
