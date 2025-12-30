//! Data configuration panel molecule
//!
//! Interactive field selectors for configuring widget data mapping.
//! Shows appropriate selectors based on widget type.
//! Supports bidirectional binding with selected widget.

use crate::features::dashboard::{
    models::{DataMapping, FieldType, WidgetType},
    DashboardContext,
};
use crate::ui::molecules::field_selector::{
    AggregationFunction, AggregationSelector, FieldSelector,
};
use crate::ui::molecules::ColumnSelector;
use leptos::prelude::*;

/// Data configuration panel component
///
/// Shows interactive field selectors for configuring widget data mapping.
/// The selectors shown depend on the widget type.
///
/// # Binding Behavior
/// - If a widget is selected: Loads and saves its data mapping configuration
/// - If no widget is selected: Shows default controls for new widget creation
#[component]
pub fn DataConfigPanel(
    /// The widget type to show configuration for
    widget_type: WidgetType,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    // Get fields from active dataset
    let fields = Memo::new(move |_| {
        dashboard
            .get_datasets()
            .into_iter()
            .find(|ds| ds.active)
            .map(|ds| ds.fields)
            .unwrap_or_default()
    });

    // Filter fields by type
    let text_fields = Memo::new(move |_| {
        fields
            .get()
            .into_iter()
            .filter(|f| {
                matches!(
                    f.field_type,
                    FieldType::Text | FieldType::Date | FieldType::Boolean
                )
            })
            .collect::<Vec<_>>()
    });

    let numeric_fields = Memo::new(move |_| {
        fields
            .get()
            .into_iter()
            .filter(|f| f.field_type == FieldType::Numeric)
            .collect::<Vec<_>>()
    });

    // Get selected widget (if any)
    let selected_widget = Memo::new(move |_| {
        dashboard
            .selected_widget_id
            .get()
            .and_then(|id| dashboard.get_widgets().into_iter().find(|w| w.id == id))
    });

    // Initialize signals with selected widget's data mapping or defaults
    // Use get_untracked() for initialization to avoid reactive tracking warnings
    let (x_axis, set_x_axis) = signal(
        selected_widget
            .get_untracked()
            .and_then(|w| w.chart_config.data_mapping.x_axis),
    );
    let (y_axis, set_y_axis) = signal(
        selected_widget
            .get_untracked()
            .and_then(|w| w.chart_config.data_mapping.y_axis.first().cloned()),
    );
    let (category, set_category) = signal(
        selected_widget
            .get_untracked()
            .and_then(|w| w.chart_config.data_mapping.category),
    );
    let (aggregation, set_aggregation) = signal(AggregationFunction::Sum);

    // Convert to Signal type for FieldSelector using derive
    let x_axis_sig = Signal::derive(move || x_axis.get());
    let y_axis_sig = Signal::derive(move || y_axis.get());
    let category_sig = Signal::derive(move || category.get());

    // Sync signals when selected widget changes (one-way: widget → signals)
    // Using selected_widget_id as the primary trigger to avoid loops when widget list updates
    Effect::new(move |_| {
        if let (Some(_id), Some(widget)) = (
            dashboard.selected_widget_id.get(),
            untrack(move || selected_widget.get_untracked()),
        ) {
            // Update signals without triggering effects if possible, though here they are primarily for UI binding
            untrack(move || {
                set_x_axis.set(widget.chart_config.data_mapping.x_axis);
                set_y_axis.set(widget.chart_config.data_mapping.y_axis.first().cloned());
                set_category.set(widget.chart_config.data_mapping.category);
            });
        }
    });

    view! {
        <div class="flex flex-col gap-4 p-4">
            // Panel header
            <div class="flex items-center justify-between">
                <h4 class="text-sm font-semibold text-base-content">"Data Configuration"</h4>
                <div class="flex items-center gap-2">
                    {move || {
                        if selected_widget.get().is_some() {
                            Some(
                                view! {
                                    <span class="badge badge-success badge-xs">
                                        "Editing Widget"
                                    </span>
                                }
                                    .into_any(),
                            )
                        } else {
                            Some(
                                view! {
                                    <span class="badge badge-neutral badge-xs">"New Widget"</span>
                                }
                                    .into_any(),
                            )
                        }
                    }}
                    <span class="text-xs text-base-content/50">{widget_type.display_name()}</span>
                </div>
            </div>

            // No dataset message
            {move || {
                let has_fields = !fields.get().is_empty();
                if !has_fields {
                    Some(
                        view! {
                            <div class="alert alert-warning text-xs py-2">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    class="w-4 h-4 stroke-current shrink-0"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.932-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                                    ></path>
                                </svg>
                                <span>
                                    "No dataset selected. Upload a CSV file or activate a dataset to configure fields."
                                </span>
                            </div>
                        }
                            .into_any(),
                    )
                } else {
                    Some(
                        match widget_type {
                            WidgetType::Line => {
                                // Widget-specific field selectors
                                view! {
                                    <div class="flex flex-col gap-4">
                                        // X-Axis (Dimension)
                                        <FieldSelector
                                            label="X-Axis (Dimension)"
                                            fields=text_fields
                                            selected=x_axis_sig
                                            placeholder="Select dimension field..."
                                            on_change=Some(
                                                Callback::new(move |name: String| {
                                                    set_x_axis.set(Some(name.clone()));
                                                    if let Some(widget) = selected_widget.get_untracked() {
                                                        let new_mapping = DataMapping {
                                                            x_axis: Some(name),
                                                            y_axis: y_axis.get_untracked().into_iter().collect(),
                                                            category: category.get_untracked(),
                                                            ..Default::default()
                                                        };
                                                        dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                    }
                                                }),
                                            )
                                        />

                                        // Y-Axis (Measures)
                                        <div class="flex flex-col gap-2">
                                            <label class="text-xs font-semibold text-base-content/70">
                                                "Y-Axis (Measure)"
                                            </label>
                                            <div class="text-xs text-base-content/50 mb-1">
                                                "Select a numeric field for values"
                                            </div>
                                            <FieldSelector
                                                label=""
                                                fields=numeric_fields
                                                selected=y_axis_sig
                                                placeholder="Select measure field..."
                                                on_change=Some(
                                                    Callback::new(move |name: String| {
                                                        set_y_axis.set(Some(name.clone()));
                                                        if let Some(widget) = selected_widget.get_untracked() {
                                                            let new_mapping = DataMapping {
                                                                x_axis: x_axis.get_untracked(),
                                                                y_axis: vec![name],
                                                                category: category.get_untracked(),
                                                                ..Default::default()
                                                            };
                                                            dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                        }
                                                    }),
                                                )
                                            />
                                        </div>

                                        // Category (Optional)
                                        <FieldSelector
                                            label="Category (Optional - for stacking/grouping)"
                                            fields=text_fields
                                            selected=category_sig
                                            placeholder="No grouping"
                                            on_change=Some(
                                                Callback::new(move |name: String| {
                                                    set_category.set(Some(name.clone()));
                                                    if let Some(widget) = selected_widget.get_untracked() {
                                                        let new_mapping = DataMapping {
                                                            x_axis: x_axis.get_untracked(),
                                                            y_axis: y_axis.get_untracked().into_iter().collect(),
                                                            category: Some(name),
                                                            ..Default::default()
                                                        };
                                                        dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                    }
                                                }),
                                            )
                                        />

                                        // Aggregation
                                        <AggregationSelector
                                            selected=Signal::derive(move || aggregation.get())
                                            on_change=Callback::new(move |agg: AggregationFunction| {
                                                set_aggregation.set(agg);
                                            })
                                        />
                                    </div>
                                }
                                    .into_any()
                            }
                            WidgetType::Bar => {

                                view! {
                                    <div class="flex flex-col gap-4">
                                        <FieldSelector
                                            label="X-Axis (Dimension)"
                                            fields=text_fields
                                            selected=x_axis_sig
                                            placeholder="Select dimension field..."
                                            on_change=Some(
                                                Callback::new(move |name: String| {
                                                    set_x_axis.set(Some(name.clone()));
                                                    if let Some(widget) = selected_widget.get_untracked() {
                                                        let new_mapping = DataMapping {
                                                            x_axis: Some(name),
                                                            y_axis: y_axis.get_untracked().into_iter().collect(),
                                                            category: category.get_untracked(),
                                                            ..Default::default()
                                                        };
                                                        dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                    }
                                                }),
                                            )
                                        />

                                        <div class="flex flex-col gap-2">
                                            <label class="text-xs font-semibold text-base-content/70">
                                                "Y-Axis (Measure)"
                                            </label>
                                            <div class="text-xs text-base-content/50 mb-1">
                                                "Select a numeric field for values"
                                            </div>
                                            <FieldSelector
                                                label=""
                                                fields=numeric_fields
                                                selected=y_axis_sig
                                                placeholder="Select measure field..."
                                                on_change=Some(
                                                    Callback::new(move |name: String| {
                                                        set_y_axis.set(Some(name.clone()));
                                                        if let Some(widget) = selected_widget.get_untracked() {
                                                            let new_mapping = DataMapping {
                                                                x_axis: x_axis.get_untracked(),
                                                                y_axis: vec![name],
                                                                category: category.get_untracked(),
                                                                ..Default::default()
                                                            };
                                                            dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                        }
                                                    }),
                                                )
                                            />
                                        </div>

                                        <FieldSelector
                                            label="Category (Optional - for stacking)"
                                            fields=text_fields
                                            selected=category_sig
                                            placeholder="No stacking"
                                            on_change=Some(
                                                Callback::new(move |name: String| {
                                                    set_category.set(Some(name.clone()));
                                                    if let Some(widget) = selected_widget.get_untracked() {
                                                        let new_mapping = DataMapping {
                                                            x_axis: x_axis.get_untracked(),
                                                            y_axis: y_axis.get_untracked().into_iter().collect(),
                                                            category: Some(name),
                                                            ..Default::default()
                                                        };
                                                        dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                    }
                                                }),
                                            )
                                        />

                                        <AggregationSelector
                                            selected=Signal::derive(move || aggregation.get())
                                            on_change=Callback::new(move |agg: AggregationFunction| {
                                                set_aggregation.set(agg);
                                            })
                                        />
                                    </div>
                                }
                                    .into_any()
                            }
                            WidgetType::Pie => {

                                view! {
                                    <div class="flex flex-col gap-4">
                                        <FieldSelector
                                            label="Labels (Dimension)"
                                            fields=text_fields
                                            selected=x_axis_sig
                                            placeholder="Select label field..."
                                            on_change=Some(
                                                Callback::new(move |name: String| {
                                                    set_x_axis.set(Some(name.clone()));
                                                    if let Some(widget) = selected_widget.get_untracked() {
                                                        let new_mapping = DataMapping {
                                                            x_axis: Some(name),
                                                            y_axis: y_axis.get_untracked().into_iter().collect(),
                                                            category: category.get_untracked(),
                                                            ..Default::default()
                                                        };
                                                        dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                    }
                                                }),
                                            )
                                        />

                                        <div class="flex flex-col gap-2">
                                            <label class="text-xs font-semibold text-base-content/70">
                                                "Values (Measure)"
                                            </label>
                                            <div class="text-xs text-base-content/50 mb-1">
                                                "Select a numeric field for values"
                                            </div>
                                            <FieldSelector
                                                label=""
                                                fields=numeric_fields
                                                selected=y_axis_sig
                                                placeholder="Select measure field..."
                                                on_change=Some(
                                                    Callback::new(move |name: String| {
                                                        set_y_axis.set(Some(name.clone()));
                                                        if let Some(widget) = selected_widget.get_untracked() {
                                                            let new_mapping = DataMapping {
                                                                x_axis: x_axis.get_untracked(),
                                                                y_axis: vec![name],
                                                                category: category.get_untracked(),
                                                                ..Default::default()
                                                            };
                                                            dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                        }
                                                    }),
                                                )
                                            />
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                            WidgetType::Candlestick => {
                                // Additional signals for OHLC fields
                                let (open_field, set_open_field) = signal(
                                    selected_widget
                                        .get_untracked()
                                        .and_then(|w| w.chart_config.data_mapping.open),
                                );
                                let (close_field, set_close_field) = signal(
                                    selected_widget
                                        .get_untracked()
                                        .and_then(|w| w.chart_config.data_mapping.close),
                                );
                                let (low_field, set_low_field) = signal(
                                    selected_widget
                                        .get_untracked()
                                        .and_then(|w| w.chart_config.data_mapping.low),
                                );
                                let (high_field, set_high_field) = signal(
                                    selected_widget
                                        .get_untracked()
                                        .and_then(|w| w.chart_config.data_mapping.high),
                                );

                                // Sync OHLC fields when widget changes
                                Effect::new(move |_| {
                                    if let (Some(_id), Some(widget)) = (
                                        dashboard.selected_widget_id.get(),
                                        untrack(move || selected_widget.get_untracked()),
                                    ) {
                                        untrack(move || {
                                            set_open_field.set(widget.chart_config.data_mapping.open);
                                            set_close_field.set(widget.chart_config.data_mapping.close);
                                            set_low_field.set(widget.chart_config.data_mapping.low);
                                            set_high_field.set(widget.chart_config.data_mapping.high);
                                        });
                                    }
                                });

                                let open_sig = Signal::derive(move || open_field.get());
                                let close_sig = Signal::derive(move || close_field.get());
                                let low_sig = Signal::derive(move || low_field.get());
                                let high_sig = Signal::derive(move || high_field.get());

                                view! {
                                    <div class="flex flex-col gap-4">
                                        // Date/Time Field
                                        <FieldSelector
                                            label="Date/Time (Dimension)"
                                            fields=text_fields
                                            selected=x_axis_sig
                                            placeholder="Select date field..."
                                            on_change=Some(
                                                Callback::new(move |name: String| {
                                                    set_x_axis.set(Some(name.clone()));
                                                    if let Some(widget) = selected_widget.get_untracked() {
                                                        let new_mapping = DataMapping {
                                                            x_axis: Some(name),
                                                            open: open_field.get_untracked(),
                                                            close: close_field.get_untracked(),
                                                            low: low_field.get_untracked(),
                                                            high: high_field.get_untracked(),
                                                            ..Default::default()
                                                        };
                                                        dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                    }
                                                }),
                                            )
                                        />

                                        // Open Price Field
                                        <div class="flex flex-col gap-2">
                                            <label class="text-xs font-semibold text-base-content/70">
                                                "Open Price"
                                            </label>
                                            <FieldSelector
                                                label=""
                                                fields=numeric_fields
                                                selected=open_sig
                                                placeholder="Select open price field..."
                                                on_change=Some(
                                                    Callback::new(move |name: String| {
                                                        set_open_field.set(Some(name.clone()));
                                                        if let Some(widget) = selected_widget.get_untracked() {
                                                            let new_mapping = DataMapping {
                                                                x_axis: x_axis.get_untracked(),
                                                                open: Some(name),
                                                                close: close_field.get_untracked(),
                                                                low: low_field.get_untracked(),
                                                                high: high_field.get_untracked(),
                                                                ..Default::default()
                                                            };
                                                            dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                        }
                                                    }),
                                                )
                                            />
                                        </div>

                                        // High Price Field
                                        <div class="flex flex-col gap-2">
                                            <label class="text-xs font-semibold text-base-content/70">
                                                "High Price"
                                            </label>
                                            <FieldSelector
                                                label=""
                                                fields=numeric_fields
                                                selected=high_sig
                                                placeholder="Select high price field..."
                                                on_change=Some(
                                                    Callback::new(move |name: String| {
                                                        set_high_field.set(Some(name.clone()));
                                                        if let Some(widget) = selected_widget.get_untracked() {
                                                            let new_mapping = DataMapping {
                                                                x_axis: x_axis.get_untracked(),
                                                                open: open_field.get_untracked(),
                                                                close: close_field.get_untracked(),
                                                                low: low_field.get_untracked(),
                                                                high: Some(name),
                                                                ..Default::default()
                                                            };
                                                            dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                        }
                                                    }),
                                                )
                                            />
                                        </div>

                                        // Low Price Field
                                        <div class="flex flex-col gap-2">
                                            <label class="text-xs font-semibold text-base-content/70">
                                                "Low Price"
                                            </label>
                                            <FieldSelector
                                                label=""
                                                fields=numeric_fields
                                                selected=low_sig
                                                placeholder="Select low price field..."
                                                on_change=Some(
                                                    Callback::new(move |name: String| {
                                                        set_low_field.set(Some(name.clone()));
                                                        if let Some(widget) = selected_widget.get_untracked() {
                                                            let new_mapping = DataMapping {
                                                                x_axis: x_axis.get_untracked(),
                                                                open: open_field.get_untracked(),
                                                                close: close_field.get_untracked(),
                                                                low: Some(name),
                                                                high: high_field.get_untracked(),
                                                                ..Default::default()
                                                            };
                                                            dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                        }
                                                    }),
                                                )
                                            />
                                        </div>

                                        // Close Price Field
                                        <div class="flex flex-col gap-2">
                                            <label class="text-xs font-semibold text-base-content/70">
                                                "Close Price"
                                            </label>
                                            <FieldSelector
                                                label=""
                                                fields=numeric_fields
                                                selected=close_sig
                                                placeholder="Select close price field..."
                                                on_change=Some(
                                                    Callback::new(move |name: String| {
                                                        set_close_field.set(Some(name.clone()));
                                                        if let Some(widget) = selected_widget.get_untracked() {
                                                            let new_mapping = DataMapping {
                                                                x_axis: x_axis.get_untracked(),
                                                                open: open_field.get_untracked(),
                                                                close: Some(name),
                                                                low: low_field.get_untracked(),
                                                                high: high_field.get_untracked(),
                                                                ..Default::default()
                                                            };
                                                            dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                        }
                                                    }),
                                                )
                                            />
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                            WidgetType::Table => {
                                // Signal for selected columns
                                let (selected_columns, set_selected_columns) = signal(
                                    selected_widget
                                        .get_untracked()
                                        .map(|w| w.chart_config.data_mapping.columns.clone())
                                        .unwrap_or_default(),
                                );

                                // Sync columns when widget changes
                                Effect::new(move |_| {
                                    if let (Some(_id), Some(widget)) = (
                                        dashboard.selected_widget_id.get(),
                                        untrack(move || selected_widget.get_untracked()),
                                    ) {
                                        untrack(move || {
                                            set_selected_columns.set(widget.chart_config.data_mapping.columns.clone());
                                        });
                                    }
                                });

                                let selected_columns_sig = Signal::derive(move || selected_columns.get());

                                view! {
                                    <div class="flex flex-col gap-4">
                                        <div class="p-3 bg-info/10 border border-info/20 rounded-lg">
                                            <p class="text-xs text-info">
                                                "Select which columns to display in the table. You can choose multiple columns from your dataset."
                                            </p>
                                        </div>

                                        <ColumnSelector
                                            fields=Signal::derive(move || fields.get())
                                            selected=selected_columns_sig
                                            on_change=Callback::new(move |cols: Vec<String>| {
                                                set_selected_columns.set(cols.clone());
                                                if let Some(widget) = selected_widget.get_untracked() {
                                                    let new_mapping = DataMapping {
                                                        columns: cols,
                                                        ..Default::default()
                                                    };
                                                    dashboard.update_widget_mapping(&widget.id, new_mapping);
                                                }
                                            })
                                        />
                                    </div>
                                }
                                    .into_any()
                            }
                            _ => {

                                // Other widget types show placeholder info
                                view! {
                                    <div class="flex flex-col gap-3">
                                        <div class="p-4 bg-base-200 rounded-lg border border-base-content/20">
                                            <p class="text-xs text-base-content/70 mb-2">
                                                "Field requirements for this widget:"
                                            </p>
                                            {move || {
                                                match widget_type {
                                                    WidgetType::Scatter => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>
                                                                    "• <strong>X-Axis:</strong> Measure field (X position)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Y-Axis:</strong> Measure field (Y position)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Size:</strong> Optional measure (bubble size)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Color:</strong> Optional dimension (color groups)"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    WidgetType::Area => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>
                                                                    "• <strong>X-Axis:</strong> Dimension field (categories)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Y-Axis:</strong> 1-2 Measure fields (values)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Category:</strong> Optional dimension for stacking"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    WidgetType::Radar => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>
                                                                    "• <strong>Measures:</strong> 3+ Measure fields (radial axes)"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    WidgetType::Candlestick => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>"• <strong>Date/Time:</strong> Time dimension"</p>
                                                                <p>
                                                                    "• <strong>OHLC:</strong> 4 Measure fields (Open, High, Low, Close)"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    WidgetType::Heatmap => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>
                                                                    "• <strong>X-Axis:</strong> Dimension field (X position)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Y-Axis:</strong> Dimension field (Y position)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Value:</strong> Measure field (color intensity)"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    WidgetType::Treemap => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>
                                                                    "• <strong>Hierarchy:</strong> 1+ Dimension fields (tree structure)"
                                                                </p>
                                                                <p>
                                                                    "• <strong>Value:</strong> Measure field (rectangle size)"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    WidgetType::Kpi => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>
                                                                    "• <strong>Measure:</strong> 1 Measure field (KPI value)"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    WidgetType::Table => {
                                                        view! {
                                                            <div class="text-xs text-base-content/60 space-y-1">
                                                                <p>
                                                                    "• <strong>Columns:</strong> Select which fields to display"
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    _ => {
                                                        view! {
                                                            <p class="text-xs text-base-content/60">
                                                                "Configuration coming soon"
                                                            </p>
                                                        }
                                                            .into_any()
                                                    }
                                                }
                                            }}
                                        </div>

                                        <div class="alert alert-info text-xs py-2">
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                class="w-4 h-4 stroke-current shrink-0"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    stroke-width="2"
                                                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                                ></path>
                                            </svg>
                                            <span>
                                                "Field selectors for this widget type will be implemented in Phase 4."
                                            </span>
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                        },
                    )
                }
            }}
        </div>
    }
}
