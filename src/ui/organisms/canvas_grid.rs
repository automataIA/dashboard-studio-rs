#![allow(dead_code)]

use crate::features::dashboard::components::*;
use crate::features::dashboard::{DashboardContext, GridPosition, WidgetId, WidgetType};
use crate::ui::atoms::{Icon, IconName};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, Default, PartialEq)]
struct InteractionState {
    widget_id: Option<WidgetId>,
    start_pointer: (f64, f64),
    start_grid: GridPosition,
}

/// Placeholder component for unimplemented widgets
///
/// Shows a "Coming Soon" message with the widget type icon.
#[component]
fn WidgetPlaceholder(widget_type: WidgetType, _widget_id: WidgetId) -> impl IntoView {
    let icon = match widget_type {
        WidgetType::Pie => IconName::PieChart,
        WidgetType::Scatter => IconName::ScatterPlot,
        WidgetType::Area => IconName::AreaChart,
        WidgetType::Radar => IconName::Radar,
        WidgetType::Candlestick => IconName::CandlestickChart,
        WidgetType::Heatmap => IconName::Heatmap,
        WidgetType::Treemap => IconName::Treemap,
        _ => IconName::ShowChart,
    };

    view! {
        <div class="border-2 border-dashed border-base-300
        rounded-xl flex flex-col items-center justify-center
        p-8 min-h-[200px] widget-stripes">
            <div class="size-12 rounded-full bg-base-100 shadow-sm
            flex items-center justify-center
            text-base-content/40
            mb-4">
                <Icon name=icon class="w-6 h-6" />
            </div>
            <p class="text-base-content font-bold text-sm">{widget_type.display_name()}</p>
            <p class="text-base-content/60 text-xs mt-1">"Coming in Phase 4"</p>
        </div>
    }
}

/// Canvas grid organism component
///
/// Main widget display area using CSS Grid layout.
/// Gets widgets from DashboardContext and renders smart widget components.
/// Supports widget selection via click.
///
/// # Example
/// ```rust
/// view! {
///     <CanvasGrid />
/// }
/// ```
#[component]
pub fn CanvasGrid() -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    let widgets = Memo::new(move |_| dashboard.get_widgets());

    // Get selected widget ID for styling
    let selected_widget_id = Memo::new(move |_| dashboard.selected_widget_id.get());

    let grid_ref = NodeRef::<leptos::html::Div>::new();

    // Track Shift key state
    let is_shift_pressed = RwSignal::new(false);

    // Global event listeners for Shift key
    {
        let window = web_sys::window().unwrap();

        let on_keydown = move |e: web_sys::KeyboardEvent| {
            if e.shift_key() {
                is_shift_pressed.set(true);
            }
        };

        let on_keyup = move |e: web_sys::KeyboardEvent| {
            if !e.shift_key() {
                is_shift_pressed.set(false);
            }
        };

        let _ = window.add_event_listener_with_callback(
            "keydown",
            Closure::wrap(Box::new(on_keydown) as Box<dyn FnMut(web_sys::KeyboardEvent)>)
                .into_js_value()
                .unchecked_ref(),
        );

        let _ = window.add_event_listener_with_callback(
            "keyup",
            Closure::wrap(Box::new(on_keyup) as Box<dyn FnMut(web_sys::KeyboardEvent)>)
                .into_js_value()
                .unchecked_ref(),
        );

        // Also reset if window loses focus
        let on_blur = move |_: web_sys::Event| {
            is_shift_pressed.set(false);
        };

        let _ = window.add_event_listener_with_callback(
            "blur",
            Closure::wrap(Box::new(on_blur) as Box<dyn FnMut(web_sys::Event)>)
                .into_js_value()
                .unchecked_ref(),
        );
    }

    // Interaction State
    let dragging = RwSignal::new(InteractionState::default());
    let resizing = RwSignal::new(InteractionState::default());

    let container_ref = grid_ref;

    // Helper to trigger ECharts resize
    let trigger_echarts_resize = move |id: &str| {
        let js_code = format!(
            r#"
            (function() {{
                const container = document.querySelector('[data-widget-id="{}"] .echarts-container');
                if (container) {{
                    const instance = echarts.getInstanceByDom(container);
                    if (instance) instance.resize();
                }}
            }})()
            "#,
            id
        );
        let _ = js_sys::eval(&js_code);
    };

    let on_pointer_move = move |e: web_sys::PointerEvent| {
        let gap = 8.0; // gap-2 = 0.5rem = 8px
        let row_height = 100.0;

        if let Some(id) = dragging.get().widget_id {
            let state = dragging.get();
            if let Some(container) = container_ref.get() {
                let rect = container
                    .unchecked_into::<web_sys::Element>()
                    .get_bounding_client_rect();
                let col_step = (rect.width() + gap) / 12.0;
                let row_step = row_height + gap;

                let dx = e.client_x() as f64 - state.start_pointer.0;
                let dy = e.client_y() as f64 - state.start_pointer.1;

                let new_x = (state.start_grid.x as f64 + dx / col_step)
                    .round()
                    .max(0.0)
                    .min(12.0 - state.start_grid.width as f64) as u32;
                let new_y = (state.start_grid.y as f64 + dy / row_step).round().max(0.0) as u32;

                if new_x != state.start_grid.x || new_y != state.start_grid.y {
                    let mut pos = state.start_grid;
                    pos.x = new_x;
                    pos.y = new_y;
                    dashboard.update_widget_position(&id, pos);
                }
            }
        } else if let Some(id) = resizing.get().widget_id {
            let state = resizing.get();
            if let Some(container) = container_ref.get() {
                let rect = container
                    .unchecked_into::<web_sys::Element>()
                    .get_bounding_client_rect();
                let col_step = (rect.width() + gap) / 12.0;
                let row_step = row_height + gap;

                let dx = e.client_x() as f64 - state.start_pointer.0;
                let dy = e.client_y() as f64 - state.start_pointer.1;

                let new_w = (state.start_grid.width as f64 + dx / col_step)
                    .round()
                    .max(1.0)
                    .min(12.0 - state.start_grid.x as f64) as u32;
                let new_h = (state.start_grid.height as f64 + dy / row_step)
                    .round()
                    .max(1.0) as u32;

                if new_w != state.start_grid.width || new_h != state.start_grid.height {
                    let mut pos = state.start_grid;
                    pos.width = new_w;
                    pos.height = new_h;
                    dashboard.update_widget_position(&id, pos);
                    trigger_echarts_resize(&id);
                }
            }
        }
    };

    let on_pointer_up = move |_: web_sys::PointerEvent| {
        if let Some(id) = dragging.get().widget_id {
            trigger_echarts_resize(&id);
        }
        if let Some(id) = resizing.get().widget_id {
            trigger_echarts_resize(&id);
        }
        dragging.set(InteractionState::default());
        resizing.set(InteractionState::default());
    };

    // Handler for clicking on canvas background to deselect widgets
    let on_canvas_click = move |ev: leptos::ev::MouseEvent| {
        // Check if the click target is the canvas itself (not a widget or child element)
        if let Some(target) = ev.target()
            && let Some(element) = target.dyn_ref::<web_sys::Element>()
            && element.class_list().contains("grid")
        {
            log::info!("Canvas background clicked - deselecting widget");
            dashboard.set_selected_widget(None);
        }
    };

    view! {
        <div
            node_ref=container_ref
            class="grid grid-cols-12 gap-2 relative min-h-[600px] bg-transparent rounded-xl p-2"
            class:shift-active=move || is_shift_pressed.get()
            style="grid-auto-rows: 100px; grid-auto-flow: dense;"
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointerleave=on_pointer_up
            on:click=on_canvas_click
        >
            {move || {
                widgets
                    .get()
                    .into_iter()
                    .map(|widget| {
                        let widget_id = widget.id.clone();
                        let widget_id_label = widget_id.clone();
                        let widget_type = widget.widget_type;
                        let pos = widget.grid_position;
                        let is_selected = selected_widget_id.get() == Some(widget_id.clone());

                        // Create clones for each event handler to avoid move issues
                        let widget_id_click = widget_id.clone();
                        let widget_id_drag_1 = widget_id.clone();
                        let widget_id_drag_2 = widget_id.clone();
                        let widget_id_drag_3 = widget_id.clone();
                        let widget_id_drag_4 = widget_id.clone();
                        let widget_id_resize = widget_id.clone();
                        let selection_class = if is_selected {
                            "ring-4 ring-primary ring-opacity-50 z-10"
                        } else {
                            "z-0"
                        };
                        let grid_style = format!(
                            "grid-column: {} / span {}; grid-row: {} / span {};",
                            pos.x + 1,
                            pos.width,
                            pos.y + 1,
                            pos.height,
                        );
                        let content = match widget_type {
                            WidgetType::Line => {
                                view! { <LineChartWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Bar => {
                                view! { <BarChartWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Kpi => {
                                view! { <KpiWidget widget_id=widget_id_label.clone() /> }.into_any()
                            }
                            WidgetType::Table => {
                                view! { <TableWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Pie => {
                                view! { <PieChartWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Candlestick => {
                                view! { <CandlestickWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Area => {
                                view! { <AreaChartWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Scatter => {
                                view! { <ScatterWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Radar => {
                                view! { <RadarWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Heatmap => {
                                view! { <HeatmapWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                            WidgetType::Treemap => {
                                view! { <TreemapWidget widget_id=widget_id_label.clone() /> }
                                    .into_any()
                            }
                        };

                        view! {
                            <div
                                class=format!(
                                    "relative group bg-base-100 rounded-xl shadow-sm transition-shadow hover:shadow-md {}",
                                    selection_class,
                                )
                                style=grid_style
                                on:click=move |ev| {
                                    ev.stop_propagation(); // Prevent canvas deselection
                                    log::info!("Widget clicked: {}", widget_id_click.clone());
                                    dashboard.set_selected_widget(Some(widget_id_click.clone()));
                                }
                                data-widget-id=widget_id_label.clone()
                            >
                                <div class="w-full h-full flex flex-col overflow-hidden p-0 relative">
                                    // Border drag handles
                                    <div
                                        class="widget-border-handle absolute top-0 left-0 right-0 h-2 z-20"
                                        on:pointerdown=move |e| {
                                            if is_shift_pressed.get() {
                                                e.stop_propagation();
                                                dragging.set(InteractionState {
                                                    widget_id: Some(widget_id_drag_1.clone()),
                                                    start_pointer: (e.client_x() as f64, e.client_y() as f64),
                                                    start_grid: pos,
                                                });
                                            }
                                        }
                                    ></div>
                                    <div
                                        class="widget-border-handle absolute bottom-0 left-0 right-0 h-2 z-20"
                                        on:pointerdown=move |e| {
                                            if is_shift_pressed.get() {
                                                e.stop_propagation();
                                                dragging.set(InteractionState {
                                                    widget_id: Some(widget_id_drag_2.clone()),
                                                    start_pointer: (e.client_x() as f64, e.client_y() as f64),
                                                    start_grid: pos,
                                                });
                                            }
                                        }
                                    ></div>
                                    <div
                                        class="widget-border-handle absolute top-0 bottom-0 left-0 w-2 z-20"
                                        on:pointerdown=move |e| {
                                            if is_shift_pressed.get() {
                                                e.stop_propagation();
                                                dragging.set(InteractionState {
                                                    widget_id: Some(widget_id_drag_3.clone()),
                                                    start_pointer: (e.client_x() as f64, e.client_y() as f64),
                                                    start_grid: pos,
                                                });
                                            }
                                        }
                                    ></div>
                                    <div
                                        class="widget-border-handle absolute top-0 bottom-0 right-0 w-2 z-20"
                                        on:pointerdown=move |e| {
                                            if is_shift_pressed.get() {
                                                e.stop_propagation();
                                                dragging.set(InteractionState {
                                                    widget_id: Some(widget_id_drag_4.clone()),
                                                    start_pointer: (e.client_x() as f64, e.client_y() as f64),
                                                    start_grid: pos,
                                                });
                                            }
                                        }
                                    ></div>

                                    // Resize handle (bottom-right)
                                    <div
                                        class="ui-resizable-se absolute bottom-1 right-1 w-4 h-4 rounded-br-lg z-30 cursor-nwse-resize"
                                        on:pointerdown=move |e| {
                                            if is_shift_pressed.get() {
                                                e.stop_propagation();
                                                resizing.set(InteractionState {
                                                    widget_id: Some(widget_id_resize.clone()),
                                                    start_pointer: (e.client_x() as f64, e.client_y() as f64),
                                                    start_grid: pos,
                                                });
                                            }
                                        }
                                    ></div>

                                    {content}
                                </div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </div>
    }
}
