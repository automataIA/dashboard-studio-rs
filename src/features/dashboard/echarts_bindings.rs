use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// JavaScript ECharts instance binding
///
/// Bindings for ECharts JavaScript library using wasm-bindgen.
/// This allows Rust/WASM code to interact with ECharts API.
///
/// # Example
/// ```rust
/// use web_sys::HtmlElement;
/// use crate::features::dashboard::echarts_bindings::*;
///
/// let element: HtmlElement = /* get from DOM */;
/// let chart = init(&element);
/// chart.set_option(&options);
/// ```
#[wasm_bindgen]
extern "C" {
    /// ECharts chart instance type
    #[wasm_bindgen(js_namespace = echarts)]
    pub type ECharts;

    /// Initialize ECharts on a DOM element
    ///
    /// # Arguments
    /// * `dom` - HTML element where chart will be rendered
    ///
    /// # Returns
    /// ECharts instance
    #[wasm_bindgen(js_namespace = echarts, js_name = init)]
    pub fn init(dom: &HtmlElement) -> ECharts;

    /// Set chart options
    ///
    /// # Arguments
    /// * `option` - JavaScript object with chart configuration
    ///
    /// Uses setOption to configure or update the chart.
    /// The option object should be a valid ECharts option.
    #[wasm_bindgen(method, js_name = setOption)]
    pub fn set_option(this: &ECharts, option: &JsValue);

    /// Resize chart to fit container
    ///
    /// Call this when the container size changes.
    #[wasm_bindgen(method)]
    pub fn resize(this: &ECharts);

    /// Dispose chart instance
    ///
    /// CRITICAL: Always call this in on_cleanup to prevent memory leaks.
    #[wasm_bindgen(method)]
    pub fn dispose(this: &ECharts);

    /// Show loading animation
    #[wasm_bindgen(method, js_name = showLoading)]
    pub fn show_loading(this: &ECharts);

    /// Hide loading animation
    #[wasm_bindgen(method, js_name = hideLoading)]
    pub fn hide_loading(this: &ECharts);
}
