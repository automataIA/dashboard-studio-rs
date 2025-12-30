use leptos::prelude::*;
//use leptos::ev::MouseEvent;
use super::project_card::{ProjectCard, ProjectData};
use crate::ui::atoms::{Icon, IconName};

/// ProjectsGrid molecule component
///
/// Displays a responsive grid of project cards with filter/sort controls.
/// Includes a "Create New" card as the last item.
///
/// # Example
/// ```rust
/// let projects = vec![
///     ProjectData {
///         id: "1".into(),
///         // ... other fields
///     },
/// ];
///
/// view! {
///     <ProjectsGrid
///         projects=projects
///         on_project_click=Some(Callback::new(|id| {
///             log::debug!("Clicked project: {}", id);
///         }))
///     />
/// }
/// ```
#[component]
pub fn ProjectsGrid(
    /// List of projects to display
    projects: Vec<ProjectData>,
    /// Callback when a project is clicked
    on_project_click: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
            // Render all project cards
            {projects
                .into_iter()
                .map(|project| {
                    let project_id = project.id.clone();
                    let click_handler = on_project_click;
                    let on_click = Callback::new(move |_| {
                        click_handler.run(project_id.clone());
                    });

                    view! { <ProjectCard data=project on_click=on_click /> }
                })
                .collect::<Vec<_>>()}
            // "Create New" card
            <div class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer border-2 border-dashed border-base-300">
                <div class="card-body p-4 flex items-center justify-center min-h-[200px]">
                    <button class="flex flex-col items-center gap-2 text-base-content/60 hover:text-primary transition-colors">
                        <div class="w-12 h-12 rounded-full bg-base-200 flex items-center justify-center">
                            <Icon name=IconName::Add class="w-6 h-6" />
                        </div>
                        <span class="text-sm font-medium">"Create New Project"</span>
                    </button>
                </div>
            </div>
        </div>
    }
}
