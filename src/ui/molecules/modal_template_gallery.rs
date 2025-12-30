use leptos::prelude::*;
use leptos::ev::MouseEvent;
use crate::ui::molecules::{CategoryTabs, CategoryTab, TemplateCard, PreviewType};
use crate::ui::organisms::data::TemplateData;

/// Modal template gallery component
///
/// Displays all dashboard templates in a grid with category filtering.
/// Used in the modal dialog when "VIEW ALL" is clicked.
///
/// # Props
/// - `templates`: All available templates (reactive)
/// - `on_select`: Callback when template is selected (receives template_id)
/// - `on_close`: Callback to close the modal
///
/// # Example
/// ```rust
/// view! {
///     <ModalTemplateGallery
///         templates=templates
///         on_select=Callback::new(|id| log::info!("Selected: {}", id))
///         on_close=Callback::new(|_| set_show_modal.set(false))
///     />
/// }
/// ```
#[component]
pub fn ModalTemplateGallery(
    /// All available templates
    #[prop(into)]
    templates: Signal<Vec<TemplateData>>,
    /// Template selection callback
    on_select: Callback<String>,
    /// Modal close callback (not used internally, kept for API consistency)
    #[prop(optional)]
    _on_close: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    // Category tabs state (same as TemplateSection)
    let (active_category, set_active_category) = signal(String::from("Generic"));
    let (categories, set_categories) = signal(vec![
        CategoryTab {
            label: "Generic".into(),
            active: true,
        },
        CategoryTab {
            label: "Business".into(),
            active: false,
        },
        CategoryTab {
            label: "Sales".into(),
            active: false,
        },
        CategoryTab {
            label: "Finance".into(),
            active: false,
        },
    ]);

    // Category change handler
    let on_category_change = Callback::new(move |index: usize| {
        let category = match index {
            0 => "Generic",
            1 => "Business",
            2 => "Sales",
            3 => "Finance",
            _ => "Generic",
        }
        .to_string();

        set_active_category.set(category.clone());

        // Update tab active states
        set_categories.update(|tabs| {
            for (i, tab) in tabs.iter_mut().enumerate() {
                tab.active = i == index;
            }
        });

        log::info!("Modal gallery: Category changed to: {}", category);
    });

    // Template selection handler - close modal after selection
    let on_template_click = Callback::new(move |template_id: String| {
        log::info!("Modal gallery: Template selected: {}", template_id);
        on_select.run(template_id);
        // Modal will be closed by parent component
    });

    view! {
        <div class="flex flex-col gap-6">
            // Category tabs
            <CategoryTabs tabs=categories on_change=on_category_change />

            // Templates grid - responsive columns
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {move || {
                    let active = active_category.get();
                    templates
                        .get()
                        .into_iter()
                        .filter(|t| t.category_name == active)
                        .map(|template| {
                            let preview_type = match template.preview_type.as_str() {
                                "bar" => PreviewType::Bar,
                                "pie" => PreviewType::Pie,
                                "line" => PreviewType::Line,
                                "kpi" => PreviewType::Bar, // KPI uses bar preview for now
                                _ => PreviewType::Bar,
                            };

                            view! {
                                <TemplateCard
                                    template_id=template.id.clone()
                                    title=template.title.clone()
                                    preview_type=preview_type
                                    on_click=on_template_click
                                />
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
