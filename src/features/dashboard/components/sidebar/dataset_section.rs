use crate::features::dashboard::csv_upload::CsvUploadManager;
use crate::features::dashboard::DashboardContext;
use crate::ui::atoms::{Icon, IconName};
use crate::ui::molecules::{DatasetItem, UploadZone};
use crate::ui::organisms::data::DatasetItemData;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Dataset section component
///
/// Displays CSV upload zone and list of uploaded datasets with activation.
///
/// # Example
/// ```rust
/// view! {
///     <DatasetSection
///         datasets=datasets
///         upload_manager=Some(upload_manager)
///     />
/// }
/// ```
#[component]
pub fn DatasetSection(
    /// List of datasets to display (reactive)
    #[prop(into)]
    datasets: Signal<Vec<DatasetItemData>>,
    /// CSV Upload Manager for handling uploads
    upload_manager: Option<CsvUploadManager>,
) -> impl IntoView {
    // Get dashboard context for dataset activation
    let dashboard = DashboardContext::use_context();

    view! {
        <div class="flex flex-col gap-3">
            // Section header
            <div class="flex items-center justify-between px-1">
                <h3 class="text-xs font-bold text-base-content/60 uppercase tracking-wider">
                    "Datasets"
                </h3>
                <button
                    class="text-base-content/40 hover:text-primary transition-colors"
                    title="Add dataset"
                >
                    <Icon name=IconName::Add class="w-4 h-4" />
                </button>
            </div>

            // Upload Zone
            {move || {
                upload_manager.as_ref().map(|manager| {
                    view! {
                        <UploadZone
                            title=String::from("Upload CSV")
                            subtitle=String::from("or drag and drop")
                            upload_manager=Some(*manager)
                        />
                    }
                })
            }}

            // Dataset items list
            <div class="flex flex-col gap-1 mt-1">
                {move || {
                    datasets
                        .get()
                        .into_iter()
                        .map(|dataset| {
                            let dataset_id = dataset.id.clone();
                            let on_delete = Callback::new(move |_: MouseEvent| {
                                // TODO: Delete dataset
                            });
                            let on_click = Callback::new(move |_: MouseEvent| {
                                dashboard.set_active_dataset(Some(dataset_id.clone()));
                            });

                            view! {
                                <div class="flex flex-col gap-0.5">
                                    <DatasetItem
                                        name=dataset.name.clone()
                                        metadata=dataset.metadata.clone()
                                        active=dataset.active
                                        show_delete=dataset.active
                                        on_delete=on_delete
                                        on_click=on_click
                                    />
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
