use leptos::prelude::*;
use leptos::ev::MouseEvent;
use crate::ui::atoms::{Icon, IconName};

/// Button style variants
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Accent,
    Danger,
    Icon,
}

impl ButtonVariant {
    /// Returns Tailwind classes for this variant
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Primary => {
                "bg-primary hover:bg-primary/90 text-primary-content rounded-lg shadow-lg shadow-primary/25"
            }
            Self::Secondary => {
                "bg-base-200 text-base-content/80 hover:text-base-content rounded-lg"
            }
            Self::Ghost => {
                "border border-base-300 text-base-content/80 hover:bg-base-200 hover:text-primary rounded-lg"
            }
            Self::Accent => {
                "bg-gradient-to-r from-primary to-accent hover:from-primary/90 hover:to-accent/90 text-primary-content rounded-lg shadow-lg shadow-accent/20 group"
            }
            Self::Danger => {
                "text-base-content/80 hover:text-error hover:bg-error/10 rounded-lg"
            }
            Self::Icon => {
                "border border-base-300 text-base-content/80 hover:bg-base-200 hover:text-primary rounded-lg"
            }
        }
    }
}

/// Button size variants
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    #[default]
    Small,   // h-8 px-2 text-xs
    Medium,  // h-9 px-3 text-sm
    Large,   // h-10 px-4 text-base
}

impl ButtonSize {
    /// Returns Tailwind classes for this size
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Small => "h-8 px-2 text-xs",
            Self::Medium => "h-9 px-3 text-sm",
            Self::Large => "h-10 px-4 text-base",
        }
    }
}

/// Button component with multiple variants and sizes
///
/// A versatile button component supporting different styles, sizes, icons,
/// and click handlers. Follows the design system specifications.
///
/// # Example
/// ```rust
/// view! {
///     // Primary button
///     <Button variant=ButtonVariant::Primary on_click=cb>
///         "Click me"
///     </Button>
///
///     // Button with left icon
///     <Button
///         variant=ButtonVariant::Primary
///         icon_left=IconName::Add
///         on_click=cb
///     >
///         "Add Item"
///     </Button>
///
///     // Accent button (Ask AI style)
///     <Button variant=ButtonVariant::Accent>
///         <Icon name=IconName::AskAi class="mr-1.5 group-hover:animate-pulse" />
///         "Ask AI"
///     </Button>
///
///     // Icon-only button
///     <Button
///         variant=ButtonVariant::Icon
///         icon_left=IconName::Notifications
///         title="Notifications".into()
///     />
///
///     // Disabled button
///     <Button disabled=true>
///         "Cannot click"
///     </Button>
/// }
/// ```
#[component]
pub fn Button(
    /// Button style variant
    #[prop(optional)]
    variant: ButtonVariant,
    /// Button size
    #[prop(optional)]
    size: ButtonSize,
    /// Whether the button is disabled
    #[prop(optional)]
    disabled: bool,
    /// Whether to show loading state (optional, visual only for now)
    #[prop(optional)]
    loading: bool,
    /// Icon to display on the left (before text)
    #[prop(optional)]
    icon_left: Option<IconName>,
    /// Icon to display on the right (after text)
    #[prop(optional)]
    icon_right: Option<IconName>,
    /// Optional click handler callback
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
    /// Tooltip/title attribute
    #[prop(optional, into)]
    title: Option<String>,
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: Option<String>,
    /// Button content (children)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let variant_class = variant.to_class();
    let size_class = size.to_class();

    let base_classes = move || {
        format!(
            "inline-flex items-center justify-center font-bold transition-all cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 hover:-translate-y-0.5 hover:shadow-md {} {} {}",
            variant_class,
            size_class,
            class.clone().unwrap_or_default()
        )
    };

    let click_handler = move |ev: MouseEvent| {
        if !disabled && !loading
            && let Some(cb) = &on_click {
                cb.run(ev);
            }
    };

    view! {
        <button
            class=base_classes
            disabled=disabled || loading
            on:click=click_handler
            title=title.unwrap_or_default()
        >
            {if loading {
                view! { <span class="icon-[lucide--loader-2] w-4 h-4 mr-2 animate-spin"></span> }
                    .into_any()
            } else {
                match icon_left {
                    Some(icon) => {
                        view! { <Icon name=icon class="w-[18px] h-[18px] mr-1.5" /> }.into_any()
                    }
                    None => ().into_any(),
                }
            }}

            {children.map(|c| c().into_any()).unwrap_or_else(|| ().into_any())}

            {match icon_right {
                Some(icon) => {
                    view! { <Icon name=icon class="w-[18px] h-[18px] ml-1.5" /> }.into_any()
                }
                None => ().into_any(),
            }}
        </button>
    }
}
