use leptos::prelude::*;

/// Divider orientation
#[derive(Default, Clone, Copy, PartialEq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Divider component for visual separation
///
/// Renders a horizontal or vertical divider line with consistent styling.
///
/// # Example
/// ```rust
/// view! {
///     // Horizontal divider (default)
///     <Divider orientation=DividerOrientation::Horizontal />
///
///     // Vertical divider
///     <Divider orientation=DividerOrientation::Vertical />
///
///     // With custom classes
///     <Divider
///         orientation=DividerOrientation::Horizontal
///         class="my-4".into()
///     />
/// }
/// ```
#[component]
pub fn Divider(
    /// Orientation of the divider
    #[prop(optional)]
    orientation: DividerOrientation,
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_classes = move || {
        let orientation_class = match orientation {
            DividerOrientation::Horizontal => "h-px w-full bg-base-300",
            DividerOrientation::Vertical => "w-px h-6 bg-base-300",
        };

        format!(
            "{} {}",
            orientation_class,
            class.clone().unwrap_or_default()
        )
    };

    view! { <div class=base_classes></div> }
}
