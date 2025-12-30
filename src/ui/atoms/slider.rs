//! Slider component
//!
//! A range slider for numeric values.

use leptos::prelude::*;

/// Slider component
///
/// A range input for selecting numeric values.
///
/// # Example
/// ```rust
/// let (value, set_value) = signal(50);
///
/// view! {
///     <Slider
///         label="Width"
///         value=value
///         on_change=set_value
///         min=0
///         max=100
///         unit="%"
///     />
/// }
/// ```
#[component]
pub fn Slider(
    /// Label text
    #[prop(into)]
    label: String,
    /// Current value
    #[prop(into)]
    value: Signal<u8>,
    /// Callback when slider changes
    on_change: Callback<u8>,
    /// Minimum value
    #[prop(default = 0)]
    min: u8,
    /// Maximum value
    #[prop(default = 100)]
    max: u8,
    /// Unit suffix to display
    #[prop(optional, into)]
    unit: Option<String>,
) -> impl IntoView {
    let unit_suffix = unit.unwrap_or_default();
    let value_display = move || format!("{}{}", value.get(), unit_suffix);

    view! {
        <div class="flex flex-col gap-2">
            <div class="flex items-center justify-between">
                <label class="text-sm font-medium text-base-content">{label}</label>
                <span class="text-xs font-mono text-base-content/70 bg-base-300 px-2 py-1 rounded">
                    {value_display}
                </span>
            </div>

            <div class="relative flex items-center gap-2">
                <input
                    type="range"
                    min=min
                    max=max
                    prop:value=move || value.get().to_string()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<u8>() {
                            on_change.run(val);
                        }
                    }
                    class="w-full h-2 bg-base-300 rounded-lg appearance-none cursor-pointer
                    accent-primary hover:accent-primary/80
                    focus:outline-none focus:ring-2 focus:ring-primary/20"
                />
            </div>

            <div class="flex justify-between text-xs text-base-content/40">
                <span>{format!("{}", min)}</span>
                <span>{format!("{}", max)}</span>
            </div>
        </div>
    }
}
