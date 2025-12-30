//! Column selector molecule
//!
//! Multi-select dropdown with checklist for selecting table columns.
//! Displays all available fields from the dataset with checkboxes.

use crate::features::dashboard::models::Field;
use crate::ui::atoms::{Icon, IconName};
use leptos::prelude::*;

/// Column selector component with checklist
///
/// Shows a dropdown menu with checkboxes for each available column.
/// Allows selecting multiple columns for table display.
///
/// # Example
/// ```rust
/// let fields = vec![
///     Field { name: "Product".to_string(), field_type: FieldType::Text },
///     Field { name: "Revenue".to_string(), field_type: FieldType::Numeric },
/// ];
/// let selected = vec!["Product".to_string()];
///
/// view! {
///     <ColumnSelector
///         fields=fields
///         selected=Signal::derive(move || selected.clone())
///         on_change=Callback::new(|cols: Vec<String>| {
///             // Handle column selection change
///         })
///     />
/// }
/// ```
#[component]
pub fn ColumnSelector(
    /// Available fields to choose from
    #[prop(into)]
    fields: Signal<Vec<Field>>,
    /// Currently selected column names
    #[prop(into)]
    selected: Signal<Vec<String>>,
    /// Callback when selection changes
    on_change: Callback<Vec<String>>,
) -> impl IntoView {
    // Track dropdown open/closed state
    let (is_open, set_is_open) = signal(false);

    // Toggle column selection
    let toggle_column = move |field_name: String| {
        let mut current = selected.get_untracked();
        if current.contains(&field_name) {
            current.retain(|name| name != &field_name);
        } else {
            current.push(field_name);
        }
        on_change.run(current);
    };

    // Select all columns
    let select_all = move || {
        let all_fields: Vec<String> = fields
            .get_untracked()
            .iter()
            .map(|f| f.name.clone())
            .collect();
        on_change.run(all_fields);
    };

    // Deselect all columns
    let deselect_all = move || {
        on_change.run(vec![]);
    };

    // Close dropdown when clicking outside
    let dropdown_ref = NodeRef::<leptos::html::Div>::new();

    view! {
        <div class="flex flex-col gap-2">
            <label class="text-xs font-semibold text-base-content/70">"Columns to Display"</label>

            <div class="relative" node_ref=dropdown_ref>
                // Dropdown trigger button
                <button
                    type="button"
                    class="w-full flex items-center justify-between gap-2 px-3 py-2
                     bg-base-100 border border-base-300 rounded-lg
                     hover:border-primary/50 transition-colors
                     text-sm text-base-content"
                    on:click=move |_| set_is_open.update(|open| *open = !*open)
                >
                    <span class="truncate">
                        {move || {
                            let sel = selected.get();
                            if sel.is_empty() {
                                "Select columns...".to_string()
                            } else if sel.len() == 1 {
                                "1 column selected".to_string()
                            } else {
                                format!("{} columns selected", sel.len())
                            }
                        }}
                    </span>
                    <Icon
                        name=IconName::ExpandMore
                        class=if is_open.get_untracked() {
                            "w-4 h-4 transform rotate-180 transition-transform"
                        } else {
                            "w-4 h-4 transition-transform"
                        }
                    />
                </button>

                // Dropdown menu
                {move || {
                    if is_open.get() {
                        Some(
                            view! {
                                <div class="absolute z-50 mt-1 w-full bg-base-100 border border-base-300
                                 rounded-lg shadow-lg max-h-64 overflow-y-auto custom-scrollbar">
                                    // Header with actions
                                    <div class="sticky top-0 bg-base-100 border-b border-base-300 p-2
                                     flex items-center justify-between gap-2">
                                        <button
                                            type="button"
                                            class="btn btn-xs btn-ghost"
                                            on:click=move |_| select_all()
                                        >
                                            "Select All"
                                        </button>
                                        <button
                                            type="button"
                                            class="btn btn-xs btn-ghost"
                                            on:click=move |_| deselect_all()
                                        >
                                            "Clear"
                                        </button>
                                    </div>

                                    // Column checkboxes
                                    <div class="p-1">
                                        {move || {
                                            fields
                                                .get()
                                                .into_iter()
                                                .map(|field| {
                                                    let field_name = field.name.clone();
                                                    let field_name_click = field_name.clone();
                                                    let is_selected = move || {
                                                        selected.get().contains(&field_name)
                                                    };

                                                    view! {
                                                        <label class="flex items-center gap-2 px-3 py-2
                                                         hover:bg-base-200 rounded cursor-pointer
                                                         transition-colors">
                                                            <input
                                                                type="checkbox"
                                                                class="checkbox checkbox-sm checkbox-primary"
                                                                checked=is_selected
                                                                on:change=move |_| {
                                                                    toggle_column(field_name_click.clone())
                                                                }
                                                            />
                                                            <div class="flex items-center gap-2 flex-1">
                                                                <span class=format!(
                                                                    "icon-[lucide--{} w-3.5 h-3.5 {}",
                                                                    match field.field_type {
                                                                        crate::features::dashboard::models::FieldType::Text => {
                                                                            "type"
                                                                        }
                                                                        crate::features::dashboard::models::FieldType::Numeric => {
                                                                            "hash"
                                                                        }
                                                                        crate::features::dashboard::models::FieldType::Date => {
                                                                            "calendar"
                                                                        }
                                                                        crate::features::dashboard::models::FieldType::Boolean => {
                                                                            "check-square"
                                                                        }
                                                                    },
                                                                    field.field_type.icon_color(),
                                                                )></span>
                                                                <span class="text-sm text-base-content">
                                                                    {field.name.clone()}
                                                                </span>
                                                            </div>
                                                        </label>
                                                    }
                                                })
                                                .collect::<Vec<_>>()
                                        }}
                                    </div>

                                    // Footer with selected count
                                    <div class="sticky bottom-0 bg-base-100 border-t border-base-300
                                     px-3 py-2 text-xs text-base-content/60">
                                        {move || {
                                            let count = selected.get().len();
                                            format!(
                                                "{} column{} selected",
                                                count,
                                                if count == 1 { "" } else { "s" },
                                            )
                                        }}
                                    </div>
                                </div>
                            }
                                .into_any(),
                        )
                    } else {
                        None
                    }
                }}
            </div>

            // Selected columns preview (tags)
            {move || {
                let sel = selected.get();
                if !sel.is_empty() {
                    Some(
                        view! {
                            <div class="flex flex-wrap gap-1 mt-1">
                                {sel
                                    .into_iter()
                                    .map(|col_name| {
                                        let col_name_remove = col_name.clone();
                                        view! {
                                            <span class="inline-flex items-center gap-1 px-2 py-0.5
                                             bg-primary/10 text-primary rounded text-xs">
                                                {col_name}
                                                <button
                                                    type="button"
                                                    class="hover:text-error transition-colors"
                                                    on:click=move |_| toggle_column(col_name_remove.clone())
                                                >
                                                    <Icon name=IconName::Close class="w-3 h-3" />
                                                </button>
                                            </span>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        }
                            .into_any(),
                    )
                } else {
                    None
                }
            }}
        </div>
    }
}
