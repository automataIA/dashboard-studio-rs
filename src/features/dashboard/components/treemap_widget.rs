use crate::features::dashboard::{
    config::builders::TreemapConfig, config::style::TreemapStyleOptions,
    config::traits::WidgetConfigBuilder, DashboardContext,
};
use crate::ui::molecules::EChartsWrapper;
use leptos::prelude::*;

/// Treemap Widget with ECharts
///
/// Displays hierarchical data using nested rectangles.
#[component]
pub fn TreemapWidget(
    #[prop(into)]
    widget_id: String,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    let widget = Memo::new(move |_| {
        dashboard
            .get_widgets()
            .into_iter()
            .find(|w| w.id == widget_id)
    });

    let options = Memo::new(move |_| {
        widget.get().and_then(|w| {
            let active_dataset = dashboard.get_datasets().into_iter().find(|ds| ds.active)?;

            let style_options: TreemapStyleOptions =
                serde_json::from_str(&w.chart_config.style_options)
                    .unwrap_or_else(|_| TreemapStyleOptions::default());

            TreemapConfig
                .build_echarts_options(
                    &active_dataset,
                    &w.chart_config.data_mapping,
                    &style_options,
                )
                .ok()
        })
    });

    view! {
        {move || {
            if widget.get().is_some() && options.get().is_some() {
                Some(
                    view! {
                        <EChartsWrapper
                            options=Signal::derive(move || options.get().unwrap_or_default())
                            class="".to_string()
                        />
                    }.into_any(),
                )
            } else {
                Some(
                    view! {
                        <div class="w-full h-full flex items-center justify-center p-4 widget-stripes">
                            <div class="text-center">
                                <p class="text-sm text-base-content/60">
                                    "Configure hierarchy fields to display treemap"
                                </p>
                            </div>
                        </div>
                    }.into_any(),
                )
            }
        }}
    }
}
