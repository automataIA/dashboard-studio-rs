use crate::ui::atoms::{Button, ButtonVariant, Icon, IconName};
use crate::ui::molecules::{project_card::ProjectData, ProjectsGrid, SearchBar};
use crate::ui::organisms::Header;
use leptos::prelude::*;

/// Convert DashboardTemplate to ProjectData
fn template_to_project(
    template: &crate::features::dashboard::export::DashboardTemplate,
    storage_key: &str,
) -> ProjectData {
    // Parse timestamps
    let created = chrono::DateTime::parse_from_rfc3339(&template.metadata.created_at)
        .unwrap_or_else(|_| chrono::Utc::now().into());
    let updated = chrono::DateTime::parse_from_rfc3339(&template.metadata.exported_at)
        .unwrap_or_else(|_| chrono::Utc::now().into());

    let created_at = created.format("%Y-%m-%d").to_string();
    let updated_at = updated.format("%Y-%m-%d").to_string();

    // Auto-generate badges
    let age_hours = chrono::Utc::now().signed_duration_since(created).num_hours();
    let badges = if age_hours < 24 {
        vec!["New".into(), "Active".into()]
    } else {
        vec!["Active".into()]
    };

    // Description from first widget
    let description = template.widgets.first()
        .map(|w| w.subtitle.as_ref()
            .map(|s| format!("{}: {}", w.title, s))
            .unwrap_or_else(|| w.title.clone()))
        .unwrap_or_else(|| "Custom dashboard".into());

    // Icon from first widget
    let icon = template.widgets.first()
        .map(|w| match w.widget_type {
            crate::features::dashboard::WidgetType::Bar => "icon-[lucide--bar-chart-2]",
            crate::features::dashboard::WidgetType::Line => "icon-[lucide--trending-up]",
            crate::features::dashboard::WidgetType::Pie => "icon-[lucide--pie-chart]",
            _ => "icon-[lucide--bar-chart-2]",
        })
        .unwrap_or("icon-[lucide--bar-chart-2]")
        .to_string();

    ProjectData {
        id: storage_key.into(),
        icon,
        title: template.metadata.title.clone(),
        description,
        views: 0,
        edits: 0,
        shares: 0,
        badges,
        color: "bg-primary/10 text-primary".into(),
        created_at,
        updated_at,
    }
}

/// Projects page component
///
/// Main projects listing page with filter/sort controls, project grid,
/// and activity sidebar. Displays all user projects in a responsive grid.
///
/// # Example
/// In router:
/// ```rust
/// <Route path=path!("/projects") view=Projects />
/// ```
#[component]
pub fn Projects() -> impl IntoView {
    // Avatar URL
    let avatar_url =
        "https://ui-avatars.com/api/?name=Data+Viz&background=1C4E80&color=fff".to_string();

    // Load dashboards from localStorage
    let projects_resource = Resource::new(
        || (),
        |_| async {
            crate::features::dashboard::io::storage::list_templates()
                .map(|keys| {
                    keys.into_iter()
                        .filter_map(|key| {
                            crate::features::dashboard::io::storage::load_template(&key)
                                .ok()
                                .map(|tpl| template_to_project(&tpl, &key))
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        }
    );

    view! {
        <div class="min-h-screen bg-base-200 text-base-content font-display">
            // Header
            <Header user_avatar_url=avatar_url />

            // Main content
            <div class="container mx-auto px-4 py-6">
                // Page header
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
                    <div>
                        <h1 class="text-2xl font-bold">"Projects"</h1>
                        <p class="text-sm text-base-content/70">
                            "Manage your data visualization projects"
                        </p>
                    </div>
                    <Button variant=ButtonVariant::Primary on_click=Callback::new(|_| {})>
                        <Icon name=IconName::Add class="w-4 h-4" />
                        "New Project"
                    </Button>
                </div>

                // Filter/sort controls
                <div class="flex flex-wrap gap-4 mb-6">
                    <SearchBar placeholder=String::from("Search projects...") />

                    <select class="select select-bordered select-sm">
                        <option>"All Projects"</option>
                        <option>"Active"</option>
                        <option>"Archived"</option>
                    </select>

                    <select class="select select-bordered select-sm">
                        <option>"Recent"</option>
                        <option>"A-Z"</option>
                        <option>"Z-A"</option>
                    </select>

                </div>

                // Projects grid
                <Suspense fallback=|| view! { <p class="text-center py-8">"Loading projects..."</p> }>
                    {move || {
                        projects_resource.get().map(|projects| {
                            view! {
                                <ProjectsGrid
                                    projects=projects
                                    on_project_click=Callback::new(|id| {
                                        log::debug!("Clicked project: {}", id);
                                        // TODO: Navigate to /dashboard with id
                                    })
                                />
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
