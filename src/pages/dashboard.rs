use crate::features::dashboard::{csv_upload::CsvUploadManager, DashboardContext, components::LeftSidebar};
use crate::ui::atoms::ToastContainer;
use crate::ui::organisms::data::*;
use crate::ui::organisms::*;
use crate::context::ToastContext;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Dashboard page component
///
/// Main application dashboard page that composes all organisms:
/// Header, LeftSidebar, Canvas area (CanvasHeader + CanvasGrid), and RightSidebar.
///
/// Now uses DashboardContext for state management instead of local mock data.
///
/// # Example
/// In router:
/// ```rust
/// <Route path=path!("/dashboard") view=Dashboard />
/// ```
#[component]
pub fn Dashboard() -> impl IntoView {
    // Initialize dashboard context
    let dashboard = DashboardContext::provide();

    // Initialize toast context for notifications
    let _toast = ToastContext::provide();

    // Initialize with empty state (users upload their own CSV files)
    dashboard.init_empty();

    // Create CSV upload manager
    let upload_manager = CsvUploadManager::new(dashboard);

    // Set up auto-save with 2-second debounce
    let debounce_timer: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

    Effect::new(move |_| {
        // Track all state changes by directly accessing the signals (not get_untracked)
        let _ = dashboard.title.get();

        // Access signals directly to trigger effect on changes
        let widgets_signal = dashboard.widgets;
        let _ = widgets_signal.get();

        let layers_signal = dashboard.layers;
        let _ = layers_signal.get();

        // Clear existing timer
        let timer = debounce_timer.clone();
        if let Some(id) = *timer.borrow() {
            web_sys::window().unwrap().clear_timeout_with_handle(id);
        }

        // Set new timer (2 seconds)
        let dashboard_clone = dashboard;
        let callback = Closure::once_into_js(Box::new(move || {
            dashboard_clone.save_to_storage();
        }));

        let timeout_id = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                2000,
            )
            .unwrap();

        *timer.borrow_mut() = Some(timeout_id);
    });

    // Derive data for components from context
    let datasets = Memo::new(move |_| {
        dashboard
            .get_datasets()
            .into_iter()
            .map(|ds| DatasetItemData {
                id: ds.id.clone(),
                name: ds.name,
                metadata: format!("{} • {}", ds.size, ds.uploaded_at),
                active: ds.active,
                fields: ds
                    .fields
                    .into_iter()
                    .map(|f| FieldData {
                        label: f.name,
                        icon: f.field_type.icon_color().into(),
                    })
                    .collect(),
            })
            .collect::<Vec<_>>()
    });

    let templates = Memo::new(move |_| {
        use crate::features::dashboard::models::TemplateCategory;

        dashboard
            .get_templates()
            .into_iter()
            .map(|t| {
                let category_name = match t.category {
                    TemplateCategory::Generic => "Generic",
                    TemplateCategory::Business => "Business",
                    TemplateCategory::Sales => "Sales",
                    TemplateCategory::Finance => "Finance",
                }
                .into();

                // Check if template has KPI widgets
                let has_kpi = t.widgets.iter().any(|w| w.widget_type == crate::features::dashboard::WidgetType::Kpi);

                TemplateData {
                    id: t.id.clone(),
                    title: t.title,
                    preview_type: match t.preview_type {
                        crate::features::dashboard::WidgetType::Bar => "bar",
                        crate::features::dashboard::WidgetType::Kpi => "kpi",
                        crate::features::dashboard::WidgetType::Line => "line",
                        crate::features::dashboard::WidgetType::Pie => "pie",
                        crate::features::dashboard::WidgetType::Table => "table",
                        _ => "line", // Default fallback
                    }
                    .into(),
                    category_name,
                    has_kpi,
                }
            })
            .collect::<Vec<_>>()
    });

    let layers = Memo::new(move |_| {
        dashboard
            .get_layers()
            .into_iter()
            .map(|l| LayerData {
                id: l.widget_id, // Use widget_id instead of layer_id for deletion
                label: l.label.clone(),
                icon: Box::leak(l.icon.into_boxed_str()),
                active: l.visible,
            })
            .collect::<Vec<_>>()
    });

    let (ai_prompt, _set_ai_prompt) = signal(String::new());
    let (active_tab, _set_active_tab) = signal("data".to_string());

    // Avatar URL (using a placeholder)
    let avatar_url =
        "https://ui-avatars.com/api/?name=Data+Viz&background=1C4E80&color=fff".to_string();

    view! {
        <div class="h-screen flex flex-col bg-base-100 text-base-content font-display overflow-hidden">

            // Toast notifications container
            <ToastContainer />

            // Header
            <Header user_avatar_url=avatar_url />

            // Main content area with sidebars and canvas
            <div class="flex flex-1 overflow-hidden relative">

                // Left Sidebar (hidden on mobile, visible on md+)
                <aside class="w-64 flex flex-col border-r border-base-300
                bg-base-200 overflow-y-auto shrink-0 z-20
                transition-all duration-300 absolute md:relative
                -translate-x-full md:translate-x-0 h-full
                shadow-xl md:shadow-none">
                    <LeftSidebar datasets=datasets templates=templates upload_manager />
                </aside>

                // Main Canvas
                <main class="flex-1 flex flex-col min-w-0 bg-base-100
                relative overflow-hidden">
                    <CanvasHeader
                        title=dashboard.title
                        last_edited=dashboard.last_edited
                        auto_saved=dashboard.auto_saved
                    />
                    <div class=move || {
                        format!(
                            "flex-1 overflow-auto p-6 md:p-8 relative {}",
                            if dashboard.grid_view_active.get() { "bg-grid-pattern" } else { "" },
                        )
                    }>
                        <CanvasGrid />
                    </div>
                </main>

                // Right Sidebar (hidden on smaller screens)
                <aside class="w-80 bg-base-200 border-l border-base-300
                flex flex-col shrink-0 overflow-hidden z-20 hidden lg:flex shadow-xl">
                    <RightSidebar
                        _ai_prompt=ai_prompt.into()
                        layers=layers
                        _active_tab=active_tab
                    />
                </aside>
            </div>
        </div>
    }
}
