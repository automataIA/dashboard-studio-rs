use crate::features::dashboard::DragDropManager;
use crate::ui::atoms::{Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Widget card component
///
/// Reusable container for dashboard widgets with header, title,
/// optional editing state, and action menu button.
///
/// # Example
/// ```rust
/// view! {
///     <WidgetCard
///         title="Revenue Trend".into()
///         subtitle="Monthly performance".into()
///         editing=false
///         show_menu=true
///         on_menu_click=Some(cb)
///     >
///         <div class="p-4">
///             <!-- Widget content here -->
///         </div>
///     </WidgetCard>
/// }
/// ```
#[component]
pub fn WidgetCard(
    /// Widget title
    #[prop(into)]
    title: String,
    /// Optional subtitle/description
    #[prop(optional, into)]
    subtitle: Option<String>,
    /// Whether this widget is currently being edited
    #[prop(optional)]
    editing: bool,
    /// Whether to show the menu button (visible on hover)
    #[prop(optional)]
    show_menu: bool,
    /// Optional menu button click handler
    #[prop(optional)]
    on_menu_click: Option<Callback<MouseEvent>>,
    /// Optional widget ID for drag & drop (enables dragging when provided)
    #[prop(optional, into)]
    widget_id: Option<String>,
    /// Widget content
    children: Children,
) -> impl IntoView {
    let menu_handler = move |ev: MouseEvent| {
        if let Some(cb) = &on_menu_click {
            cb.run(ev);
        }
    };

    let container_classes = move || {
        let base = "widget-card bg-base-100 rounded-xl shadow-widget hover:shadow-widget-hover \
            transition-smooth relative flex flex-col gpu-accelerated";

        if editing {
            format!("{} border-2 border-primary ring-4 ring-primary/10", base)
        } else {
            base.to_string()
        }
    };

    let widget_id_for_drag = widget_id.clone();
    let widget_id_for_draggable = widget_id.clone();
    let widget_id_for_dragend = widget_id.clone();

    view! {
        <div
            class=container_classes
            draggable=move || widget_id_for_draggable.is_some()
            on:dragstart=move |ev| {
                if let Some(id) = &widget_id_for_drag {
                    DragDropManager::on_drag_start(id.clone())(ev);
                }
            }
            on:dragend=move |ev| {
                if widget_id_for_dragend.is_some() {
                    DragDropManager::on_drag_end()(ev);
                }
            }
        >
            // Widget Header
            <div class="flex justify-between items-start p-5 pb-2">
                <div>
                    <h3 class="text-base-content font-bold text-sm">{title}</h3>
                    {subtitle
                        .map(|sub| {
                            view! { <p class="text-xs text-base-content/60">{sub}</p> }
                        })}
                </div>

                {if editing {
                    view! {
                        <div class="absolute -top-3 left-1/2 -translate-x-1/2 bg-primary \
                        text-primary-content text-[10px] font-bold px-3 py-1 rounded-full uppercase \
                        tracking-wider shadow-sm">Editing</div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }}

                {if show_menu {
                    view! {
                        <button
                            class="p-1.5 hover:bg-base-200 rounded-lg \
                            text-base-content/60 transition-all"
                            on:click=menu_handler
                        >
                            <Icon name=IconName::MoreHoriz class="w-5 h-5" />
                        </button>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }}
            </div>

            // Widget Content
            {children()}
        </div>
    }
}
