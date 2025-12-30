use crate::features::dashboard::models::FieldType;
use crate::ui::atoms::{Icon, IconName};
use leptos::ev;
use leptos::prelude::*;

/// Field selector dropdown component
///
/// Allows users to select a field from available dataset fields.
/// Shows a dropdown list with icons indicating field types.
///
/// # Example
/// ```rust
/// view! {
///     <FieldSelector
///         label="Select Dimension"
///         fields=available_fields
///         selected=selected_field
///         on_change=on_field_change
///     />
/// }
/// ```
#[component]
pub fn FieldSelector(
    /// Label for the selector
    #[prop(into)]
    label: String,
    /// Available fields to select from
    #[prop(into)]
    fields: Signal<Vec<crate::features::dashboard::models::Field>>,
    /// Currently selected field name
    #[prop(into)]
    selected: Signal<Option<String>>,
    /// Callback when selection changes (field_name)
    on_change: Option<Callback<String>>,
    /// Placeholder text when nothing is selected
    #[prop(optional, into)]
    placeholder: String,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let placeholder_text = if placeholder.is_empty() {
        "Select field...".to_string()
    } else {
        placeholder
    };

    // Generate unique ID for this dropdown instance
    let dropdown_id = format!("field-selector-{}", label.clone().replace(' ', "-"));
    let dropdown_id_for_closure = dropdown_id.clone();

    // Node ref for click-outside detection
    let node_ref = NodeRef::new();

    // Click outside handler - closes dropdown when clicking outside
    let handle_click_outside = window_event_listener(ev::click, move |evt| {
        if !is_open.get() {
            return;
        }

        // Import JsCast trait for type conversions
        use wasm_bindgen::JsCast;

        // Get the composed path (all elements the event passed through)
        let path = evt.composed_path();
        let path_array: js_sys::Array = match path.dyn_into::<js_sys::Array>() {
            Ok(arr) => arr,
            Err(_) => return,
        };

        // Check if any element in the path has our dropdown ID
        let mut clicked_inside = false;
        for i in 0..path_array.length() {
            let item: wasm_bindgen::JsValue = path_array.get(i);

            if let Some(el) = item.dyn_ref::<web_sys::Element>() {
                // Check if this element or a parent has our data attribute
                let mut current: Option<web_sys::Element> = Some(el.clone());
                while let Some(elem) = current {
                    if elem
                        .get_attribute("data-dropdown-id")
                        .filter(|id| id == &dropdown_id_for_closure)
                        .is_some()
                    {
                        clicked_inside = true;
                        break;
                    }
                    current = elem.parent_element();
                }
                if clicked_inside {
                    break;
                }
            }
        }

        // Close dropdown only if click was outside
        if !clicked_inside {
            set_is_open.set(false);
        }
    });
    on_cleanup(move || handle_click_outside.remove());

    // Get selected field details
    let selected_field = Memo::new(move |_| {
        let selected_name = selected.get();
        let fields_list = fields.get();
        selected_name.and_then(|name| fields_list.into_iter().find(|f| f.name == name))
    });

    // Toggle dropdown
    let toggle_dropdown = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    // Select field
    let select_field = move |field_name: String| {
        set_is_open.set(false);
        if let Some(cb) = &on_change {
            cb.run(field_name);
        }
    };

    // Get icon for field type
    let get_icon = |field_type: &FieldType| -> Option<IconName> {
        match field_type {
            FieldType::Text => Some(IconName::Category),
            FieldType::Numeric => Some(IconName::TrendingUp),
            FieldType::Date => Some(IconName::Calendar),
            FieldType::Boolean => Some(IconName::Check),
        }
    };

    view! {
        <div
            class="flex flex-col gap-1.5 relative"
            node_ref=node_ref
            data-dropdown-id=dropdown_id.clone()
        >
            <label class="text-xs font-medium text-base-content/80">{label}</label>

            // Dropdown trigger button
            <button
                class="w-full min-h-[38px] rounded-lg flex items-center justify-between
                gap-2 px-3 py-2 bg-base-100
                border border-base-300
                hover:border-primary
                focus:ring-2 focus:ring-primary/20
                transition-all text-left"
                on:click=toggle_dropdown
            >
                {move || {
                    match selected_field.get() {
                        Some(field) => {
                            let icon = get_icon(&field.field_type);
                            view! {
                                <div class="flex items-center gap-2 flex-1 min-w-0">
                                    {icon
                                        .map(|i| {
                                            view! {
                                                <Icon
                                                    name=i
                                                    class="w-4 h-4 text-base-content/50 shrink-0"
                                                />
                                            }
                                        })}
                                    <span class="text-sm text-base-content truncate">
                                        {field.name.clone()}
                                    </span>
                                </div>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <span class="text-sm text-base-content/40">
                                    {placeholder_text.clone()}
                                </span>
                            }
                                .into_any()
                        }
                    }
                }}

                <Icon name=IconName::ChevronDown class="w-4 h-4 text-base-content/40 shrink-0" />
            </button>

            // Dropdown menu
            {move || {
                if is_open.get() {
                    let fields_list = fields.get();
                    if fields_list.is_empty() {
                        view! {
                            <div class="absolute z-50 w-full mt-1 bg-base-100
                            border border-base-300
                            rounded-lg shadow-lg p-2 text-center">
                                <p class="text-xs text-base-content/40">
                                    "No fields available. Upload a CSV first."
                                </p>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="absolute z-50 w-full mt-1 bg-base-100
                            border border-base-300
                            rounded-lg shadow-lg max-h-96 overflow-y-auto
                            custom-scrollbar">
                                {fields_list
                                    .into_iter()
                                    .map(|field| {
                                        let icon = get_icon(&field.field_type);
                                        let field_name_clone = field.name.clone();
                                        let is_selected_memo = {
                                            let field_name = field_name_clone.clone();
                                            Memo::new(move |_| {
                                                Some(field_name.clone()) == selected.get()
                                            })
                                        };
                                        let button_on_click = {
                                            let field_name = field_name_clone.clone();
                                            move |_| select_field(field_name.clone())
                                        };

                                        view! {
                                            <button
                                                class="w-full flex items-center gap-2 px-3 py-2
                                                text-left hover:bg-base-200
                                                transition-colors rounded"
                                                class:bg-base-200=move || is_selected_memo.get()
                                                on:click=button_on_click
                                            >
                                                {icon
                                                    .map(|i| {
                                                        view! {
                                                            <Icon
                                                                name=i
                                                                class="w-4 h-4 text-base-content/50 shrink-0"
                                                            />
                                                        }
                                                    })}
                                                <span class="text-sm text-base-content truncate">
                                                    {field.name.clone()}
                                                </span>
                                                <span class="text-[10px] text-base-content/40
                                                uppercase ml-auto">
                                                    {format!("{:?}", field.field_type)}
                                                </span>
                                            </button>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        }
                            .into_any()
                    }
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

/// Aggregation function options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AggregationFunction {
    #[default]
    Sum,
    Avg,
    Count,
    Min,
    Max,
    CountDistinct,
}

impl AggregationFunction {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Sum => "SUM",
            Self::Avg => "AVG",
            Self::Count => "COUNT",
            Self::Min => "MIN",
            Self::Max => "MAX",
            Self::CountDistinct => "COUNTD",
        }
    }
}

/// Aggregation selector dropdown component
///
/// Allows users to select an aggregation function (SUM, AVG, COUNT, etc.)
/// for measure fields.
///
/// # Example
/// ```rust
/// view! {
///     <AggregationSelector
///         selected=aggregation
///         on_change=on_aggregation_change
///     />
/// }
/// ```
#[component]
pub fn AggregationSelector(
    /// Currently selected aggregation function
    #[prop(optional)]
    selected: Signal<AggregationFunction>,
    /// Callback when selection changes
    on_change: Callback<AggregationFunction>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Generate unique ID for this dropdown instance
    let dropdown_id = "aggregation-selector".to_string();
    let dropdown_id_for_closure = dropdown_id.clone();

    // Node ref for click-outside detection
    let node_ref = NodeRef::new();

    // Click outside handler - closes dropdown when clicking outside
    let handle_click_outside = window_event_listener(ev::click, move |evt| {
        if !is_open.get() {
            return;
        }

        // Import JsCast trait for type conversions
        use wasm_bindgen::JsCast;

        // Get the composed path (all elements the event passed through)
        let path = evt.composed_path();
        let path_array: js_sys::Array = match path.dyn_into::<js_sys::Array>() {
            Ok(arr) => arr,
            Err(_) => return,
        };

        // Check if any element in the path has our dropdown ID
        let mut clicked_inside = false;
        for i in 0..path_array.length() {
            let item: wasm_bindgen::JsValue = path_array.get(i);

            if let Some(el) = item.dyn_ref::<web_sys::Element>() {
                // Check if this element or a parent has our data attribute
                let mut current: Option<web_sys::Element> = Some(el.clone());
                while let Some(elem) = current {
                    if elem
                        .get_attribute("data-dropdown-id")
                        .filter(|id| id == &dropdown_id_for_closure)
                        .is_some()
                    {
                        clicked_inside = true;
                        break;
                    }
                    current = elem.parent_element();
                }
                if clicked_inside {
                    break;
                }
            }
        }

        // Close dropdown only if click was outside
        if !clicked_inside {
            set_is_open.set(false);
        }
    });
    on_cleanup(move || handle_click_outside.remove());

    let options = vec![
        AggregationFunction::Sum,
        AggregationFunction::Avg,
        AggregationFunction::Count,
        AggregationFunction::Min,
        AggregationFunction::Max,
        AggregationFunction::CountDistinct,
    ];

    // Toggle dropdown
    let toggle_dropdown = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    // Select aggregation
    let select_aggregation = move |agg: AggregationFunction| {
        set_is_open.set(false);
        on_change.run(agg);
    };

    view! {
        <div
            class="flex items-center gap-2 relative"
            node_ref=node_ref
            data-dropdown-id=dropdown_id.clone()
        >
            <span class="text-xs text-base-content/60">"Aggregation:"</span>

            // Dropdown trigger button
            <button
                class="h-7 rounded flex items-center justify-between
                gap-1 px-2 py-1 bg-base-100
                border border-base-300
                hover:border-primary
                focus:ring-2 focus:ring-primary/20
                transition-all text-sm"
                on:click=toggle_dropdown
            >
                <span class="font-bold text-base-content">{move || selected.get().label()}</span>
                <Icon name=IconName::ChevronDown class="w-3 h-3 text-base-content/40" />
            </button>

            // Dropdown menu
            {move || {
                if is_open.get() {
                    let options_clone = options.clone();
                    view! {
                        <div
                            class="absolute z-50 mt-1 bg-base-100
                            border border-base-300
                            rounded-lg shadow-lg overflow-hidden"
                            style="left: 0; right: 0;"
                        >
                            {options_clone
                                .into_iter()
                                .map(|agg| {
                                    let is_selected = move || agg == selected.get();
                                    view! {
                                        <button
                                            class="w-full flex items-center gap-2 px-3 py-1.5
                                            text-left hover:bg-base-200
                                            transition-colors text-sm"
                                            class:bg-base-200=move || is_selected()
                                            on:click=move |_| select_aggregation(agg)
                                        >
                                            <span class="font-bold text-base-content">
                                                {agg.label()}
                                            </span>
                                        </button>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
