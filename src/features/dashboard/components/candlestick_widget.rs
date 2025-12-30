use crate::features::dashboard::{
    config::builders::CandlestickConfig, config::style::CandlestickStyleOptions,
    config::traits::WidgetConfigBuilder, DashboardContext,
};
use crate::ui::molecules::EChartsWrapper;
use leptos::prelude::*;

/// Candlestick Widget with ECharts
///
/// Displays financial data in OHLC (Open-High-Low-Close) candlestick format.
/// Commonly used for stock prices, forex, and cryptocurrency data visualization.
///
/// # Features
/// - Real data from CSV datasets
/// - Configurable OHLC field mapping
/// - Customizable rise/fall colors
/// - Optional border colors
/// - Configurable candle width
/// - Dark mode support
/// - Responsive sizing
///
/// # Data Requirements
/// Requires 5 fields:
/// - Date/Time field (X-axis)
/// - Open price (numeric)
/// - Close price (numeric)
/// - Low price (numeric)
/// - High price (numeric)
///
/// # Example
/// ```rust
/// view! {
///     <CandlestickWidget widget_id="widget_candlestick" />
/// }
/// ```
#[component]
pub fn CandlestickWidget(
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

    // Generate ECharts options using CandlestickConfig builder
    let options = Memo::new(move |_| {
        widget.get().and_then(|w| {
            // Get active dataset
            let active_dataset = dashboard.get_datasets().into_iter().find(|ds| ds.active)?;

            // Parse style options from JSON
            let style_options: CandlestickStyleOptions =
                serde_json::from_str(&w.chart_config.style_options)
                    .unwrap_or_else(|_| CandlestickStyleOptions::default());

            // Build ECharts options using CandlestickConfig
            CandlestickConfig
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
                                    "Configure OHLC data fields to display candlestick chart"
                                </p>
                                <p class="text-xs text-base-content/40 mt-2">
                                    "Requires: Date, Open, Close, Low, High"
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
