use crate::features::dashboard::{
    config::builders::ScatterConfig, config::style::ScatterStyleOptions,
    config::traits::WidgetConfigBuilder, DashboardContext,
};
use crate::ui::molecules::EChartsWrapper;
use leptos::prelude::*;

/// Scatter Plot Widget with ECharts
///
/// Displays scatter plots with optional bubble sizing and color dimensions.
/// Uses real data from active dataset with configurable field mapping.
///
/// # Features
/// - Real data from CSV datasets
/// - Configurable field mapping (X, Y, optional Size, optional Color)
/// - Style options (point size, opacity, labels, etc.)
/// - Dark mode support (dynamic colors)
/// - Responsive sizing
///
/// # Example
/// ```rust
/// view! {
///     <ScatterWidget widget_id="widget_scatter_analysis" />
/// }
/// ```
#[component]
pub fn ScatterWidget(
    /// Widget ID to fetch from DashboardContext
    #[prop(into)]
    widget_id: String,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    // Get widget configuration from context
    let widget = Memo::new(move |_| {
        dashboard
            .get_widgets()
            .into_iter()
            .find(|w| w.id == widget_id)
    });

    // Generate ECharts options using ScatterConfig builder
    let options = Memo::new(move |_| {
        widget.get().and_then(|w| {
            // Get active dataset
            let active_dataset = dashboard.get_datasets().into_iter().find(|ds| ds.active)?;

            // Parse style options from JSON
            let style_options: ScatterStyleOptions =
                serde_json::from_str(&w.chart_config.style_options)
                    .unwrap_or_else(|_| ScatterStyleOptions::default());

            // Build ECharts options using ScatterConfig
            ScatterConfig
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
            let has_widget = widget.get().is_some();
            let has_options = options.get().is_some();
            if has_widget && has_options {
                Some(
                    view! {
                        <EChartsWrapper
                            options=Signal::derive(move || options.get().unwrap_or_default())
                            class="".to_string()
                        />
                    }
                        .into_any(),
                )
            } else {
                Some(
                    view! {
                        <div class="w-full h-full flex items-center justify-center p-4 widget-stripes">
                            <div class="text-center">
                                <p class="text-sm text-base-content/60">
                                    "Configure data fields to display scatter plot"
                                </p>
                            </div>
                        </div>
                    }
                        .into_any(),
                )
            }
        }}
    }
}
