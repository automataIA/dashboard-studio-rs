use crate::features::dashboard::{
    config::style::KpiStyleOptions,
    DashboardContext,
    kpi_aggregation::calculate_kpi,
    models::KpiAggregation,
};
use crate::ui::atoms::{Icon, IconName};
use leptos::prelude::*;

/// KPI Widget - displays single metric with trend
///
/// # Example
/// ```rust
/// view! {
///     <KpiWidget widget_id="widget_total_profit" />
/// }
/// ```
#[component]
pub fn KpiWidget(
    /// Widget ID to fetch from DashboardContext
    #[prop(into)]
    widget_id: String,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    // Get widget data from context
    let widget = Memo::new(move |_| {
        dashboard
            .get_widgets()
            .into_iter()
            .find(|w| w.id == widget_id)
    });

    // Parse style options
    let style_options = Memo::new(move |_| {
        widget
            .get()
            .and_then(|w| {
                serde_json::from_str::<KpiStyleOptions>(&w.chart_config.style_options).ok()
            })
            .unwrap_or_default()
    });

    // Calculate KPI value from real data
    let kpi_result = Memo::new(move |_| {
        widget.get().and_then(|w| {
            // Get active dataset
            let active_dataset = dashboard.get_datasets().into_iter().find(|ds| ds.active)?;

            // Get KPI field and aggregation from data mapping
            let kpi_field = w.chart_config.data_mapping.kpi_field.as_ref()?;
            let aggregation = w.chart_config.data_mapping.kpi_aggregation
                .unwrap_or(KpiAggregation::Sum);

            // Calculate KPI value
            calculate_kpi(&active_dataset, kpi_field, aggregation)
        })
    });

    // Format value based on style options
    let value = Memo::new(move |_| {
        if let Some(kpi) = kpi_result.get() {
            let style = style_options.get();
            match style.value_format {
                crate::features::dashboard::config::style::KpiValueFormat::Currency => {
                    format!("${}", kpi.formatted)
                }
                crate::features::dashboard::config::style::KpiValueFormat::Percentage => {
                    format!("{}%", kpi.formatted)
                }
                _ => kpi.formatted,
            }
        } else {
            "No data".to_string()
        }
    });

    // TODO: Calculate comparison and trend from historical data
    let comparison = "Configure KPI field in data panel";
    let trend = "+0.0%";
    let trend_positive = true;
    let progress = 0.0;

    view! {
        {move || {
            let show_trend = style_options.get().show_trend;
            let show_progress = style_options.get().show_progress;
            let show_comparison = style_options.get().show_comparison;
            widget
                .get()
                .map(|_w| {

                    view! {
                        <div class="flex flex-col h-full p-4 sm:p-6">
                            // Main KPI value
                            <div class="flex flex-col gap-1 my-auto">
                                <p class="text-4xl font-extrabold text-slate-900 dark:text-white tracking-tight">
                                    {value}
                                </p>
                                {move || {
                                    if show_comparison {
                                        Some(
                                            view! {
                                                <p class="text-xs text-slate-500 dark:text-secondary mt-1">
                                                    {comparison}
                                                </p>
                                            },
                                        )
                                    } else {
                                        None
                                    }
                                }}

                                // Trend badge
                                {move || {
                                    if show_trend {
                                        Some(
                                            view! {
                                                <span class=format!(
                                                    "text-xs font-bold px-2 py-1 rounded-md flex items-center w-fit mt-3 {}",
                                                    if trend_positive {
                                                        "text-emerald-500 bg-emerald-500/10 border border-emerald-500/20"
                                                    } else {
                                                        "text-rose-500 bg-rose-500/10 border border-rose-500/20"
                                                    },
                                                )>
                                                    <Icon
                                                        name=if trend_positive {
                                                            IconName::TrendingUp
                                                        } else {
                                                            IconName::TrendingDown
                                                        }
                                                        class="w-[14px] h-[14px] mr-1"
                                                    />
                                                    {trend}
                                                </span>
                                            },
                                        )
                                    } else {
                                        None
                                    }
                                }}
                            </div>

                            // Progress bar
                            {move || {
                                if show_progress {
                                    Some(
                                        view! {
                                            <div class="mt-auto">
                                                <div class="flex justify-between text-[10px] text-slate-500 dark:text-secondary mb-1">
                                                    <span>"Progress"</span>
                                                    <span>{format!("{}%", progress as i32)}</span>
                                                </div>
                                                <div class="h-2 w-full bg-slate-100 dark:bg-neutral rounded-full overflow-hidden">
                                                    <div
                                                        class="h-full bg-gradient-to-r from-primary to-purple-500 rounded-full transition-all duration-500"
                                                        style=format!("width: {}%", progress)
                                                    ></div>
                                                </div>
                                            </div>
                                        },
                                    )
                                } else {
                                    None
                                }
                            }}
                        </div>
                    }
                })
        }}
    }
}
