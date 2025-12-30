use leptos::prelude::*;

/// Category tab data
#[derive(Clone, PartialEq)]
pub struct CategoryTab {
    pub label: String,
    pub active: bool,
}

/// Category tabs component
///
/// Horizontal tab switcher for template categories.
/// Used in the sidebar to filter templates by category.
///
/// # Example
/// ```rust
/// let tabs = vec![
///     CategoryTab {
///         label: "Business".into(),
///         active: true,
///     },
///     CategoryTab {
///         label: "Sales".into(),
///         active: false,
///     },
/// ];
///
/// view! {
///     <CategoryTabs tabs=tabs />
/// }
/// ```
#[component]
pub fn CategoryTabs(
    /// List of category tabs (reactive)
    #[prop(into)]
    tabs: Signal<Vec<CategoryTab>>,
    /// Optional change handler
    #[prop(optional)]
    on_change: Option<Callback<usize>>,
) -> impl IntoView {
    let change_handler = move |index: usize| {
        if let Some(cb) = &on_change {
            cb.run(index);
        }
    };

    view! {
        <div class="flex gap-1 p-1 bg-base-200 rounded-lg">
            {move || {
                tabs.get()
                    .into_iter()
                    .enumerate()
                    .map(|(index, tab)| {
                        view! {
                            <button
                                class=move || {
                                    if tab.active {
                                        "flex-1 py-1 px-1 text-[11px] font-semibold rounded bg-base-100 \
                                    shadow-sm text-base-content text-center"
                                    } else {
                                        "flex-1 py-1 px-1 text-[11px] font-medium rounded hover:bg-base-100/60 \
                                    text-base-content/60 \
                                    text-center transition-colors"
                                    }
                                }
                                on:click=move |_| change_handler(index)
                            >
                                {tab.label}
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </div>
    }
}
