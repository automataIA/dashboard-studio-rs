use leptos::prelude::*;

/// Tab item data
#[derive(Clone, PartialEq)]
pub struct TabItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub active: bool,
}

/// Tab bar component
///
/// Horizontal tab switcher for organizing properties panels.
/// Used in the inspector to switch between Data, Style, and AI tabs.
///
/// # Example
/// ```rust
/// let tabs = vec![
///     TabItem {
///         id: "data".into(),
///         label: "Data".into(),
///         icon: Some("icon-[lucide--database]".into()),
///         active: true,
///     },
///     TabItem {
///         id: "style".into(),
///         label: "Style".into(),
///         icon: None,
///         active: false,
///     },
/// ];
///
/// view! {
///     <TabBar tabs=tabs />
/// }
/// ```
#[component]
pub fn TabBar(
    /// List of tabs (reactive getter)
    #[prop(into)]
    tabs: Signal<Vec<TabItem>>,
    /// Optional change handler
    #[prop(optional)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let change_handler = move |id: String| {
        if let Some(cb) = &on_change {
            cb.run(id);
        }
    };

    view! {
        <div class="flex p-1 bg-base-200 rounded-lg">
            {move || {
                tabs.get()
                    .into_iter()
                    .map(|tab| {
                        let tab_id = tab.id.clone();
                        let tab_label = tab.label.clone();
                        let tab_icon = tab.icon.clone();
                        let tab_active = tab.active;

                        view! {
                            <button
                                class=move || {
                                    if tab_active {
                                        "flex-1 py-1.5 px-3 text-[11px] font-semibold rounded bg-base-100 \
                                        shadow-sm text-base-content \
                                        text-center transition-all"
                                    } else {
                                        "flex-1 py-1.5 px-3 text-[11px] font-medium rounded hover:bg-base-100/50 \
                                        text-base-content/60 \
                                        text-center transition-colors"
                                    }
                                }
                                on:click=move |_| change_handler(tab_id.clone())
                            >
                                <span class="flex items-center gap-1">
                                    {if let Some(icon_class) = tab_icon {
                                        view! { <span class=icon_class></span> }.into_any()
                                    } else {
                                        ().into_any()
                                    }}
                                    <span>{tab_label}</span>
                                </span>
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </div>
    }
}
