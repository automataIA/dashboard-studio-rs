use crate::ui::atoms::{Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Dataset item component for sidebar
///
/// Displays a dataset row with icon, name, metadata, and optional delete button.
/// Shows different styling for active state.
///
/// # Example
/// ```rust
/// let on_delete = |ev: MouseEvent| {
///     // Handle delete
/// };
///
/// view! {
///     <DatasetItem
///         name="Q3_Sales_Data.csv".into()
///         metadata="2.4 MB • Today".into()
///         active=true
///         show_delete=true
///         on_delete=Some(on_delete)
///     />
/// }
/// ```
#[component]
pub fn DatasetItem(
    /// Dataset display name
    #[prop(into)]
    name: String,
    /// Metadata text (size, date, etc.)
    #[prop(into)]
    metadata: String,
    /// Whether this dataset is currently active/selected
    #[prop(optional)]
    active: bool,
    /// Whether to show the delete button on hover
    #[prop(optional)]
    show_delete: bool,
    /// Optional delete handler
    #[prop(optional)]
    on_delete: Option<Callback<MouseEvent>>,
    /// Optional click handler (for dataset activation)
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let delete_handler = move |ev: MouseEvent| {
        ev.stop_propagation();
        if let Some(cb) = &on_delete {
            cb.run(ev);
        }
    };

    let click_handler = move |ev: MouseEvent| {
        if let Some(cb) = &on_click {
            cb.run(ev);
        }
    };

    let container_classes = move || {
        if active {
            "flex items-center gap-3 px-3 py-2 rounded-lg bg-primary/5 border \
                border-primary/20 cursor-pointer relative group transition-all"
        } else {
            "flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-base-100 \
                cursor-pointer group transition-colors border border-transparent hover:border-base-300"
        }
    };

    view! {
        <div class=container_classes on:click=click_handler>
            {if active {
                view! {
                    <div class="absolute left-0 top-1.5 bottom-1.5 w-0.5 bg-primary rounded-r"></div>
                }
                    .into_any()
            } else {
                ().into_any()
            }}

            {if active {
                view! { <Icon name=IconName::Table class="text-primary text-[20px]" /> }.into_any()
            } else {
                view! {
                    <Icon
                        name=IconName::Table
                        class="text-base-content/40 text-[20px] group-hover:text-base-content/70"
                    />
                }
                    .into_any()
            }}

            <div class="flex-1 min-w-0">
                <p class=move || {
                    if active {
                        "text-base-content text-sm font-medium truncate"
                    } else {
                        "text-base-content/60 group-hover:text-base-content \
                            text-sm font-medium truncate transition-colors"
                    }
                }>{name}</p>
                <p class="text-base-content/50 text-[10px] mt-0.5">{metadata}</p>
            </div>

            {if show_delete {
                view! {
                    <button
                        class="text-slate-400 hover:text-red-500 opacity-0 group-hover:opacity-100 transition-all"
                        on:click=delete_handler
                    >
                        <Icon name=IconName::Delete class="w-4 h-4" />
                    </button>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
