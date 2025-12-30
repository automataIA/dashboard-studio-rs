use leptos::prelude::*;

/// Variants for Badge styling (DaisyUI badge component)
#[derive(Default, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl BadgeVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Neutral => "", // Default badge style
            Self::Primary => "badge-primary",
            Self::Secondary => "badge-secondary",
            Self::Accent => "badge-accent",
            Self::Info => "badge-info",
            Self::Success => "badge-success",
            Self::Warning => "badge-warning",
            Self::Error => "badge-error",
        }
    }
}

/// Size variants for Badge
#[derive(Default, Clone, Copy, PartialEq)]
pub enum BadgeSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl BadgeSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Small => "badge-sm",
            Self::Medium => "badge-md",
            Self::Large => "badge-lg",
        }
    }
}

/// Badge component - A versatile label component using DaisyUI
///
/// Displays small contextual information like statuses, counts, or labels.
///
/// # Example
/// ```rust
/// view! {
///     <Badge variant=BadgeVariant::Success>
///         "Active"
///     </Badge>
///
///     <Badge variant=BadgeVariant::Error size=BadgeSize::Large>
///         "Offline"
///     </Badge>
///
///     <Badge variant=BadgeVariant::Info>
///         <span class="icon-[lucide--bell] w-3 h-3 mr-1"></span>
///         "3 notifications"
///     </Badge>
/// }
/// ```
#[component]
pub fn Badge(
    #[prop(optional)] variant: BadgeVariant,
    #[prop(optional)] size: BadgeSize,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] outline: bool,
    children: Children,
) -> impl IntoView {
    let badge_classes = move || {
        let outline_class = if outline { "badge-outline" } else { "" };
        format!(
            "badge {} {} {} {}",
            variant.to_class(),
            size.to_class(),
            outline_class,
            class.clone().unwrap_or_default()
        )
    };

    view! {
        <span class=badge_classes>
            {children()}
        </span>
    }
}
