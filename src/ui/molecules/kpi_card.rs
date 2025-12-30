use crate::ui::atoms::{Badge, BadgeSize, BadgeVariant, Icon, IconName};
use leptos::prelude::*;

/// KPI card component
///
/// Displays a key performance indicator with large value,
/// comparison badge, and optional progress bar.
///
/// # Example
/// ```rust
/// view! {
///     <KpiCard
///         title="Total Profit".into()
///         value="$84,392".into()
///         comparison="+12.5%".into()
///         subtitle="vs $75,000 last month".into()
///         progress_percent=75
///     />
/// }
/// ```
#[component]
pub fn KpiCard(
    /// Card title
    #[prop(into)]
    title: String,
    /// Main KPI value to display
    #[prop(into)]
    value: String,
    /// Comparison text (e.g., "+12.5%")
    #[prop(into)]
    comparison: String,
    /// Subtitle/explanation text
    #[prop(into)]
    subtitle: String,
    /// Optional progress percentage (0-100)
    #[prop(optional)]
    progress_percent: Option<u8>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-between p-5 bg-base-100 rounded-xl \
        shadow-sm hover:shadow-md transition-all flex-1">

            // Header
            <div class="flex justify-between items-start mb-2">
                <h3 class="text-base-content font-bold text-sm">{title}</h3>

                <Badge variant=BadgeVariant::Success size=BadgeSize::Small class="px-2 py-1">
                    <Icon name=IconName::TrendingUp class="w-[14px] h-[14px] mr-1" />
                    {comparison}
                </Badge>
            </div>

            // Main Value
            <div class="flex flex-col gap-1 my-auto py-4">
                <p class="text-4xl font-extrabold text-base-content tracking-tight">{value}</p>
                <p class="text-xs text-base-content/70">{subtitle}</p>
            </div>

            // Progress Bar (optional)
            {if let Some(percent) = progress_percent {
                view! {
                    <div class="mt-auto">
                        <div class="flex justify-between text-[10px] text-base-content/70 mb-1">
                            <span>Progress</span>
                            <span>{percent}{"%"}</span>
                        </div>
                        <div class="h-2 w-full bg-base-200 rounded-full overflow-hidden">
                            <div
                                class="h-full bg-gradient-to-r from-primary to-accent rounded-full"
                                style:width=move || format!("{}%", percent)
                            ></div>
                        </div>
                    </div>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
