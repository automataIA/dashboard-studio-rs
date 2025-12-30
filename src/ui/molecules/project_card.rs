use crate::ui::atoms::{Badge, BadgeVariant, Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use serde::{Serialize, Deserialize};

/// Project card data structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub id: String,
    pub icon: String,
    pub title: String,
    pub description: String,
    pub views: u32,
    pub edits: u32,
    pub shares: u32,
    pub badges: Vec<String>,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

/// ProjectCard molecule component
///
/// Displays a project card with icon, title, description, stats, and badges.
/// Hover effects and optional click handler for navigation.
///
/// # Example
/// ```rust
/// let project = ProjectData {
///     id: "1".into(),
///     icon: "icon-[lucide--bar-chart-2]".into(),
///     title: "Q3 Sales Dashboard".into(),
///     description: "Sales performance analysis with KPIs".into(),
///     views: 1240,
///     edits: 45,
///     shares: 12,
///     badges: vec!["Active".into(), "Team".into()],
///     color: "bg-blue-100 text-blue-600".into(),
/// };
///
/// view! {
///     <ProjectCard
///         data=project
///         on_click=Some(Callback::new(|id| {
///             log::debug!("Clicked project: {}", id);
///         }))
///     />
/// }
/// ```
#[component]
pub fn ProjectCard(
    /// Project data to display
    data: ProjectData,
    /// Click handler
    on_click: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <div
            class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer"
            on:click=move |ev| {
                on_click.run(ev);
            }
        >
            <div class="card-body p-4">
                // Header with icon and actions
                <div class="flex items-start justify-between mb-3">
                    // Icon with colored background
                    <div class=format!(
                        "w-12 h-12 rounded-lg flex items-center justify-center shrink-0 {}",
                        data.color,
                    )>
                        <span class=format!("w-6 h-6 {}", data.icon)></span>
                    </div>

                    // Actions menu button
                    <button
                        class="btn btn-ghost btn-xs btn-circle"
                        on:click=|ev: MouseEvent| {
                            ev.stop_propagation();
                        }
                    >
                        <Icon name=IconName::MoreHoriz class="w-4 h-4" />
                    </button>
                </div>

                // Title and description
                <div class="mb-3">
                    <h3 class="card-title text-sm font-semibold mb-1">{data.title}</h3>
                    <p class="text-xs text-base-content/70 line-clamp-2">{data.description}</p>
                </div>

                // Stats row
                <div class="flex items-center gap-4 text-xs text-base-content/60 mb-3">
                    <div class="flex items-center gap-1">
                        <Icon name=IconName::Visibility class="w-3 h-3" />
                        <span>{data.views}</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <Icon name=IconName::MoreHoriz class="w-3 h-3" />
                        <span>{data.edits}</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <Icon name=IconName::Upload class="w-3 h-3" />
                        <span>{data.shares}</span>
                    </div>
                </div>

                // Dates row
                <div class="flex flex-col gap-1 text-[10px] text-base-content/50 mb-3">
                    <div class="flex items-center gap-1">
                        <span class="font-medium">"Created:"</span>
                        <span>{data.created_at}</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <span class="font-medium">"Modified:"</span>
                        <span>{data.updated_at}</span>
                    </div>
                </div>

                // Badges
                <div class="flex flex-wrap gap-1">
                    {data
                        .badges
                        .into_iter()
                        .map(|badge| {
                            let badge_variant = match badge.as_str() {
                                "Active" => BadgeVariant::Success,
                                "Archived" => BadgeVariant::Neutral,
                                "Featured" => BadgeVariant::Primary,
                                _ => BadgeVariant::Info,
                            };

                            view! {
                                <Badge variant=badge_variant class="badge-sm">
                                    {badge}
                                </Badge>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
