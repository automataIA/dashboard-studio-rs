use leptos::prelude::*;

/// Icon size variants corresponding to Tailwind spacing
#[derive(Default, Clone, Copy, PartialEq)]
pub enum IconSize {
    Sm = 16, // 16px - w-4 h-4
    Md = 20, // 20px - w-5 h-5
    #[default]
    Lg = 24, // 24px - w-6 h-6
    Xl = 32, // 32px - w-8 h-8
}

impl IconSize {
    /// Returns Tailwind width/height class for this size
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Sm => "w-4 h-4",
            Self::Md => "w-5 h-5",
            Self::Lg => "w-6 h-6",
            Self::Xl => "w-8 h-8",
        }
    }
}

/// Complete icon name enum mapping Material Symbols to Lucide
#[derive(Clone, Copy, PartialEq)]
pub enum IconName {
    // Header icons
    Analytics,
    Search,
    Contrast,
    Notifications,
    AskAi,

    // Sidebar icons
    Table,
    Add,
    Upload,
    Download,
    File,
    Delete,
    Category,
    Calendar,
    MenuBook,

    // Canvas icons
    ShowChart,
    TrendingUp,
    TrendingDown,
    PieChart,
    AddChart,
    MoreHoriz,
    Undo,
    Redo,
    GridView,
    BarChart,
    ScatterPlot,
    AreaChart,
    Radar,
    CandlestickChart,
    Heatmap,
    Treemap,
    TableChart,

    // Inspector icons
    AutoAwesome,
    Mic,
    Lightbulb,
    Visibility,
    Close,
    SmartToy,
    Sparkles,

    // Misc
    Check,
    Warning,
    Error,
    Info,
    Help,
    Hash,
    Settings,
    Filter,
    Sort,
    Schedule,
    History,
    ChevronDown,
    ChevronUp,
    ChevronRight,
    ChevronLeft,
    ArrowDown,
    ArrowUp,

    // Settings page icons
    Palette,
    Person,
    Shield,
    Link,
    ExpandMore,
    Copy,
    DragHandle,
}

impl IconName {
    /// Maps IconName to Lucide icon class name
    /// See specific icon mappings in the match statement below
    pub fn to_lucide_class(&self) -> &'static str {
        match self {
            // Header icons
            Self::Analytics => "icon-[lucide--bar-chart-3]",
            Self::Search => "icon-[lucide--search]",
            Self::Contrast => "icon-[lucide--sun-moon]",
            Self::Notifications => "icon-[lucide--bell]",
            Self::AskAi => "icon-[lucide--sparkles]",

            // Sidebar icons
            Self::Table => "icon-[lucide--table]",
            Self::Add => "icon-[lucide--plus]",
            Self::Upload => "icon-[lucide--upload]",
            Self::Download => "icon-[lucide--download]",
            Self::File => "icon-[lucide--file]",
            Self::Delete => "icon-[lucide--trash-2]",
            Self::Category => "icon-[lucide--tag]",
            Self::Calendar => "icon-[lucide--calendar]",
            Self::MenuBook => "icon-[lucide--book-open]",

            // Canvas icons
            Self::ShowChart => "icon-[lucide--trending-up]",
            Self::TrendingUp => "icon-[lucide--trending-up]",
            Self::TrendingDown => "icon-[lucide--trending-down]",
            Self::PieChart => "icon-[lucide--pie-chart]",
            Self::AddChart => "icon-[lucide--plus-circle]",
            Self::MoreHoriz => "icon-[lucide--more-horizontal]",
            Self::Undo => "icon-[lucide--undo-2]",
            Self::Redo => "icon-[lucide--redo-2]",
            Self::GridView => "icon-[lucide--grid]",
            Self::BarChart => "icon-[lucide--bar-chart-2]",
            Self::ScatterPlot => "icon-[lucide--git-commit]",
            Self::AreaChart => "icon-[lucide--line-chart]",
            Self::Radar => "icon-[lucide--network]",
            Self::CandlestickChart => "icon-[lucide--bar-chart-2]",
            Self::Heatmap => "icon-[lucide--layout-grid]",
            Self::Treemap => "icon-[lucide--layout-dashboard]",
            Self::TableChart => "icon-[lucide--table-2]",

            // Inspector icons
            Self::AutoAwesome => "icon-[lucide--sparkles]",
            Self::Mic => "icon-[lucide--mic]",
            Self::Lightbulb => "icon-[lucide--lightbulb]",
            Self::Visibility => "icon-[lucide--eye]",
            Self::Close => "icon-[lucide--x]",
            Self::SmartToy => "icon-[lucide--bot]",
            Self::Sparkles => "icon-[lucide--sparkles]",

            // Misc
            Self::Check => "icon-[lucide--check]",
            Self::Warning => "icon-[lucide--alert-triangle]",
            Self::Error => "icon-[lucide--alert-circle]",
            Self::Info => "icon-[lucide--info]",
            Self::Help => "icon-[lucide--help-circle]",
            Self::Hash => "icon-[lucide--hash]",
            Self::Settings => "icon-[lucide--settings]",
            Self::Filter => "icon-[lucide--filter]",
            Self::Sort => "icon-[lucide--arrow-up-down]",
            Self::Schedule => "icon-[lucide--clock]",
            Self::History => "icon-[lucide--history]",
            Self::ChevronDown => "icon-[lucide--chevron-down]",
            Self::ChevronUp => "icon-[lucide--chevron-up]",
            Self::ChevronRight => "icon-[lucide--chevron-right]",
            Self::ChevronLeft => "icon-[lucide--chevron-left]",
            Self::ArrowDown => "icon-[lucide--arrow-down]",
            Self::ArrowUp => "icon-[lucide--arrow-up]",

            // Settings page icons
            Self::Palette => "icon-[lucide--palette]",
            Self::Person => "icon-[lucide--user]",
            Self::Shield => "icon-[lucide--shield]",
            Self::Link => "icon-[lucide--link]",
            Self::ExpandMore => "icon-[lucide--chevron-down]",
            Self::Copy => "icon-[lucide--copy]",
            Self::DragHandle => "icon-[lucide--grip-vertical]",
        }
    }
}

/// Icon component using Iconify Lucide icons
///
/// Displays Lucide icons via Iconify dynamic icon classes.
///
/// # Example
/// ```rust
/// view! {
///     // Basic icon with default size
///     <Icon name=IconName::Search />
///
///     // Large icon with custom class
///     <Icon name=IconName::Analytics size=IconSize::Xl class="text-primary" />
///
///     // Small icon in a button
///     <button>
///         <Icon name=IconName::Close size=IconSize::Sm />
///     </button>
/// }
/// ```
#[component]
pub fn Icon(
    /// The icon to display
    name: IconName,
    /// Size of the icon (affects width/height classes)
    #[prop(optional)]
    size: IconSize,
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let size_class = size.to_class();
    let icon_class = name.to_lucide_class();
    let combined_class = move || {
        format!(
            "{} {} {}",
            icon_class,
            size_class,
            class.clone().unwrap_or_default()
        )
    };

    view! { <span class=combined_class></span> }
}
