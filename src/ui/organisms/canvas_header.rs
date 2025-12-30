use crate::features::dashboard::context::DashboardContext;
use crate::ui::atoms::{Badge, BadgeSize, BadgeVariant, Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Canvas header organism component
///
/// Top bar of the main canvas area showing the dashboard title,
/// auto-save status, and action buttons (undo/redo, grid view, add widget).
///
/// # Example
/// ```rust
/// view! {
///     <CanvasHeader
///         title=dashboard.title
///         last_edited=dashboard.last_edited
///         auto_saved=dashboard.auto_saved
///     />
/// }
/// ```
#[component]
pub fn CanvasHeader(
    /// Dashboard title (reactive signal)
    title: ReadSignal<String>,
    /// Last edited timestamp text (reactive signal)
    last_edited: ReadSignal<String>,
    /// Whether to show the "Auto-saved" badge (reactive signal)
    auto_saved: ReadSignal<bool>,
) -> impl IntoView {
    // Get dashboard context
    let dashboard = DashboardContext::use_context();

    // Action button handlers
    let on_undo = move |_: MouseEvent| {
        dashboard.undo();
    };

    let on_redo = move |_: MouseEvent| {
        dashboard.redo();
    };

    let on_grid_view = move |_: MouseEvent| {
        dashboard
            .set_grid_view_active
            .update(|active| *active = !*active);
        log::info!("Grid view toggled: {}", dashboard.grid_view_active.get());
    };

    // Local state for title editing
    let (is_editing, set_is_editing) = signal(false);
    let (edit_value, set_edit_value) = signal(String::new());

    let on_double_click_title = move |_| {
        set_edit_value.set(title.get());
        set_is_editing.set(true);
    };

    let save_title = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            dashboard.set_dashboard_title(edit_value.get());
            set_is_editing.set(false);
        }
    };

    let on_blur = move |_| {
        dashboard.set_dashboard_title(edit_value.get());
        set_is_editing.set(false);
    };

    view! {
        <div class="h-14 border-b border-base-300
        bg-base-100/80 flex items-center justify-between
        px-6 backdrop-blur-md z-10 sticky top-0">

            // Left side: Title + subtitle + auto-save badge
            <div class="flex items-center gap-4">
                <div class="flex flex-col">
                    {move || {
                        if is_editing.get() {
                            view! {
                                <input
                                    type="text"
                                    prop:value=edit_value
                                    on:input=move |ev| {
                                        set_edit_value.set(event_target_value(&ev));
                                    }
                                    on:keydown=save_title
                                    on:blur=on_blur
                                    class="text-sm font-bold text-base-content
                                    bg-base-200 border border-primary rounded px-2 py-0.5
                                    focus:outline-none focus:ring-2 focus:ring-primary/50"
                                    autofocus
                                />
                            }.into_any()
                        } else {
                            view! {
                                <h1
                                    class="text-sm font-bold text-base-content
                                    cursor-pointer hover:text-primary transition-colors"
                                    on:dblclick=on_double_click_title
                                    title="Double-click to edit"
                                >
                                    {move || title.get()}
                                </h1>
                            }.into_any()
                        }
                    }}
                    <p class="text-[10px] text-base-content/60">{move || last_edited.get()}</p>
                </div>

                {move || {
                    if auto_saved.get() {
                        view! {
                            <Badge
                                variant=BadgeVariant::Success
                                size=BadgeSize::Small
                                class="uppercase tracking-wide"
                            >
                                "Auto-saved"
                            </Badge>
                        }
                            .into_any()
                    } else {
                        ().into_any()
                    }
                }}
            </div>

            // Right side: Actions
            <div class="flex items-center gap-3">
                // Undo/Redo button group
                <div class="flex bg-base-200 rounded-lg p-0.5">
                    <button
                        class=move || {
                            format!(
                                "p-1.5 rounded-md transition-all {}",
                                if dashboard.can_undo.get() {
                                    "hover:bg-base-100 shadow-none hover:shadow-sm text-base-content/60 cursor-pointer"
                                } else {
                                    "text-base-content/20 cursor-not-allowed"
                                }
                            )
                        }
                        title=move || {
                            if dashboard.can_undo.get() { "Undo" } else { "Nothing to undo" }
                        }
                        on:click=on_undo
                        disabled=move || !dashboard.can_undo.get()
                    >
                        <Icon name=IconName::Undo class="w-[18px] h-[18px]" />
                    </button>
                    <button
                        class=move || {
                            format!(
                                "p-1.5 rounded-md transition-all {}",
                                if dashboard.can_redo.get() {
                                    "hover:bg-base-100 shadow-none hover:shadow-sm text-base-content/60 cursor-pointer"
                                } else {
                                    "text-base-content/20 cursor-not-allowed"
                                }
                            )
                        }
                        title=move || {
                            if dashboard.can_redo.get() { "Redo" } else { "Nothing to redo" }
                        }
                        on:click=on_redo
                        disabled=move || !dashboard.can_redo.get()
                    >
                        <Icon name=IconName::Redo class="w-[18px] h-[18px]" />
                    </button>
                </div>

                // Vertical divider
                <div class="w-px h-6 bg-base-300"></div>

                // Grid view toggle
                <button
                    class=move || {
                        format!(
                            "flex items-center justify-center w-9 h-9 rounded-lg transition-all \
                             hover:-translate-y-0.5 hover:shadow-md {}",
                            if dashboard.grid_view_active.get() {
                                "bg-primary text-primary-content shadow-lg shadow-primary/25"
                            } else {
                                "hover:bg-base-200 text-base-content/60"
                            },
                        )
                    }
                    title="Grid View"
                    on:click=on_grid_view
                >
                    <Icon name=IconName::GridView class="w-5 h-5" />
                </button>
            </div>
        </div>
    }
}
