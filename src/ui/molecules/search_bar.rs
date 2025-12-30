use crate::ui::atoms::{Icon, IconName, Input};
use leptos::prelude::*;

/// Search bar component with icon
///
/// A specialized input field for search functionality with the search icon
/// on the left side. Used in the header of the dashboard.
///
/// # Example
/// ```rust
/// let (search_value, set_search_value) = signal(String::new());
///
/// view! {
///     <SearchBar
///         placeholder="Search data...".into()
///         value=search_value
///     />
/// }
/// ```
#[component]
pub fn SearchBar(
    /// Placeholder text
    #[prop(into)]
    placeholder: String,
    /// Current search value (signal)
    #[prop(optional, into)]
    value: Signal<String>,
    /// Optional width constraint
    #[prop(optional)]
    width: SearchBarWidth,
) -> impl IntoView {
    let container_class = move || {
        format!(
            "hidden lg:flex flex-col min-w-40 !h-9 max-w-64 {}",
            match width {
                SearchBarWidth::Min => "min-w-40",
                SearchBarWidth::Full => "w-full",
                SearchBarWidth::Custom => "",
            }
        )
    };

    let wrapper_class = "flex w-full flex-1 items-stretch rounded-lg h-full group focus-within:ring-2 focus-within:ring-primary/20 transition-shadow";

    view! {
        <label class=container_class>
            <div class=wrapper_class>
                <div class="text-base-content/50 group-focus-within:text-primary \
                flex border-none bg-base-200 items-center justify-center pl-3 rounded-l-lg border-r-0">
                    <Icon name=IconName::Search class="w-[18px] h-[18px]" />
                </div>
                <Input placeholder=placeholder value=value class="pl-2" input_type="text" />
            </div>
        </label>
    }
}

/// Width variants for SearchBar
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SearchBarWidth {
    #[default]
    Min, // min-w-40 max-w-64
    Full,   // w-full
    Custom, // no default width class
}
