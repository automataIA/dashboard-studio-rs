use crate::features::dashboard::{calculate_kpi, analyze_dataset_for_kpis, DashboardContext, KpiAggregation};
use crate::ui::atoms::{Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// KPI Quick Actions modal
///
/// Shows suggested KPI configurations based on the active dataset.
/// Appears when user creates a KPI widget from template.
///
/// # Example
/// ```rust
/// let (show_quick_actions, set_show_quick_actions) = signal(false);
/// let widget_id = "widget_123".into();
///
/// view! {
///     <KpiQuickActions
///         show=show_quick_actions
///         on_close=move |_| set_show_quick_actions.set(false)
///         on_select=on_kpi_select
///     />
/// }
/// ```
#[component]
pub fn KpiQuickActions(
    /// Whether to show the modal
    #[prop(into)]
    show: Signal<bool>,
    /// Callback when modal is closed
    on_close: Callback<MouseEvent>,
    /// Callback when a KPI option is selected: (field_name, aggregation, formatted_label)
    on_select: Callback<(String, KpiAggregation, String)>,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    // Analyze active dataset for KPI suggestions
    let kpi_suggestions = Memo::new(move |_| {
        if let Some(dataset) = dashboard.get_active_dataset() {
            return analyze_dataset_for_kpis(&dataset);
        }

        vec![] // No active dataset
    });

    // Select a KPI configuration
    let handle_select = {
        let on_select = on_select.clone();
        move |field_name: String, aggregation: KpiAggregation, label: String| {
            log::info!("KPI selected: {} - {:?}", label, aggregation);

            // Calculate actual value for preview
            if let Some(dataset) = dashboard.get_active_dataset() {
                if let Some(kpi_value) = calculate_kpi(&dataset, &field_name, aggregation) {
                    log::info!("KPI value: {}", kpi_value.formatted);
                }
            }

            on_select.run((field_name, aggregation, label));
        }
    };

    view! {
        <div
            class="modal modal-open"
            class:opacity-0=!show.get()
            class:pointer-events-none=!show.get()
            style="transition-opacity 0.2s"
        >
            <div class="modal-box max-w-2xl">
                // Header
                <div class="flex items-center justify-between mb-6">
                    <div>
                        <h3 class="font-bold text-lg flex items-center gap-2">
                            <Icon name=IconName::ShowChart class="w-5 h-5 text-primary" />
                            "Configure KPI"
                        </h3>
                        <p class="text-sm text-base-content/60 mt-1">
                            "Choose what to display in this KPI card"
                        </p>
                    </div>
                    <button
                        class="btn btn-sm btn-circle btn-ghost"
                        on:click=move |ev| on_close.run(ev)
                    >
                        <Icon name=IconName::Close class="w-4 h-4" />
                    </button>
                </div>

                // No dataset message or KPI suggestions
                {move || {
                    let suggestions = kpi_suggestions.get();
                    let is_empty = suggestions.is_empty();

                    view! {
                        <div class:hidden=is_empty>
                            <div class="text-center py-8">
                                <Icon name=IconName::Upload class="w-12 h-12 text-base-content/20 mx-auto mb-3" />
                                <p class="text-base-content/60 mb-1">"No dataset selected"</p>
                                <p class="text-xs text-base-content/40">
                                    "Upload a CSV file first to enable KPI suggestions"
                                </p>
                            </div>
                        </div>

                        <div class:hidden=(!is_empty)>
                            // KPI suggestions grid
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                                {suggestions
                                    .into_iter()
                                    .flat_map(|(field_name, suggestions)| {
                                        suggestions.into_iter().map(move |(aggregation, label)| {
                                            let field_clone = field_name.clone();
                                            let label_clone = label.clone();
                                            let aggregation_clone = aggregation;

                                            // Determine icon
                                            let icon = match aggregation {
                                                KpiAggregation::Sum => IconName::Add,
                                                KpiAggregation::Average => IconName::TrendingUp,
                                                KpiAggregation::Count => IconName::Hash,
                                                KpiAggregation::Min => IconName::ArrowDown,
                                                KpiAggregation::Max => IconName::ArrowUp,
                                                KpiAggregation::Last => IconName::Schedule,
                                                KpiAggregation::First => IconName::History,
                                            };

                                            view! {
                                                <button
                                                    class="btn btn h-auto py-3 px-4 justify-start gap-3"
                                                    on:click=move |_| {
                                                        handle_select(
                                                            field_clone.clone(),
                                                            aggregation_clone,
                                                            label_clone.clone(),
                                                        );
                                                    }
                                                >
                                                    <div class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center shrink-0">
                                                        <Icon name=icon class="w-5 h-5 text-primary" />
                                                    </div>
                                                    <div class="text-left flex-1">
                                                        <div class="font-semibold text-sm">
                                                            {label_clone.clone()}
                                                        </div>
                                                        <div class="text-xs text-base-content/60 mt-0.5">
                                                            {aggregation_clone.display_name()}
                                                            " of "
                                                            {field_clone.clone()}
                                                        </div>
                                                    </div>
                                                    <Icon name=IconName::ChevronRight class="w-4 h-4 text-base-content/40" />
                                                </button>
                                            }
                                        })
                                    })
                                    .collect::<Vec<_>>()
                                }
                            </div>

                            // Custom option (placeholder for future)
                            <div class="mt-4 pt-4 border-t border-base-300">
                                <button
                                    class="btn btn-ghost btn-sm w-full justify-start gap-2 text-base-content/60"
                                >
                                    <Icon name=IconName::Settings class="w-4 h-4" />
                                    "Custom configuration..."
                                </button>
                            </div>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
