use crate::ui::atoms::{Button, ButtonSize, ButtonVariant, Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Type alias for chart type change handler
type ChartTypeHandler = Callback<ChartType>;

/// Chart type enumeration
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum ChartType {
    #[default]
    Line,
    Bar,
    Pie,
    Scatter,
    Area,
    Radar,
    Candlestick,
    Heatmap,
    Treemap,
    Table,
}

impl ChartType {
    /// Returns the icon for this chart type
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
            Self::Table => IconName::TableChart,
        }
    }
}

/// Chart type selector component
///
/// Grid of buttons for selecting chart types.
/// Used in the inspector Data tab to choose visualization type.
///
/// # Example
/// ```rust
/// let on_change = |chart_type: ChartType| {
///     // Handle chart type change
/// };
///
/// view! {
///     <ChartTypeSelector
///         selected=ChartType::Line
///         on_change=Some(on_change)
///     />
/// }
/// ```
#[component]
pub fn ChartTypeSelector(
    /// Currently selected chart type
    #[prop(optional)]
    selected: ChartType,
    /// Optional change handler
    #[prop(optional)]
    on_change: Option<ChartTypeHandler>,
) -> impl IntoView {
    let basic_charts = vec![
        ChartType::Line,
        ChartType::Bar,
        ChartType::Pie,
        ChartType::Scatter,
        ChartType::Area,
    ];

    let advanced_charts = vec![
        ChartType::Radar,
        ChartType::Candlestick,
        ChartType::Heatmap,
        ChartType::Treemap,
        ChartType::Table,
    ];

    view! {
        <div class="flex flex-col gap-3">
            // Basic Charts
            <div>
                <p class="text-[10px] text-base-content/40 font-semibold mb-1.5 ml-1">Basic</p>
                <div class="grid grid-cols-5 gap-1.5">
                    {basic_charts
                        .into_iter()
                        .map(|chart_type| {
                            let is_selected = selected == chart_type;
                            let click_cb: Callback<MouseEvent> = Callback::new(move |_| {
                                if let Some(handler) = &on_change {
                                    handler.run(chart_type);
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
                                    title=format!("{:?}", chart_type)
                                >
                                    <Icon name=chart_type.icon() class="w-5 h-5" />
                                </Button>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>

            // Advanced & Financial Charts
            <div class="mt-1">
                <p class="text-[10px] text-base-content/40 font-semibold mb-1.5 ml-1">
                    Advanced & Financial
                </p>
                <div class="grid grid-cols-5 gap-1.5">
                    {advanced_charts
                        .into_iter()
                        .map(|chart_type| {
                            let is_selected = selected == chart_type;
                            let click_cb: Callback<MouseEvent> = Callback::new(move |_| {
                                if let Some(handler) = &on_change {
                                    handler.run(chart_type);
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
                                    title=format!("{:?}", chart_type)
                                >
                                    <Icon name=chart_type.icon() class="w-5 h-5" />
                                </Button>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
