use crate::ui::atoms::{Icon, IconName};
use leptos::prelude::*;

/// Input component with optional icon support
///
/// A text input field that can display icons on the left side.
/// Styled to match the design system specifications.
///
/// # Example
/// ```rust
/// let (value, set_value) = signal(String::new());
///
/// view! {
///     // Basic input
///     <Input
///         placeholder="Enter text..."
///         value=value
///     />
///
///     // Input with left icon
///     <Input
///         placeholder="Search..."
///         value=value
///         icon_left=IconName::Search
///     />
///
///     // Input with custom styling
///     <Input
///         placeholder="Email..."
///         value=value
///         class="min-w-64".into()
///     />
/// }
/// ```
#[component]
pub fn Input(
    /// Placeholder text
    #[prop(into)]
    placeholder: String,
    /// Current input value (signal)
    #[prop(optional, into)]
    value: Signal<String>,
    /// Optional icon to display on the left
    #[prop(optional)]
    icon_left: Option<IconName>,
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: Option<String>,
    /// Input type attribute (text, email, password, etc.)
    #[prop(optional, into)]
    input_type: String,
) -> impl IntoView {
    let base_input_classes =
        "form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg \
        text-base-content focus:outline-0 focus:ring-0 border-none \
        bg-base-200 h-full placeholder:text-base-content/40 \
        px-3 text-sm font-normal leading-normal";

    let combined_classes = move || {
        format!(
            "{} {}",
            base_input_classes,
            class.clone().unwrap_or_default()
        )
    };

    view! {
        <div class="flex items-stretch rounded-lg h-full w-full group focus-within:ring-2 focus-within:ring-primary/20">
            {match icon_left {
                Some(icon) => {
                    view! {
                        <div class="text-base-content/60 flex border-none bg-base-200 \
                        items-center justify-center pl-3 rounded-l-lg border-r-0">
                            <Icon name=icon class="w-[18px] h-[18px]" />
                        </div>
                    }
                        .into_any()
                }
                None => view! { <div class="hidden"></div> }.into_any(),
            }}
            <input
                type=input_type
                placeholder=placeholder
                prop:value=value
                on:input=move |_ev| {}
                class=combined_classes
            />
        </div>
    }
}
