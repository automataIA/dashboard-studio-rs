use crate::features::dashboard::echarts_bindings::*;
use leptos::html::Div;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// Generic ECharts wrapper component
///
/// Manages ECharts lifecycle: init, update, resize, dispose.
/// Uses the dataset pattern for reactive updates.
///
/// # Critical Pattern
/// - **Effect 1**: Initialize chart on mount
/// - **Effect 2**: Update chart when options change (via setOption)
/// - **Effect 3**: Cleanup - dispose chart on unmount (prevents memory leaks!)
///
/// # Example
/// ```rust
/// let options = Memo::new(move |_| {
///     serde_json::json!({
///         "xAxis": { "type": "category" },
///         "yAxis": {},
///         "series": [{ "type": "line", "data": [1, 2, 3] }]
///     }).to_string()
/// });
///
/// view! {
///     <EChartsWrapper
///         options=options.into()
///         class="w-full h-full"
///     />
/// }
/// ```
#[component]
pub fn EChartsWrapper(
    /// ECharts options as JSON string (reactive signal)
    #[prop(into)]
    options: Signal<String>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,

    /// Chart height (CSS value, e.g., "400px" or "100%")
    #[prop(optional, into)]
    height: Option<String>,
) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let chart_instance = SendWrapper::new(Rc::new(RefCell::new(None::<ECharts>)));

    // Effect 1: Initialize chart on mount
    {
        let chart_instance = chart_instance.clone();
        Effect::new(move |_| {
            if let Some(container) = container_ref.get() {
                // Convert leptos Div to web_sys HtmlElement
                let element = container.unchecked_into::<web_sys::HtmlElement>();

                // Initialize ECharts
                let chart = init(&element);
                *chart_instance.borrow_mut() = Some(chart);

                log::info!("ECharts instance initialized");

                // Effect: ResizeObserver to handle container resize
                let element_copy = element.clone();
                let chart_instance_resize = chart_instance.clone();

                let resize_callback = Closure::wrap(Box::new(
                    move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                        if let Some(chart) = chart_instance_resize.borrow().as_ref() {
                            chart.resize();
                            log::debug!("ECharts resized due to container change");
                        }
                    },
                )
                    as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);

                let observer =
                    web_sys::ResizeObserver::new(resize_callback.as_ref().unchecked_ref())
                        .expect("Failed to create ResizeObserver");

                observer.observe(&element_copy);

                // Keep closure alive
                resize_callback.forget();
            }
        });
    }

    // Effect 2: Update chart when options change
    {
        let chart_instance = chart_instance.clone();
        Effect::new(move |_| {
            let options_json = options.get();

            if let Some(chart) = chart_instance.borrow().as_ref() {
                // Parse JSON string to JsValue
                match js_sys::JSON::parse(&options_json) {
                    Ok(js_options) => {
                        chart.set_option(&js_options);
                        log::debug!("ECharts options updated");
                    }
                    Err(e) => {
                        log::error!("Failed to parse ECharts options: {:?}", e);
                    }
                }
            }
        });
    }

    // Effect 3: Cleanup on component unmount
    {
        let chart_instance = chart_instance.clone();
        on_cleanup(move || {
            if let Some(chart) = chart_instance.borrow_mut().take() {
                chart.dispose();
                log::info!("ECharts instance disposed");
            }
        });
    }

    let container_class = format!(
        "echarts-container chart-container {}",
        class.unwrap_or_default()
    );

    let container_style = height
        .map(|h| format!("height: {}", h))
        .unwrap_or_else(|| "height: 100%".to_string());

    view! {
        <div
            node_ref=container_ref
            class=container_class
            style=format!("width: 100%; {}", container_style)
        ></div>
    }
}
