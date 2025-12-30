use crate::features::dashboard::FieldDragDrop;
use crate::ui::atoms::IconName;
use crate::ui::molecules::DataPill;
use leptos::prelude::*;

/// Drop zone component for data mapping
///
/// Dashed border area where data fields can be dropped.
/// Supports displaying dropped items as DataPills.
///
/// # Example
/// ```rust
/// let items = vec![
///     DataPillItem {
///         label: "Product Category".into(),
///         icon: Some(IconName::Category),
///         aggregation: None,
///     },
/// ];
///
/// view! {
///     <DropZone
///         label="Dimensions".into()
///         hint="(X-Axis)".into()
///         items=items
///     />
/// }
/// ```
#[component]
pub fn DropZone(
    /// Label text
    #[prop(into)]
    label: String,
    /// Hint text (e.g., "(X-Axis)")
    #[prop(into)]
    hint: String,
    /// Zone type identifier (e.g., "x-axis", "y-axis", "filters")
    #[prop(into)]
    zone_type: String,
    /// Optional dropped items
    #[prop(optional)]
    items: Vec<DataPillItem>,
    /// Optional field drop handler (zone_type, field_name)
    #[prop(optional)]
    on_field_drop: Option<Callback<(String, String)>>,
) -> impl IntoView {
    let zone_type_clone = zone_type.clone();
    view! {
        <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-base-content/80 flex items-center gap-1">
                {label} <span class="text-base-content/40 font-normal text-[10px]">{hint}</span>
            </label>

            <div
                class="drop-zone w-full min-h-[48px] rounded-lg flex flex-wrap items-center \
                gap-1.5 p-1.5 cursor-pointer relative group transition-all"
                on:dragover=move |ev| {
                    FieldDragDrop::on_drag_over()(ev);
                }
                on:dragleave=move |ev| {
                    FieldDragDrop::on_drag_leave()(ev);
                }
                on:drop=move |ev| {
                    let zone = zone_type_clone.clone();
                    let handler = on_field_drop;
                    FieldDragDrop::on_drop_zone(
                        "temp",
                        move |_zone_type, field_name| {
                            if let Some(cb) = &handler {
                                cb.run((zone.clone(), field_name));
                            }
                        },
                    )(ev);
                }
            >
                // Placeholder when empty
                {if items.is_empty() {
                    view! {
                        <span class="absolute inset-0 flex items-center justify-center \
                        text-[11px] text-base-content/40 pointer-events-none">
                            Drop categories here
                        </span>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }}

                // Dropped Items
                {items
                    .into_iter()
                    .map(|item| {
                        let agg = item.aggregation.clone().unwrap_or_default();
                        let icon = item.icon;
                        view! {
                            <DataPill
                                label=item.label.clone()
                                icon=icon
                                aggregation=agg
                                show_remove=true
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Data pill item structure
#[derive(Clone, PartialEq)]
pub struct DataPillItem {
    pub label: String,
    pub icon: Option<IconName>,
    pub aggregation: Option<String>,
}
