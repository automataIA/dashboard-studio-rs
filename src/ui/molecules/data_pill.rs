use crate::ui::atoms::{Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Data pill component
///
/// Represents a data field dropped into a drop zone with optional
/// aggregation function and remove button.
///
/// # Example
/// ```rust
/// let on_remove = |ev: MouseEvent| {
///     // Handle remove
/// };
///
/// view! {
///     <DataPill
///         label="Total Revenue".into()
///         icon=Some(IconName::ShowChart)
///         aggregation="SUM".into()
///         show_remove=true
///         on_remove=Some(on_remove)
///     />
/// }
/// ```
#[component]
pub fn DataPill(
    /// Field display name
    #[prop(into)]
    label: String,
    /// Optional icon to show
    icon: Option<IconName>,
    /// Aggregation function (SUM, AVG, etc.)
    #[prop(optional, into)]
    aggregation: Option<String>,
    /// Whether to show the remove button
    #[prop(optional)]
    show_remove: bool,
    /// Optional remove handler
    #[prop(optional)]
    on_remove: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let remove_handler = move |ev: MouseEvent| {
        if let Some(cb) = &on_remove {
            cb.run(ev);
        }
    };

    view! {
        <div class="data-pill flex items-center gap-1.5 py-1 px-2.5 bg-base-100 \
        border border-success/30 rounded-md text-xs \
        font-medium text-base-content group-hover:border-primary/50 \
        transition-colors w-full">

            {match icon {
                Some(icon_name) => {
                    view! { <Icon name=icon_name class="w-[14px] h-[14px] text-success" /> }
                        .into_any()
                }
                None => ().into_any(),
            }}
            <span class="flex-1 truncate">{label}</span>
            {aggregation
                .map(|agg| {
                    view! {
                        <button
                            class="flex items-center gap-0.5 text-[9px] font-bold text-base-content/60 \
                            hover:text-primary uppercase bg-base-100 \
                            px-1.5 py-0.5 rounded hover:bg-base-200 \
                            transition-colors"
                            title="Change Aggregation"
                        >
                            {agg}
                            <Icon name=IconName::ChevronDown class="w-[10px] h-[10px]" />
                        </button>
                    }
                })}
            {if show_remove {
                view! {
                    <button class="text-base-content/40 hover:text-error" on:click=remove_handler>
                        <Icon name=IconName::Close class="w-[14px] h-[14px]" />
                    </button>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
