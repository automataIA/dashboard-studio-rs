use leptos::ev::DragEvent;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Drag & Drop manager for widget reordering
///
/// Handles HTML5 Drag API events for moving widgets around the canvas.
/// Uses dataTransfer to store widget_id during drag operations.
///
/// # Example
/// ```rust
/// view! {
///     <div
///         draggable="true"
///         on:dragstart=DragDropManager::on_drag_start(widget_id.clone())
///         on:dragend=DragDropManager::on_drag_end()
///     >
///         "Draggable widget"
///     </div>
/// }
/// ```
pub struct DragDropManager;

impl DragDropManager {
    /// Creates a dragstart event handler for widgets
    ///
    /// Sets the widget_id in dataTransfer and adds "dragging" CSS class
    pub fn on_drag_start(widget_id: String) -> impl Fn(DragEvent) + 'static {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();

            // Set data transfer with widget ID
            if let Some(data_transfer) = native_ev.data_transfer() {
                let _ = data_transfer.set_data("text/widget-id", &widget_id);
                data_transfer.set_effect_allowed("move");
            }

            // Add dragging class to current target
            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().add_1("dragging");
                }
        }
    }

    /// Creates a dragend event handler for widgets
    ///
    /// Removes "dragging" CSS class when drag ends
    pub fn on_drag_end() -> impl Fn(DragEvent) {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();

            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().remove_1("dragging");
                }
        }
    }

    /// Creates a dragover event handler for drop zones
    ///
    /// Must prevent default to allow dropping
    #[allow(dead_code)]
    pub fn on_drag_over() -> impl Fn(DragEvent) {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();
            native_ev.prevent_default();

            // Set drop effect
            if let Some(data_transfer) = native_ev.data_transfer() {
                data_transfer.set_drop_effect("move");
            }

            // Add drag-over class
            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().add_1("drag-over");
                }
        }
    }

    /// Creates a dragleave event handler for drop zones
    ///
    /// Removes visual feedback when drag leaves zone
    #[allow(dead_code)]
    pub fn on_drag_leave() -> impl Fn(DragEvent) {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();

            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().remove_1("drag-over");
                }
        }
    }

    /// Creates a drop event handler with custom handler function
    ///
    /// Extracts widget_id from dataTransfer and calls handler
    #[allow(dead_code)]
    pub fn on_drop<F>(handler: F) -> impl Fn(DragEvent)
    where
        F: Fn(String) + 'static,
    {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();
            native_ev.prevent_default();

            // Remove drag-over class
            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().remove_1("drag-over");
                }

            // Get widget ID from dataTransfer
            if let Some(data_transfer) = native_ev.data_transfer()
                && let Ok(widget_id) = data_transfer.get_data("text/widget-id")
                && !widget_id.is_empty() {
                    handler(widget_id);
                }
        }
    }
}

/// Drag & Drop manager for field mapping
///
/// Handles dragging fields from dataset sidebar to chart configuration zones.
/// Uses dataTransfer type "application/x-field" to distinguish from widget drops.
///
/// # Example
/// ```rust
/// view! {
///     <div
///         draggable="true"
///         on:dragstart=FieldDragDrop::on_drag_start(field.name.clone())
///     >
///         {field.name}
///     </div>
/// }
/// ```
pub struct FieldDragDrop;

impl FieldDragDrop {
    /// Creates a dragstart event handler for fields
    ///
    /// Sets field_name in dataTransfer with custom MIME type
    pub fn on_drag_start(field_name: String) -> impl Fn(DragEvent) + 'static {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();

            if let Some(data_transfer) = native_ev.data_transfer() {
                let _ = data_transfer.set_data("application/x-field", &field_name);
                let _ = data_transfer.set_data("text/plain", &field_name); // Fallback
                data_transfer.set_effect_allowed("copy");
            }

            // Add dragging class
            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().add_1("dragging");
                }
        }
    }

    /// Creates a dragend event handler for fields
    pub fn on_drag_end() -> impl Fn(DragEvent) {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();

            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().remove_1("dragging");
                }
        }
    }

    /// Creates a drop zone handler for field mapping
    ///
    /// zone_type: "x-axis", "y-axis", "filters", "group-by", etc.
    pub fn on_drop_zone<F>(zone_type: &'static str, handler: F) -> impl Fn(DragEvent)
    where
        F: Fn(&str, String) + 'static,
    {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();
            native_ev.prevent_default();

            // Remove drag-over class
            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().remove_1("drag-over");
                }

            // Get field name from dataTransfer
            if let Some(data_transfer) = native_ev.data_transfer() {
                // Try custom MIME type first
                let field_name = data_transfer
                    .get_data("application/x-field")
                    .ok()
                    .or_else(|| data_transfer.get_data("text/plain").ok())
                    .unwrap_or_default();

                if !field_name.is_empty() {
                    handler(zone_type, field_name);
                }
            }
        }
    }

    /// Creates dragover handler for drop zones
    pub fn on_drag_over() -> impl Fn(DragEvent) {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();
            native_ev.prevent_default();

            if let Some(data_transfer) = native_ev.data_transfer() {
                data_transfer.set_drop_effect("copy");
            }

            // Add drag-over class
            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().add_1("drag-over");
                }
        }
    }

    /// Creates dragleave handler for drop zones
    pub fn on_drag_leave() -> impl Fn(DragEvent) {
        move |ev: DragEvent| {
            let native_ev: &web_sys::DragEvent = ev.unchecked_ref();

            if let Some(target) = native_ev.current_target()
                && let Ok(element) = target.dyn_into::<HtmlElement>() {
                    let _ = element.class_list().remove_1("drag-over");
                }
        }
    }
}
