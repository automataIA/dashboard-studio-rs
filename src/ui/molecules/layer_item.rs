use crate::ui::atoms::{Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Widget/Layer item component
///
/// Represents a widget in the Widgets List of the right sidebar.
/// Shows different styling for the currently active widget and includes a delete button.
/// Supports renaming via double-click.
///
/// # Example
/// ```rust
/// let on_click = |ev: MouseEvent| {
///     // Handle widget selection
/// };
///
/// let on_delete = |ev: MouseEvent| {
///     // Handle widget deletion
/// };
///
/// let on_rename = |new_name: String| {
///     // Handle widget rename
/// };
///
/// view! {
///     <LayerItem
///         widget_id="widget_123".into()
///         label="Revenue Trend".into()
///         icon=IconName::ShowChart
///         active=true
///         on_click=Some(on_click)
///         on_delete=Some(on_delete)
///         on_rename=Some(on_rename)
///     />
/// }
/// ```
#[component]
pub fn LayerItem(
    /// Widget ID for renaming
    #[prop(into)]
    _widget_id: String,
    /// Widget/Layer display name
    #[prop(into)]
    label: String,
    /// Icon to represent this widget type
    #[prop(optional)]
    icon: Option<IconName>,
    /// Whether this is the currently active widget
    #[prop(optional)]
    active: bool,
    /// Optional click handler for selecting the widget
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
    /// Optional delete handler for removing the widget
    #[prop(optional)]
    on_delete: Option<Callback<MouseEvent>>,
    /// Optional rename handler
    #[prop(optional)]
    on_rename: Option<Callback<String>>,
) -> impl IntoView {
    // State for editing mode
    let (is_editing, set_is_editing) = signal(false);
    let (edit_value, set_edit_value) = signal(label.clone());

    let click_handler = move |ev: MouseEvent| {
        if !is_editing.get()
            && let Some(cb) = &on_click {
                cb.run(ev);
            }
    };

    // Clone label for use in closures
    let label_for_dblclick = label.clone();
    let double_click_handler = move |ev: MouseEvent| {
        ev.stop_propagation();
        if on_rename.is_some() {
            set_edit_value.set(label_for_dblclick.clone());
            set_is_editing.set(true);
        }
    };

    let delete_handler = move |ev: MouseEvent| {
        ev.stop_propagation(); // Prevent triggering on_click
        if let Some(cb) = &on_delete {
            cb.run(ev);
        }
    };

    let save_rename = move || {
        let new_name = edit_value.get();
        if !new_name.trim().is_empty()
            && let Some(cb) = on_rename {
                cb.run(new_name);
            }
        set_is_editing.set(false);
    };

    let on_input = move |ev: leptos::ev::Event| {
        set_edit_value.set(event_target_value(&ev));
    };

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            save_rename();
        } else if ev.key() == "Escape" {
            set_is_editing.set(false);
        }
    };

    let on_blur = move |_| {
        save_rename();
    };

    let container_classes = move || {
        if active {
            "flex items-center gap-2 px-3 py-2 rounded-lg bg-primary/5 \
                border border-primary/20 cursor-pointer relative group transition-colors"
        } else {
            "flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-base-200 \
                cursor-pointer group transition-colors"
        }
    };

    view! {
        <div class=container_classes on:click=click_handler on:dblclick=double_click_handler>
            {if active {
                view! {
                    <div class="absolute left-0 top-2 bottom-2 w-0.5 bg-primary rounded-r"></div>
                }
                    .into_any()
            } else {
                ().into_any()
            }}

            {match icon {
                Some(icon_name) => {
                    view! { <Icon name=icon_name class="w-[18px] h-[18px]" /> }.into_any()
                }
                None => ().into_any(),
            }}

            // Label or input for editing
            {move || {
                if is_editing.get() {
                    view! {
                        <input
                            type="text"
                            prop:value=edit_value
                            on:input=on_input
                            on:keydown=on_keydown
                            on:blur=on_blur
                            class="text-sm flex-1 bg-base-200 border border-primary rounded px-2 py-0.5
                            focus:outline-none focus:ring-2 focus:ring-primary/50"
                            autofocus
                        />
                    }
                        .into_any()
                } else {
                    view! { <span class="text-sm flex-1 truncate">{label.clone()}</span> }
                        .into_any()
                }
            }}

            // Delete button - always visible when on_delete is provided
            {match on_delete {
                Some(_) => {
                    view! {
                        <button
                            class="btn btn-xs btn-error btn-outline opacity-0 group-hover:opacity-100 transition-all"
                            on:click=delete_handler
                            title="Delete widget"
                        >
                            <Icon name=IconName::Delete class="w-3.5 h-3.5" />
                        </button>
                    }
                        .into_any()
                }
                None => ().into_any(),
            }}
        </div>
    }
}
