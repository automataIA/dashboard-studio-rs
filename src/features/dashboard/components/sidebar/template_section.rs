use crate::context::ToastContext;
use crate::features::dashboard::{DashboardContext, KpiAggregation};
use crate::ui::molecules::{CategoryTabs as MoleculeCategoryTabs, KpiQuickActions, Modal, ModalTemplateGallery, PreviewType, TemplateCard};
use crate::ui::organisms::data::TemplateData;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::portal::Portal;

/// Template section component
///
/// Displays template library with category filtering and KPI Quick Actions.
/// Handles template click and KPI detection internally.
///
/// # Example
/// ```rust
/// view! {
///     <TemplateSection templates=templates />
/// }
/// ```
#[component]
pub fn TemplateSection(
    /// List of templates to display (reactive)
    #[prop(into)]
    templates: Signal<Vec<TemplateData>>,
) -> impl IntoView {
    let dashboard = DashboardContext::use_context();
    let toast = ToastContext::use_context();

    // Category tabs state
    let (active_category, set_active_category) = signal(String::from("Generic"));
    let (categories, set_categories) = signal(vec![
        crate::ui::molecules::CategoryTab {
            label: "Generic".into(),
            active: true,
        },
        crate::ui::molecules::CategoryTab {
            label: "Business".into(),
            active: false,
        },
        crate::ui::molecules::CategoryTab {
            label: "Sales".into(),
            active: false,
        },
        crate::ui::molecules::CategoryTab {
            label: "Finance".into(),
            active: false,
        },
    ]);

    // KPI Quick Actions state
    let (show_kpi_quick_actions, set_show_kpi_quick_actions) = signal(false);
    let (pending_template_id, set_pending_template_id) = signal(String::new());

    // Modal state for "VIEW ALL" gallery
    let (show_modal, set_show_modal) = signal(false);

    // Handle category change
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

        log::info!("Category changed to: {}", category);
    });

    // Template selection callback - handles KPI detection
    let on_template_click = Callback::new(move |template_id: String| {
        log::info!("Template selected: {}", template_id);

        // Find template in dashboard context
        let all_templates = dashboard.get_templates();
        let template = all_templates.iter().find(|t| t.id == template_id);

        if let Some(tpl) = template {
            // Check if template has KPI widgets
            let has_kpi = tpl.widgets.iter().any(|w| w.widget_type == crate::features::dashboard::WidgetType::Kpi);

            if has_kpi {
                // Show quick actions for KPI configuration
                log::info!("Template contains KPI widgets, showing quick actions");
                set_pending_template_id.set(template_id);
                set_show_kpi_quick_actions.set(true);
            } else {
                // No KPI, create widgets directly
                let count = dashboard.create_widgets_from_template(&template_id, None, None);

                if count > 0 {
                    log::info!("Created {} widgets from template: {}", count, template_id);
                    toast.show_success(
                        "Template Applied!",
                        &format!("Created {} widget{}", count, if count == 1 { "" } else { "s" }),
                    );
                } else {
                    log::error!("Failed to create widgets from template: {}", template_id);
                    toast.show_error("Error", "Could not create widgets from selected template");
                }
            }
        } else {
            log::error!("Template not found: {}", template_id);
            toast.show_error("Error", "Template not found");
        }
    });

    // KPI quick actions close handler
    let on_kpi_quick_actions_close = Callback::new(move |_: MouseEvent| {
        set_show_kpi_quick_actions.set(false);
        set_pending_template_id.set(String::new());
    });

    // KPI quick actions select handler
    let on_kpi_select = Callback::new(move |(field_name, aggregation, _label): (String, KpiAggregation, String)| {
        log::info!("KPI selected: {:?} - {:?}", field_name, aggregation);

        // Get pending template
        let template_id = pending_template_id.get();
        if template_id.is_empty() {
            log::error!("No pending template ID");
            return;
        }

        // Create widgets from template with KPI configuration
        let count = dashboard.create_widgets_from_template(
            &template_id,
            Some(field_name),
            Some(aggregation),
        );

        if count > 0 {
            log::info!("Created {} widgets from template: {}", count, template_id);
            toast.show_success(
                "Template Applied!",
                &format!("Created {} widget{}", count, if count == 1 { "" } else { "s" }),
            );
        } else {
            log::error!("Failed to create widgets from template: {}", template_id);
            toast.show_error("Error", "Could not create widgets from selected template");
        }

        // Close modal
        set_show_kpi_quick_actions.set(false);
        set_pending_template_id.set(String::new());
    });

    view! {
        <div class="flex flex-col gap-4">
            // View all button with improved styling
            <div class="flex items-center justify-end">
                <button
                    class="
                        flex items-center gap-1.5
                        text-primary hover:text-primary-focus
                        text-xs font-semibold
                        px-2 py-1 rounded-md
                        hover:bg-primary/5
                        transition-all
                    "
                    on:click=move |_| set_show_modal.set(true)
                >
                    <span>"VIEW ALL"</span>
                    <span class="icon-[lucide--chevron-right] w-3 h-3"></span>
                </button>
            </div>

            // Category tabs with improved spacing
            <MoleculeCategoryTabs tabs=categories on_change=on_category_change />

            // Template cards grid (filtered by active category) with improved spacing
            <div class="grid grid-cols-2 gap-3">
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

        // KPI Quick Actions Modal
        <KpiQuickActions
            show=show_kpi_quick_actions
            on_close=on_kpi_quick_actions_close
            on_select=on_kpi_select
        />

        // Template Gallery Modal (rendered in portal to escape sidebar)
        <Portal>
            <Modal
                show=show_modal.into()
                on_close=Callback::new(move |_: MouseEvent| {
                    set_show_modal.set(false);
                })
                title="All Templates".to_string()
            >
                <ModalTemplateGallery
                    templates=templates
                    on_select=on_template_click
                />
            </Modal>
        </Portal>
    }
}
