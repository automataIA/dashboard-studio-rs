use leptos::prelude::*;
use leptos::ev::{MouseEvent, DragEvent};
use wasm_bindgen::JsCast;
use crate::ui::atoms::{Icon, IconName};
use crate::features::dashboard::csv_upload::{CsvUploadManager, UploadState};

/// Upload zone component with dashed border
///
/// A drag-and-drop upload area with dashed border, used for uploading
/// CSV files in the sidebar. Supports progress tracking and error display.
///
/// # Example
/// ```rust
/// // With upload manager (recommended)
/// view! {
///     <UploadZone
///         title="Upload CSV".into()
///         subtitle="or drag and drop".into()
///         upload_manager=Some(upload_manager)
///     />
/// }
///
/// // With simple click handler
/// let on_upload = |ev: MouseEvent| {
///     // Handle upload click
/// };
///
/// view! {
///     <UploadZone
///         title="Upload CSV".into()
///         subtitle="or drag and drop".into()
///         on_click=Some(on_upload)
///     />
/// }
/// ```
#[component]
pub fn UploadZone(
    /// Main title text
    #[prop(into)]
    title: String,
    /// Subtitle text
    #[prop(into)]
    subtitle: String,
    /// Optional click handler (use upload_manager instead for CSV upload)
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
    /// CSV Upload Manager for handling uploads
    upload_manager: Option<CsvUploadManager>,
) -> impl IntoView {
    let (is_dragging, set_is_dragging) = signal(false);

    let click_handler = {
        let manager = upload_manager;
        move |ev: MouseEvent| {
            if let Some(ref mgr) = manager {
                mgr.select_file();
            } else if let Some(cb) = &on_click {
                cb.run(ev);
            }
        }
    };

    let drag_over_handler = move |ev: DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(true);
    };

    let drag_leave_handler = move |_ev: DragEvent| {
        set_is_dragging.set(false);
    };

    let drop_handler = {
        let manager = upload_manager;
        move |ev: DragEvent| {
            ev.prevent_default();
            set_is_dragging.set(false);

            if let Some(ref mgr) = manager {
                let native_ev: &web_sys::DragEvent = ev.unchecked_ref();
                if let Some(data_transfer) = native_ev.data_transfer()
                    && let Some(files) = data_transfer.files() {
                        mgr.handle_drop(files);
                    }
            }
        }
    };

    // Show upload progress if available (reactive)
    let progress_view = if let Some(manager) = &upload_manager {
        let manager = *manager;
        view! {
            {move || {
                let progress = manager.progress();
                match &progress.state {
                    UploadState::Uploading { progress, .. }
                    | UploadState::Parsing { progress, .. } => {
                        view! {
                            <div class="absolute inset-0 bg-base-100/90 rounded-lg flex flex-col items-center justify-center gap-2">
                                <div class="w-full px-4">
                                    <div class="w-full bg-base-300 rounded-full h-2">
                                        <div
                                            class="bg-primary h-2 rounded-full transition-all duration-300"
                                            style=format!("width: {}%", progress)
                                        ></div>
                                    </div>
                                    <p class="text-xs text-center mt-2 text-base-content/60">
                                        {format!("{:.0}%", progress)}
                                    </p>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                    UploadState::Completed => {
                        view! {
                            <div class="absolute inset-0 bg-success/10 rounded-lg flex flex-col items-center justify-center gap-2">
                                <Icon name=IconName::Check class="w-8 h-8 text-success" />
                            </div>
                        }
                            .into_any()
                    }
                    UploadState::Failed(msg) => {
                        view! {
                            <div class="absolute inset-0 bg-error/10 rounded-lg flex flex-col items-center justify-center gap-2 p-2">
                                <Icon name=IconName::Error class="w-8 h-8 text-error" />
                                <p class="text-xs text-center text-error font-medium">
                                    "Upload Failed"
                                </p>
                                <p class="text-[10px] text-center text-error">{msg.clone()}</p>
                            </div>
                        }
                            .into_any()
                    }
                    _ => ().into_any(),
                }
            }}
        }.into_any()
    } else {
        ().into_any()
    };

    let has_progress_overlay = {
        let manager = upload_manager;
        move || {
            if let Some(mgr) = manager {
                matches!(
                    mgr.progress().state,
                    UploadState::Uploading { .. } | UploadState::Parsing { .. } | UploadState::Completed | UploadState::Failed(_)
                )
            } else {
                false
            }
        }
    };

    view! {
        <div
            class="flex flex-col items-center justify-center gap-2 rounded-lg border-2 \
            border-dashed relative overflow-hidden transition-all cursor-pointer group
            hover:border-primary hover:bg-base-100
            hover:scale-[1.02] active:scale-[0.98]
            border-base-300
            p-5"
            class:drag-over=is_dragging
            class:dragging=is_dragging
            on:click=click_handler
            on:dragover=drag_over_handler
            on:dragleave=drag_leave_handler
            on:drop=drop_handler
        >
            {progress_view}

            <div class=move || {
                if !(has_progress_overlay)() {
                    "flex flex-col items-center gap-2".to_string()
                } else {
                    "hidden".to_string()
                }
            }>
                <div class="p-2.5 rounded-full bg-base-100 text-base-content/40
                group-hover:text-primary
                group-hover:bg-primary/10 transition-colors">
                    <Icon name=IconName::Upload class="w-6 h-6" />
                </div>
                <div class="text-center">
                    <p class="text-base-content text-sm font-medium
                    group-hover:text-primary transition-colors">{title}</p>
                    <p class="text-base-content/50 text-xs">{subtitle}</p>
                </div>
            </div>
        </div>
    }
}
