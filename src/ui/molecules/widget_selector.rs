//! Widget selector component
//!
//! Grid-based widget type selector positioned ABOVE the tabs in RightSidebar.
//! Replaces the old ChartTypeSelector.

use crate::features::dashboard::models::WidgetType;
use crate::ui::atoms::{Button, ButtonSize, ButtonVariant, Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Type alias for widget type change handler
pub type WidgetTypeHandler = Callback<WidgetType>;

impl WidgetType {
    /// Returns the icon for this widget type
    pub fn icon(&self) -> IconName {
        match self {
            Self::Line => IconName::ShowChart,
            Self::Bar => IconName::BarChart,
            Self::Pie => IconName::PieChart,
            Self::Scatter => IconName::ScatterPlot,
            Self::Area => IconName::AreaChart,
            Self::Radar => IconName::Radar,
            Self::Candlestick => IconName::CandlestickChart,
            Self::Heatmap => IconName::Heatmap,
            Self::Treemap => IconName::Treemap,
            Self::Kpi => IconName::TrendingUp,
            Self::Table => IconName::TableChart,
        }
    }

    /// Get category for this widget type
    pub fn category(&self) -> WidgetCategory {
        match self {
            Self::Line | Self::Bar | Self::Pie | Self::Scatter | Self::Area => {
                WidgetCategory::Basic
            }
            Self::Radar | Self::Candlestick | Self::Heatmap | Self::Treemap => {
                WidgetCategory::Advanced
            }
            Self::Kpi | Self::Table => WidgetCategory::Data,
        }
    }
}

/// Widget category for grouping
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WidgetCategory {
    Basic,    // Line, Bar, Pie, Scatter, Area
    Advanced, // Radar, Candlestick, Heatmap, Treemap
    Data,     // KPI, Table
}

impl WidgetCategory {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Basic => "Basic Charts",
            Self::Advanced => "Advanced Charts",
            Self::Data => "Data Widgets",
        }
    }
}

/// Widget selector component
///
/// Grid of buttons for selecting widget types. Positioned ABOVE the tabs
/// in the RightSidebar, not inside any tab.
///
/// # Example
/// ```rust
/// let on_change = |widget_type: WidgetType| {
///     // Handle widget type change
/// };
///
/// view! {
///     <WidgetSelector
///         selected=move || WidgetType::Line
///         on_change=Some(on_change)
///     />
/// }
/// ```
#[component]
pub fn WidgetSelector(
    /// Currently selected widget type (reactive)
    #[prop(into)]
    selected: Signal<WidgetType>,
    /// Optional change handler
    #[prop(optional)]
    on_change: Option<WidgetTypeHandler>,
) -> impl IntoView {
    // Define all widget types grouped by category
    let basic_widgets = [
        WidgetType::Line,
        WidgetType::Bar,
        WidgetType::Pie,
        WidgetType::Scatter,
        WidgetType::Area,
    ];

    let advanced_widgets = [
        WidgetType::Radar,
        WidgetType::Candlestick,
        WidgetType::Heatmap,
        WidgetType::Treemap,
    ];

    let data_widgets = [WidgetType::Kpi, WidgetType::Table];

    view! {
        <div class="flex flex-col gap-3">
            // Section Header
            <div class="flex items-center gap-2 px-1">
                <h3 class="text-sm font-semibold text-base-content">"Widgets"</h3>
                <div class="flex-1 h-px bg-base-content/20"></div>
            </div>

            // Basic Charts Section
            <div>
                <p class="text-[10px] text-base-content/50 font-semibold mb-1.5 ml-1 uppercase tracking-wider">
                    {WidgetCategory::Basic.label()}
                </p>
                <div class="grid grid-cols-5 gap-1.5">
                    {move || {
                        basic_widgets
                            .iter()
                            .map(|&widget_type| {
                                let is_selected = selected.get() == widget_type;
                                let click_cb: Callback<MouseEvent> = Callback::new(move |_| {
                                    if let Some(handler) = &on_change {
                                        handler.run(widget_type);
                                    }
                                });
                                view! {
                                    <Button
                                        variant=if is_selected {
                                            ButtonVariant::Primary
                                        } else {
                                            ButtonVariant::Secondary
                                        }
                                        size=ButtonSize::Small
                                        on_click=click_cb
                                        class=if is_selected {
                                            "shadow-md shadow-primary/30 ring-2 ring-offset-1 ring-offset-base-100 ring-primary"
                                        } else {
                                            ""
                                        }
                                        title=widget_type.display_name()
                                    >
                                        <Icon name=widget_type.icon() class="w-5 h-5" />
                                    </Button>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>

            // Advanced Charts Section
            <div>
                <p class="text-[10px] text-base-content/50 font-semibold mb-1.5 ml-1 uppercase tracking-wider">
                    {WidgetCategory::Advanced.label()}
                </p>
                <div class="grid grid-cols-4 gap-1.5">
                    {move || {
                        advanced_widgets
                            .iter()
                            .map(|&widget_type| {
                                let is_selected = selected.get() == widget_type;
                                let click_cb: Callback<MouseEvent> = Callback::new(move |_| {
                                    if let Some(handler) = &on_change {
                                        handler.run(widget_type);
                                    }
                                });
                                view! {
                                    <Button
                                        variant=if is_selected {
                                            ButtonVariant::Primary
                                        } else {
                                            ButtonVariant::Secondary
                                        }
                                        size=ButtonSize::Small
                                        on_click=click_cb
                                        class=if is_selected {
                                            "shadow-md shadow-primary/30 ring-2 ring-offset-1 ring-offset-base-100 ring-primary"
                                        } else {
                                            ""
                                        }
                                        title=widget_type.display_name()
                                    >
                                        <Icon name=widget_type.icon() class="w-5 h-5" />
                                    </Button>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>

            // Data Widgets Section
            <div>
                <p class="text-[10px] text-base-content/50 font-semibold mb-1.5 ml-1 uppercase tracking-wider">
                    {WidgetCategory::Data.label()}
                </p>
                <div class="grid grid-cols-2 gap-1.5">
                    {move || {
                        data_widgets
                            .iter()
                            .map(|&widget_type| {
                                let is_selected = selected.get() == widget_type;
                                let click_cb: Callback<MouseEvent> = Callback::new(move |_| {
                                    if let Some(handler) = &on_change {
                                        handler.run(widget_type);
                                    }
                                });
                                view! {
                                    <Button
                                        variant=if is_selected {
                                            ButtonVariant::Primary
                                        } else {
                                            ButtonVariant::Secondary
                                        }
                                        size=ButtonSize::Small
                                        on_click=click_cb
                                        class=if is_selected {
                                            "shadow-md shadow-primary/30 ring-2 ring-offset-1 ring-offset-base-100 ring-primary"
                                        } else {
                                            ""
                                        }
                                        title=widget_type.display_name()
                                    >
                                        <Icon name=widget_type.icon() class="w-5 h-5" />
                                    </Button>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}
