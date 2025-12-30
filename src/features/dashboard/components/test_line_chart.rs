use leptos::prelude::*;
use crate::ui::molecules::EChartsWrapper;

/// Test line chart to validate ECharts integration
///
/// This component demonstrates the basic ECharts integration pattern:
/// 1. Create reactive options signal
/// 2. Pass to EChartsWrapper
/// 3. Chart updates automatically when signal changes
///
/// # Example Usage
/// ```rust
/// view! {
///     <TestLineChart />
/// }
/// ```
#[component]
pub fn TestLineChart() -> impl IntoView {
    // Create reactive chart options
    let options = Memo::new(|_| {
        serde_json::json!({
            "title": {
                "text": "ECharts Integration Test",
                "left": "center",
                "textStyle": {
                    "fontSize": 14,
                    "fontWeight": "bold"
                }
            },
            "tooltip": {
                "trigger": "axis"
            },
            "dataset": {
                "source": [
                    ["Month", "Sales"],
                    ["Jan", 150],
                    ["Feb", 230],
                    ["Mar", 224],
                    ["Apr", 218],
                    ["May", 135],
                    ["Jun", 147],
                    ["Jul", 260]
                ]
            },
            "xAxis": {
                "type": "category"
            },
            "yAxis": {
                "type": "value"
            },
            "series": [{
                "type": "line",
                "smooth": true,
                "lineStyle": {
                    "width": 3,
                    "color": "#1C4E80"
                },
                "itemStyle": {
                    "color": "#1C4E80"
                },
                "areaStyle": {
                    "color": {
                        "type": "linear",
                        "x": 0,
                        "y": 0,
                        "x2": 0,
                        "y2": 1,
                        "colorStops": [
                            { "offset": 0, "color": "rgba(28, 78, 128, 0.2)" },
                            { "offset": 1, "color": "rgba(28, 78, 128, 0)" }
                        ]
                    }
                }
            }]
        }).to_string()
    });

    view! {
        <div class="w-full border border-base-300 rounded-lg p-4 bg-base-100">
            <EChartsWrapper
                options=Signal::derive(move || options.get())
                height="300px".to_string()
                class="w-full".to_string()
            />
        </div>
    }
}
