use leptos::prelude::*;

/// Team member size variants
#[derive(Default, Clone, Copy, PartialEq)]
pub enum TeamMemberSize {
    Sm = 24,  // 24px
    Md = 32,  // 32px
    #[default]
    Lg = 40,  // 40px
}

impl TeamMemberSize {
    /// Returns size class for avatar
    pub fn to_size_class(&self) -> &'static str {
        match self {
            Self::Sm => "w-6 h-6",
            Self::Md => "w-8 h-8",
            Self::Lg => "w-10 h-10",
        }
    }

    /// Returns indicator size class
    pub fn to_indicator_class(&self) -> &'static str {
        match self {
            Self::Sm => "w-2 h-2",
            Self::Md => "w-2.5 h-2.5",
            Self::Lg => "w-3 h-3",
        }
    }
}

/// TeamMember molecule component
///
/// Displays a team member avatar with optional online indicator and name.
/// Useful for avatar stacks and team displays.
///
/// # Example
/// ```rust
/// view! {
///     <TeamMember
///         name="Alice Johnson".into()
///         avatar_url="https://ui-avatars.com/api/?name=Alice+Johnson&background=3B82F6&color=fff".into()
///         online=true
///         size=TeamMemberSize::Md
///     />
/// }
/// ```
#[component]
pub fn TeamMember(
    /// Team member name
    #[prop(into)]
    name: String,
    /// Avatar image URL
    #[prop(into)]
    avatar_url: String,
    /// Online status indicator
    #[prop(default = false)]
    online: bool,
    /// Avatar size
    #[prop(optional)]
    size: TeamMemberSize,
    /// Show name label
    #[prop(default = false)]
    show_name: bool,
) -> impl IntoView {
    let size_class = size.to_size_class();
    let indicator_class = size.to_indicator_class();

    view! {
        <div class="relative group">
            // Avatar
            <img
                src=avatar_url
                alt=name.clone()
                class=format!("{} rounded-full object-cover border-2 border-base-100", size_class)
                title=name.clone()
            />

            // Online indicator
            {move || {
                if online {
                    view! {
                        <span class=format!(
                            "{} absolute bottom-0 right-0 bg-success rounded-full border-2 border-base-100",
                            indicator_class
                        )></span>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}

            // Tooltip (always shown if show_name is true)
            {show_name.then(|| {
                view! {
                    <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-1 px-2 py-1 bg-base-300 text-base-content text-xs rounded whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
                        {name}
                    </div>
                }
            })}
        </div>
    }
}
