use crate::features::dashboard::FieldDragDrop;
use crate::ui::atoms::{Icon, IconName};
use leptos::prelude::*;

/// Field type for data columns
#[derive(Default, Clone, Copy, PartialEq)]
pub enum FieldType {
    #[default]
    Dimension, // Categorical data (category icon)
    Measure, // Numerical data (123 icon)
}

impl FieldType {
    /// Returns the icon for this field type
    pub fn icon(&self) -> IconName {
        match self {
            Self::Dimension => IconName::Category,
            Self::Measure => IconName::ShowChart, // Using chart icon as measure indicator
        }
    }

    /// Returns color class for this field type
    pub fn color_class(&self) -> &'static str {
        match self {
            Self::Dimension => "text-base-content/40",
            Self::Measure => "text-success",
        }
    }
}

/// Draggable field item component
///
/// Represents a data field (column) that can be dragged to drop zones.
/// Displays as an indented row under a dataset in the sidebar.
///
/// # Example
/// ```rust
/// view! {
///     <FieldItem
///         label="Product Category".into()
///         field_type=FieldType::Dimension
///         draggable=true
///     />
///
///     <FieldItem
///         label="Total Revenue".into()
///         field_type=FieldType::Measure
///         draggable=true
///     />
/// }
/// ```
#[component]
pub fn FieldItem(
    /// Field display name
    #[prop(into)]
    label: String,
    /// Type of field (dimension or measure)
    #[prop(optional)]
    field_type: FieldType,
    /// Whether this field can be dragged
    #[prop(optional)]
    draggable: bool,
) -> impl IntoView {
    let label_for_drag = label.clone();

    view! {
        <div
            class="flex items-center gap-2 px-2 py-1 rounded text-base-content/60 \
            hover:bg-base-200 cursor-grab active:cursor-grabbing \
            draggable-field group transition-all"
            draggable=draggable
            on:dragstart=move |ev| {
                if draggable {
                    FieldDragDrop::on_drag_start(label_for_drag.clone())(ev);
                }
            }
            on:dragend=move |ev| {
                if draggable {
                    FieldDragDrop::on_drag_end()(ev);
                }
            }
        >
            <Icon
                name=field_type.icon()
                class=format!("text-[14px] {}", field_type.color_class())
            />
            <span class="text-xs">{label}</span>
        </div>
    }
}
