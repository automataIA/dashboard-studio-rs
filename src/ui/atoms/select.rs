//! Select dropdown component
//!
//! A styled select dropdown for choosing from options.

use leptos::prelude::*;

/// Select option
#[derive(Clone, Debug)]
pub struct SelectOption<T: Clone + 'static> {
    pub label: String,
    pub value: T,
}

/// Select dropdown component
///
/// A styled select dropdown for choosing from a list of options.
///
/// # Example
/// ```rust
/// let options = vec![
///     SelectOption { label: "Small".to_string(), value: "small".to_string() },
///     SelectOption { label: "Large".to_string(), value: "large".to_string() },
/// ];
/// let (selected, set_selected) = signal("small".to_string());
///
/// view! {
///     <Select
///         label="Size"
///         options=options
///         selected=selected
///         on_change=set_selected
///     />
/// }
/// ```
#[component]
pub fn Select<T>(
    /// Label text
    #[prop(into)]
    label: String,
    /// Available options
    #[prop(into)]
    options: Signal<Vec<SelectOption<T>>>,
    /// Currently selected value
    #[prop(into)]
    selected: Signal<T>,
    /// Callback when selection changes
    on_change: Callback<T>,
    /// Optional placeholder text
    #[prop(optional, into)]
    placeholder: Option<String>,
) -> impl IntoView
where
    T: Clone + PartialEq + std::fmt::Display + 'static + Send + Sync,
{
    let _placeholder_text = placeholder.unwrap_or_else(|| "Select...".to_string());

    // Note: Native select elements automatically close when clicking outside
    view! {
        <div class="flex flex-col gap-1">
            <label class="text-sm font-medium text-base-content">{label}</label>

            <select
                class="w-full rounded-lg border border-base-300
                bg-base-100 px-3 py-2 text-sm text-base-content
                focus:border-primary focus:outline-none focus:ring-2
                focus:ring-primary/20 transition-colors"
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    if let Some(opt) = options.get().into_iter().find(|opt| opt.label == value) {
                        on_change.run(opt.value);
                    }
                }
            >
                {move || {
                    let current = selected.get();
                    options
                        .get()
                        .into_iter()
                        .map(|opt| {
                            let is_selected = opt.value == current;
                            let label_for_value = opt.label.clone();
                            let label_for_display = opt.label.clone();
                            view! {
                                <option value=label_for_value selected=is_selected>
                                    {label_for_display}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </select>
        </div>
    }
}
