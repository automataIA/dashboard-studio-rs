use crate::features::dashboard::context::DashboardContext;
use crate::features::dashboard::models::WidgetType;
use crate::ui::atoms::{
    Badge, BadgeSize, BadgeVariant, Button, ButtonSize, ButtonVariant, Icon, IconName,
};
use crate::ui::molecules::{LayerItem, TabBar, TabItem, WidgetSelector};
use crate::ui::organisms::data::LayerData;
use crate::ui::organisms::WidgetConfigurationPanel;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Right sidebar organism component
///
/// Right panel containing AI Chart Creator, Active Layers, Widget Selector,
/// and contextual Properties inspector (Data/Style/AI tabs).
///
/// NEW STRUCTURE:
/// 1. AI Chart Creator (top)
/// 2. Active Layers Panel
/// 3. Widget Selector (EXTERNAL to tabs - NEW!)
/// 4. Properties Panel with Tabs (contextual to widget type)
///
/// # Example
/// ```rust
/// let (ai_prompt, set_ai_prompt) = signal(String::new());
/// let layers = vec![...];
/// let (active_tab, set_active_tab) = signal("data".to_string());
/// view! {
///     <RightSidebar
///         _ai_prompt=ai_prompt
///         layers=layers
///         _active_tab=active_tab
///     />
/// }
/// ```
#[component]
pub fn RightSidebar(
    /// AI prompt textarea content
    _ai_prompt: Signal<String>,
    /// List of active layers (reactive)
    #[prop(into)]
    layers: Signal<Vec<LayerData>>,
    /// Currently active property tab
    #[prop(into)]
    _active_tab: Signal<String>,
) -> impl IntoView {
    // Get Dashboard context
    let dashboard = DashboardContext::use_context();

    // Widget type selection state
    let (selected_widget, set_selected_widget) = signal(WidgetType::Line);

    // Track if user is manually changing widget type (for new widget creation)
    let (user_changing_type, set_user_changing_type) = signal(false);

    // Active tab state
    let (active_tab, set_active_tab) = signal("data".to_string());

    // Synchronize widget type selector with selected widget in canvas
    // BUT only if user is not manually changing the type for new widget creation
    Effect::new(move |_| {
        // Track selected_widget_id explicitly to ensure Effect triggers on changes
        let selected_id = dashboard.selected_widget_id.get();

        // Don't override if user is actively selecting a widget type
        if user_changing_type.get() {
            return;
        }

        if let Some(widget_id) = selected_id {
            // Get widget by ID to ensure we have the current widget
            if let Some(widget) = dashboard.get_widgets().into_iter().find(|w| w.id == widget_id) {
                log::info!("Syncing widget type selector to: {:?}", widget.widget_type);
                set_selected_widget.set(widget.widget_type);
            }
        }
    });

    // Widget type change handler - save to DashboardContext
    let on_widget_change = Callback::new(move |widget_type: WidgetType| {
        set_selected_widget.set(widget_type);
        dashboard.set_pending_widget_type.set(widget_type);
        // Mark that user is actively changing type (prevents Effect from overriding)
        set_user_changing_type.set(true);

        // Reset the flag after a short delay using wasm_bindgen setTimeout
        let set_user_changing_type = set_user_changing_type;
        let timeout_closure = wasm_bindgen::closure::Closure::once(move || {
            set_user_changing_type.set(false);
        });
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_closure.as_ref().unchecked_ref(),
                100,
            )
            .unwrap();
        timeout_closure.forget();

        log::info!("Widget type changed to: {:?}", widget_type);
    });

    // Handlers
    let on_generate = Callback::new(|_: MouseEvent| { /* TODO: Generate chart with AI */ });
    let on_mic_click = Callback::new(|_: MouseEvent| { /* TODO: Voice input */ });

    // Layer/Widget selection handler
    let on_layer_click = Callback::new(move |_: MouseEvent| {
        // TODO: Select layer/widget
    });

    // Widget deletion handler
    let on_widget_delete = Callback::new(move |widget_id: String| {
        log::info!("Deleting widget: {}", widget_id);
        dashboard.remove_widget(&widget_id);
    });

    // Add Widget handler
    let on_add_widget = Callback::new(move |_: MouseEvent| {
        // Get the selected widget type from the WidgetSelector
        let widget_type = dashboard.pending_widget_type.get();

        // Create a new widget with the selected type and add it to the dashboard
        let widget_id = format!("widget_{}", uuid::Uuid::new_v4());

        // Get default style options based on widget type
        use crate::features::dashboard::models::{ChartConfig, DataMapping, GridPosition, Widget};

        let default_style = match widget_type {
            crate::features::dashboard::models::WidgetType::Line => "{\"animation\":true,\"animation_duration\":1000,\"area_fill\":false,\"line_width\":3,\"show_points\":false,\"point_size\":4,\"show_labels\":false}",
            crate::features::dashboard::models::WidgetType::Bar => "{\"animation\":true,\"animation_duration\":1000,\"bar_width\":0.8,\"show_labels\":false,\"label_position\":\"top\"}",
            crate::features::dashboard::models::WidgetType::Pie => "{\"animation\":true,\"animation_duration\":1000,\"show_labels\":true,\"label_position\":\"outside\",\"inner_radius\":0}",
            crate::features::dashboard::models::WidgetType::Kpi => "{\"value_format\":\"number\",\"show_trend\":true,\"show_progress\":true,\"decimals\":0,\"show_comparison\":true}",
            crate::features::dashboard::models::WidgetType::Table => "{\"show_pagination\":true,\"page_size\":10,\"show_sorting\":true,\"compact_mode\":false}",
            crate::features::dashboard::models::WidgetType::Scatter => "{\"animation\":true,\"show_points\":true,\"point_size\":8,\"show_labels\":false}",
            crate::features::dashboard::models::WidgetType::Area => "{\"animation\":true,\"area_fill\":true,\"area_opacity\":0.3,\"line_width\":2}",
            crate::features::dashboard::models::WidgetType::Radar => "{\"animation\":true,\"show_area\":false,\"line_width\":2,\"show_points\":true}",
            crate::features::dashboard::models::WidgetType::Candlestick => "{\"animation\":true,\"show_values\":true,\"bar_width\":0.6}",
            crate::features::dashboard::models::WidgetType::Heatmap => "{\"animation\":true,\"show_values\":true,\"cell_size\":20}",
            crate::features::dashboard::models::WidgetType::Treemap => "{\"animation\":true,\"show_labels\":true,\"leaf_depth\":1}",
        };

        // Count existing widgets of this type to generate sequential number
        let existing_count = dashboard
            .get_widgets()
            .iter()
            .filter(|w| w.widget_type == widget_type)
            .count();
        let widget_number = existing_count + 1;

        // Generate title in format "Line Chart #1", "Bar Chart #2", etc.
        let title = format!("{} #{}", widget_type.display_name(), widget_number);

        let new_widget = Widget {
            id: widget_id.clone(),
            title,
            subtitle: None,
            widget_type,
            chart_config: ChartConfig {
                chart_type: Some(widget_type),
                data_mapping: DataMapping::default(),
                style_options: default_style.to_string(),
            },
            grid_position: GridPosition::default(),
            editing: false,
        };

        dashboard.add_widget(new_widget);

        // Automatically select the newly created widget
        dashboard.set_selected_widget(Some(widget_id.clone()));

        log::info!(
            "Added new widget with type {:?} and ID: {}",
            widget_type,
            widget_id
        );
    });

    // Tab change handler
    let on_tab_change = Callback::new(move |tab_id: String| {
        set_active_tab.set(tab_id);
    });

    // Calculate layers length reactively
    let layers_count = move || layers.get().len();

    // Property tabs (as reactive Signal)
    let tabs = Signal::derive(move || {
        vec![
            TabItem {
                id: "data".into(),
                label: "Data".into(),
                icon: None,
                active: active_tab.get() == "data",
            },
            TabItem {
                id: "style".into(),
                label: "Style".into(),
                icon: None,
                active: active_tab.get() == "style",
            },
            TabItem {
                id: "ai".into(),
                label: "AI".into(),
                icon: Some("icon-[lucide--sparkles]".into()),
                active: active_tab.get() == "ai",
            },
        ]
    });

    view! {
        <div class="flex flex-col h-full">
            // ========== AI Chart Creator Section ==========
            <div class="p-4 border-b border-base-300 bg-base-200/80 backdrop-blur-sm shrink-0 flex flex-col gap-3">
                // Header
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <div class="flex items-center justify-center size-6 rounded
                        bg-primary/10 text-primary">
                            <Icon name=IconName::AutoAwesome class="w-4 h-4" />
                        </div>
                        <h2 class="text-xs font-bold text-base-content/80 uppercase tracking-wider">
                            "AI Chart Creator"
                        </h2>
                    </div>
                    <div class="flex items-center gap-1">
                        <Badge variant=BadgeVariant::Info size=BadgeSize::Small class="uppercase">
                            "Local LLM"
                        </Badge>
                    </div>
                </div>

                // Textarea with mic button
                <div class="relative group">
                    <textarea
                        class="w-full bg-base-100 border border-base-300 rounded-lg
                        text-xs text-base-content p-3 pr-8
                        focus:ring-2 focus:ring-primary/20 focus:border-primary
                        transition-all resize-none h-20
                        placeholder:text-base-content/40 leading-relaxed
                        shadow-sm custom-scrollbar"
                        placeholder="Describe your chart...
                        e.g. 'Compare sales vs cost by region'"
                    ></textarea>
                    <div
                        class="absolute bottom-2 right-2 cursor-pointer text-base-content/40
                        hover:text-primary transition-colors"
                        title="Use Voice"
                        on:click=move |ev| on_mic_click.run(ev)
                    >
                        <Icon name=IconName::Mic class="w-4 h-4" />
                    </div>
                </div>

                // Generate button
                <Button
                    variant=ButtonVariant::Primary
                    size=ButtonSize::Medium
                    on_click=on_generate
                    class="w-full items-center justify-center gap-2"
                >
                    <Icon name=IconName::SmartToy class="w-4 h-4" />
                    "Generate Chart"
                </Button>

                // Suggestion box
                <div class="bg-info/10 border border-info/20
                rounded-lg p-2.5 flex gap-2 items-start">
                    <Icon name=IconName::Lightbulb class="w-4 h-4 text-info shrink-0 mt-0.5" />
                    <div class="flex flex-col gap-0.5">
                        <p class="text-[10px] font-bold text-info">"Suggestion"</p>
                        <p class="text-[10px] text-info/80 leading-snug">
                            "Try: \"Show me the distribution of customer age groups as a pie chart.\""
                        </p>
                    </div>
                </div>
            </div>

            // ========== Widgets List Panel ==========
            <div class="flex flex-col border-b border-base-300 max-h-56 shrink-0">
                <div class="p-4 pb-2 flex items-center justify-between">
                    <h2 class="text-xs font-bold text-base-content/60 uppercase tracking-wider">
                        "Widgets List"
                    </h2>
                    <Badge variant=BadgeVariant::Neutral size=BadgeSize::Small>
                        {layers_count}
                    </Badge>
                </div>

                <div class="flex flex-col gap-1 px-3 pb-3 overflow-y-auto custom-scrollbar">
                    {move || {
                        layers
                            .get()
                            .into_iter()
                            .map(|layer| {
                                let widget_id = layer.id.clone();
                                let widget_id_delete = widget_id.clone();
                                let widget_id_rename = widget_id.clone();

                                let on_delete_cb = Callback::new(move |_: MouseEvent| {
                                    on_widget_delete.run(widget_id_delete.clone());
                                });

                                let on_rename_cb = Callback::new(move |new_name: String| {
                                    log::info!("Renaming widget {} to: {}", widget_id_rename.clone(), new_name);
                                    dashboard.update_widget(&widget_id_rename, |widget| {
                                        widget.title = new_name;
                                    });
                                });

                                view! {
                                    <LayerItem
                                        _widget_id=widget_id
                                        label=layer.label
                                        active=layer.active
                                        on_click=on_layer_click
                                        on_delete=on_delete_cb
                                        on_rename=on_rename_cb
                                    />
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>

            // ========== Widget Selector (EXTERNAL to tabs) ==========
            <div class="p-4 border-b border-base-300 shrink-0 flex flex-col gap-3">
                <WidgetSelector selected=selected_widget on_change=on_widget_change />

                // Add Widget button
                <Button
                    variant=ButtonVariant::Primary
                    size=ButtonSize::Medium
                    on_click=on_add_widget
                    class="w-full flex items-center justify-center gap-2"
                >
                    <Icon name=IconName::Add class="w-[18px] h-[18px]" />
                    "Add Widget"
                </Button>
            </div>

            // ========== Properties Panel with Tabs ==========
            <div class="flex-1 flex flex-col overflow-hidden">
                // Tab bar
                <div class="px-4 pt-4 shrink-0">
                    <TabBar tabs=tabs on_change=on_tab_change />
                </div>

                // Tab content (contextual to widget type)
                <WidgetConfigurationPanel
                    widget_type=Signal::derive(move || selected_widget.get())
                    active_tab=Signal::derive(move || active_tab.get())
                />
            </div>
        </div>
    }
}
