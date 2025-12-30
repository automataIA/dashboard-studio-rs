use leptos::prelude::*;

/// Activity item data structure
#[derive(Clone, Debug)]
pub struct ActivityItem {
    pub user: String,
    pub user_avatar: String,
    pub action: String,
    pub project: String,
    pub timestamp: String,
    pub icon: String,
}

/// ActivityTimeline molecule component
///
/// Displays a vertical timeline of recent activities with user avatars,
/// action descriptions, and timestamps.
///
/// # Example
/// ```rust
/// let activities = vec![
///     ActivityItem {
///         user: "Alice Johnson".into(),
///         user_avatar: "https://ui-avatars.com/api/?name=Alice+Johnson&background=3B82F6&color=fff".into(),
///         action: "edited".into(),
///         project: "Q3 Sales Dashboard".into(),
///         timestamp: "2 mins ago".into(),
///         icon: "icon-[lucide--edit]".into(),
///     },
/// ];
///
/// view! {
///     <ActivityTimeline
///         activities=activities
///         max_items=10
///     />
/// }
/// ```
#[component]
pub fn ActivityTimeline(
    /// List of activities to display
    activities: Vec<ActivityItem>,
    /// Maximum number of items to show (default: all)
    #[prop(optional)]
    max_items: Option<usize>,
) -> impl IntoView {
    let display_activities = if let Some(limit) = max_items {
        activities.into_iter().take(limit).collect::<Vec<_>>()
    } else {
        activities
    };

    view! {
        <div class="space-y-4 relative">
            // Timeline vertical line
            <div class="absolute left-4 top-0 bottom-0 w-0.5 bg-base-300"></div>

            // Activity items
            {display_activities.into_iter().map(|activity| {
                view! {
                    <div class="flex gap-3 relative">
                        // User avatar
                        <div class="relative shrink-0 z-10">
                            <img
                                src=activity.user_avatar
                                alt=activity.user.clone()
                                class="w-8 h-8 rounded-full object-cover"
                            />
                        </div>

                        // Activity content
                        <div class="flex-1 min-w-0">
                            <p class="text-xs leading-snug">
                                <span class="font-semibold text-base-content">{activity.user}</span>
                                " " {activity.action} " "
                                <span class="font-semibold text-primary">{activity.project}</span>
                            </p>
                            <p class="text-[10px] text-base-content/60 mt-0.5">
                                {activity.timestamp}
                            </p>
                        </div>

                        // Action icon
                        <div class="shrink-0">
                            <span class=format!("{} {}", "w-6 h-6 flex items-center justify-center rounded-full bg-base-200 text-primary/70", activity.icon)></span>
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
