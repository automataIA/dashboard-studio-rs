use crate::ui::atoms::{Badge, BadgeVariant, Icon, IconName};
use crate::ui::molecules::{
    activity_timeline::ActivityItem,
    team_member::{TeamMember, TeamMemberSize},
    ActivityTimeline,
};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Team member data for activity sidebar
#[derive(Clone, Debug)]
pub struct TeamMemberData {
    pub name: String,
    pub avatar_url: String,
    pub online: bool,
}

/// System status data for activity sidebar
#[derive(Clone, Debug)]
pub struct SystemStatusData {
    pub cpu_usage: u8,
    pub memory_usage: u8,
    pub storage_usage: u8,
    pub status: String,
}

/// ActivitySidebar organism component
///
/// Displays a collapsible sidebar with activity timeline, online team members,
/// and system status panel. Can be toggled in/out from the right side.
///
/// # Example
/// ```rust
/// let activities = vec![/* ... */];
/// let team_members = vec![/* ... */];
/// let system_status = SystemStatusData {
///     cpu_usage: 45,
///     memory_usage: 62,
///     storage_usage: 38,
///     status: "Operational".into(),
/// };
///
/// view! {
///     <ActivitySidebar
///         activities=activities
///         team_members=team_members
///         system_status=system_status
///         visible=true
///         on_close=Callback::new(|_| { /* close sidebar */ })
///     />
/// }
/// ```
#[component]
pub fn ActivitySidebar(
    /// List of activities to display
    activities: Vec<ActivityItem>,
    /// List of online team members
    team_members: Vec<TeamMemberData>,
    /// System status information
    system_status: SystemStatusData,
    /// Visibility state
    #[prop(default = false)]
    visible: bool,
    /// Close button handler
    on_close: Callback<MouseEvent>,
) -> impl IntoView {
    // Calculate online count
    let online_count = team_members.iter().filter(|m| m.online).count();

    // Determine status badge variant
    let status_variant = match system_status.status.as_str() {
        "Operational" => BadgeVariant::Success,
        "Degraded" => BadgeVariant::Warning,
        "Down" => BadgeVariant::Error,
        _ => BadgeVariant::Neutral,
    };

    view! {
        <div class=format!(
            "fixed inset-y-0 right-0 w-80 bg-base-100 shadow-xl transform transition-transform duration-300 z-50 overflow-y-auto {}",
            if visible { "translate-x-0" } else { "translate-x-full" },
        )>
            // Header
            <div class="sticky top-0 bg-base-100 border-b border-base-300 p-4 flex items-center justify-between z-10">
                <h2 class="text-sm font-bold uppercase tracking-wider text-base-content/70">
                    "Activity Panel"
                </h2>
                <button
                    class="btn btn-ghost btn-xs btn-circle"
                    on:click=move |ev| {
                        on_close.run(ev);
                    }
                >
                    <Icon name=IconName::Close class="w-4 h-4" />
                </button>
            </div>

            <div class="p-4 space-y-6">
                // Recent Activity Section
                <div>
                    <h3 class="text-xs font-bold uppercase tracking-wider text-base-content/60 mb-3">
                        "Recent Activity"
                    </h3>
                    <ActivityTimeline activities=activities max_items=10 />
                </div>

                // Divider
                <div class="divider divider-horizontal my-0"></div>

                // Online Team Section
                <div>
                    <h3 class="text-xs font-bold uppercase tracking-wider text-base-content/60 mb-3">
                        "Team Online"
                    </h3>
                    <div class="flex items-center gap-3">
                        // Avatar stack
                        <div class="flex -space-x-2">
                            {team_members
                                .into_iter()
                                .map(|member| {
                                    view! {
                                        <TeamMember
                                            name=member.name
                                            avatar_url=member.avatar_url
                                            online=member.online
                                            size=TeamMemberSize::Md
                                        />
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                        // Online count badge
                        <Badge variant=BadgeVariant::Success class="badge-sm">
                            {online_count}
                        </Badge>
                    </div>
                </div>

                // Divider
                <div class="divider divider-horizontal my-0"></div>

                // System Status Section
                <div>
                    <h3 class="text-xs font-bold uppercase tracking-wider text-base-content/60 mb-3">
                        "System Status"
                    </h3>

                    // Status badge
                    <div class="flex items-center gap-2 mb-4">
                        <Badge variant=status_variant class="badge-sm uppercase">
                            {system_status.status}
                        </Badge>
                    </div>

                    // Progress bars
                    <div class="space-y-3">
                        // CPU
                        <div>
                            <div class="flex justify-between text-xs mb-1">
                                <span class="text-base-content/70">
                                    {String::from("CPU Usage")}
                                </span>
                                <span class="font-semibold">
                                    {system_status.cpu_usage}{String::from("%")}
                                </span>
                            </div>
                            <progress
                                class=String::from("progress progress-primary w-full")
                                value=system_status.cpu_usage
                                max=100
                            ></progress>
                        </div>

                        // Memory
                        <div>
                            <div class="flex justify-between text-xs mb-1">
                                <span class="text-base-content/70">
                                    {String::from("Memory Usage")}
                                </span>
                                <span class="font-semibold">
                                    {system_status.memory_usage}{String::from("%")}
                                </span>
                            </div>
                            <progress
                                class=String::from("progress progress-secondary w-full")
                                value=system_status.memory_usage
                                max=100
                            ></progress>
                        </div>

                        // Storage
                        <div>
                            <div class="flex justify-between text-xs mb-1">
                                <span class="text-base-content/70">
                                    {String::from("Storage Usage")}
                                </span>
                                <span class="font-semibold">
                                    {system_status.storage_usage}{String::from("%")}
                                </span>
                            </div>
                            <progress
                                class=String::from("progress progress-accent w-full")
                                value=system_status.storage_usage
                                max=100
                            ></progress>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        // Overlay backdrop
        <div
            class=format!(
                "{} {}",
                "fixed inset-0 bg-black/50 z-40 transition-opacity duration-300",
                if visible { "opacity-100" } else { "opacity-0 pointer-events-none" },
            )
            on:click=move |ev| {
                on_close.run(ev);
            }
        ></div>
    }
}
