//! Style configuration panel molecule
//!
//! Interactive style configuration forms for widget customization.
//! Shows appropriate controls based on widget type.
//! Supports bidirectional binding with selected widget using JSON serialization.

use crate::features::dashboard::{models::Widget, DashboardContext};
use crate::ui::molecules::style_config::*;
use leptos::prelude::*;

/// Style configuration panel component
///
/// Shows interactive style controls for customizing widget appearance.
/// The controls shown depend on the widget type.
///
/// # Binding Behavior
/// - If a widget is selected: Loads and saves its style options configuration
/// - If no widget is selected: Shows default controls for new widget creation
#[component]
pub fn StyleConfigPanel(
    /// The widget type to show configuration for
    widget_type: crate::features::dashboard::models::WidgetType,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    // Get selected widget
    let selected_widget = Memo::new(move |_| {
        dashboard
            .selected_widget_id
            .get()
            .and_then(|id| dashboard.get_widgets().into_iter().find(|w| w.id == id))
    });

    // Panel header
    let header = view! {
        <div class="flex items-center justify-between">
            <h4 class="text-sm font-semibold text-base-content">"Style Configuration"</h4>
            <div class="flex items-center gap-2">
                {move || {
                    if selected_widget.get().is_some() {
                        view! {
                            <span class="badge badge-success badge-xs">"Editing"</span>
                        }.into_any()
                    } else {
                        view! {
                            <span class="badge badge-neutral badge-xs">"New"</span>
                        }.into_any()
                    }
                }}
                <span class="text-xs text-base-content/50">{widget_type.display_name()}</span>
            </div>
        </div>
    };

    // Route to appropriate style config implementation based on widget type
    let controls = match widget_type {
        crate::features::dashboard::models::WidgetType::Line => {
            render_style_config::<LineStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Bar => {
            render_style_config::<BarStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Pie => {
            render_style_config::<PieStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Kpi => {
            render_style_config::<KpiStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Table => {
            render_style_config::<TableStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Scatter => {
            render_style_config::<ScatterStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Area => {
            render_style_config::<AreaStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Radar => {
            render_style_config::<RadarStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Candlestick => {
            render_style_config::<CandlestickStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Heatmap => {
            render_style_config::<HeatmapStyleConfig>(selected_widget).into_any()
        }
        crate::features::dashboard::models::WidgetType::Treemap => {
            render_style_config::<TreemapStyleConfig>(selected_widget).into_any()
        }
    };

    view! {
        <div class="flex flex-col gap-4 p-4">
            {header}
            {controls}
        </div>
    }
}

/// Helper function to render style config for a specific widget type
fn render_style_config<T>(
    selected_widget: Memo<Option<Widget>>,
) -> impl IntoView
where
    T: StyleConfigUI,
{
    let dashboard = DashboardContext::use_context();

    // Parse initial options from JSON or use defaults
    let initial_options = Memo::new(move |_| {
        selected_widget
            .get()
            .and_then(|w| serde_json::from_str::<T::Options>(&w.chart_config.style_options).ok())
            .unwrap_or_default()
    });

    // Local state for UI
    let (options, set_options) = signal(initial_options.get_untracked());

    // Create save callback
    let save_callback = Callback::new(move |new_options: T::Options| {
        if let Some(widget) = selected_widget.get_untracked()
            && let Ok(json) = serde_json::to_string(&new_options)
        {
            dashboard.update_widget(&widget.id, |w| {
                w.chart_config.style_options = json;
            });
        }
    });

    // Sync when selected widget changes
    Effect::new(move |_| {
        let _id = dashboard.selected_widget_id.get();
        if let Some(widget) = untrack(|| selected_widget.get_untracked())
            && let Ok(parsed) = serde_json::from_str::<T::Options>(&widget.chart_config.style_options)
        {
            untrack(|| set_options.set(parsed));
        }
    });

    // Render widget-specific controls
    T::render_controls(options.into(), save_callback)
}
