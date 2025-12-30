//! Toggle switch component
//!
//! A toggle switch for boolean values.

use leptos::prelude::*;

/// Toggle switch component
///
/// An iOS-style toggle switch for boolean values.
///
/// # Example
/// ```rust
/// let (enabled, set_enabled) = signal(false);
///
/// view! {
///     <Toggle
///         label="Enable feature"
///         checked=enabled
///         on_change=set_enabled
///     />
/// }
/// ```
#[component]
pub fn Toggle(
    /// Label text
    #[prop(into)]
    label: String,
    /// Current checked state
    #[prop(into)]
    checked: Signal<bool>,
    /// Callback when toggle changes
    on_change: Callback<bool>,
    /// Optional description text
    #[prop(optional, into)]
    description: Option<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <div class="flex items-center justify-between gap-3">
                <div class="flex flex-col">
                    <label class="text-sm font-medium text-base-content">{label}</label>
                    {description
                        .map(|desc| view! { <p class="text-xs text-base-content/60">{desc}</p> })}
                </div>

                <button
                    class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-primary/20"
                    class:bg-primary=move || checked.get()
                    class:bg-base-300=move || !checked.get()
                    on:click=move |_| on_change.run(!checked.get())
                    aria-checked=move || checked.get().to_string()
                    role="switch"
                    type="button"
                >
                    <span
                        class="inline-block h-4 w-4 transform rounded-full bg-white shadow transition-transform"
                        class:translate-x-6=move || checked.get()
                        class:translate-x-1=move || !checked.get()
                    />
                </button>
            </div>
        </div>
    }
}
