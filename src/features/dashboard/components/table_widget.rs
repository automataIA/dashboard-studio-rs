use crate::features::dashboard::{
    config::style::{TableStyleOptions, TableRowHeight},
    DashboardContext,
};
use leptos::prelude::*;

/// Table Widget - HTML table (no ECharts)
///
/// Displays dataset in tabular format with configurable columns.
/// Uses real data from active dataset with column selection.
///
/// # Features
/// - Configurable column selection
/// - Responsive table layout
/// - Sorting support (style option)
/// - Pagination support (style option)
/// - Hover row effects
/// - Dark mode support
/// - Clean borders
///
/// # Example
/// ```rust
/// view! {
///     <TableWidget widget_id="widget_table" />
/// }
/// ```
#[component]
pub fn TableWidget(
    /// Widget ID to fetch from DashboardContext
    #[prop(into)]
    widget_id: String,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();

    // Get widget configuration from context
    let widget = Memo::new(move |_| {
        dashboard
            .get_widgets()
            .into_iter()
            .find(|w| w.id == widget_id)
    });

    // Get active dataset and selected columns
    let table_data = Memo::new(move |_| {
        widget.get().and_then(|w| {
            // Get active dataset
            let dataset = dashboard.get_datasets().into_iter().find(|ds| ds.active)?;

            // Get selected columns from data mapping
            let selected_columns = &w.chart_config.data_mapping.columns;

            if selected_columns.is_empty() {
                return None;
            }

            // Find column indexes
            let column_indexes: Vec<(String, usize)> = selected_columns
                .iter()
                .filter_map(|col_name| {
                    dataset
                        .fields
                        .iter()
                        .position(|f| &f.name == col_name)
                        .map(|idx| (col_name.clone(), idx))
                })
                .collect();

            if column_indexes.is_empty() {
                return None;
            }

            // Extract headers and data
            let headers: Vec<String> = column_indexes.iter().map(|(name, _)| name.clone()).collect();

            let rows: Vec<Vec<String>> = dataset
                .data
                .iter()
                .map(|row| {
                    column_indexes
                        .iter()
                        .map(|(_, idx)| {
                            row.get(*idx)
                                .map(|val| {
                                    // Format value based on type
                                    match val {
                                        serde_json::Value::String(s) => s.clone(),
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        serde_json::Value::Null => "—".to_string(),
                                        _ => val.to_string(),
                                    }
                                })
                                .unwrap_or_else(|| "—".to_string())
                        })
                        .collect()
                })
                .collect();

            Some((headers, rows))
        })
    });

    // Parse style options
    let style_options = Memo::new(move |_| {
        widget
            .get()
            .and_then(|w| serde_json::from_str::<TableStyleOptions>(&w.chart_config.style_options).ok())
            .unwrap_or_default()
    });

    view! {
        {move || {
            if let Some((headers, rows)) = table_data.get() {
                let style = style_options.get();

                // Determine row height class
                let row_height_class = match style.row_height {
                    TableRowHeight::Compact => "py-1.5",
                    TableRowHeight::Normal => "py-3",
                    TableRowHeight::Comfortable => "py-4",
                };

                Some(
                    view! {
                        <div class="w-full h-full overflow-auto custom-scrollbar">
                            <table class=format!(
                                "w-full text-sm {}",
                                if style.show_borders { "border-collapse" } else { "border-separate border-spacing-0" },
                            )>
                                <thead class=format!(
                                    "border-b {} sticky top-0 bg-base-100 z-10",
                                    if style.show_borders { "border-base-300" } else { "border-transparent" },
                                )>
                                    <tr>
                                        {headers
                                            .iter()
                                            .map(|h| {
                                                view! {
                                                    <th class=format!(
                                                        "text-left {} px-4 font-semibold text-base-content/70 bg-base-100",
                                                        row_height_class,
                                                    )>
                                                        <div class="flex items-center gap-2">
                                                            {h.clone()}
                                                            {if style.show_sorting {
                                                                Some(
                                                                    view! {
                                                                        <span class="icon-[lucide--chevrons-up-down] w-3 h-3 text-base-content/40"></span>
                                                                    }
                                                                        .into_any(),
                                                                )
                                                            } else {
                                                                None
                                                            }}
                                                        </div>
                                                    </th>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </tr>
                                </thead>
                                <tbody>
                                    {rows
                                        .iter()
                                        .enumerate()
                                        .map(|(row_idx, row)| {
                                            let row_class = if style.striped && row_idx % 2 == 1 {
                                                "bg-base-200/50"
                                            } else {
                                                "bg-base-100"
                                            };
                                            let hover_class = if style.hover {
                                                "hover:bg-base-200 transition-colors cursor-pointer"
                                            } else {
                                                ""
                                            };
                                            let border_class = if style.show_borders {
                                                "border-b border-base-300/50"
                                            } else {
                                                ""
                                            };

                                            view! {
                                                <tr class=format!("{} {} {}", row_class, hover_class, border_class)>
                                                    {row
                                                        .iter()
                                                        .map(|cell| {
                                                            view! {
                                                                <td class=format!(
                                                                    "{} px-4 text-base-content",
                                                                    row_height_class,
                                                                )>{cell.clone()}</td>
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </tbody>
                            </table>

                            // Pagination footer (if enabled)
                            {if style.show_pagination {
                                Some(
                                    view! {
                                        <div class="sticky bottom-0 bg-base-100 border-t border-base-300
                                                    px-4 py-3 flex items-center justify-between">
                                            <div class="text-xs text-base-content/60">
                                                {format!("Showing {} rows", rows.len())}
                                            </div>
                                            <div class="flex gap-2">
                                                <button class="btn btn-xs btn-ghost" disabled>
                                                    <span class="icon-[lucide--chevron-left] w-3 h-3"></span>
                                                    "Previous"
                                                </button>
                                                <button class="btn btn-xs btn-ghost" disabled>
                                                    "Next"
                                                    <span class="icon-[lucide--chevron-right] w-3 h-3"></span>
                                                </button>
                                            </div>
                                        </div>
                                    }
                                        .into_any(),
                                )
                            } else {
                                None
                            }}
                        </div>
                    }
                        .into_any(),
                )
            } else {
                Some(
                    view! {
                        <div class="w-full h-full flex items-center justify-center p-4 widget-stripes">
                            <div class="text-center">
                                <div class="icon-[lucide--table] w-12 h-12 mx-auto text-base-content/20 mb-3"></div>
                                <p class="text-sm text-base-content/60 font-medium">"No columns selected"</p>
                                <p class="text-xs text-base-content/40 mt-1">
                                    "Configure table columns in the Data tab"
                                </p>
                            </div>
                        </div>
                    }
                        .into_any(),
                )
            }
        }}
    }
}
