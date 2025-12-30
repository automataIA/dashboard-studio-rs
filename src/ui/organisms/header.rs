use crate::config::path;
use crate::context::ThemeContext;
use crate::ui::atoms::{Avatar, AvatarSize, Divider, DividerOrientation, Icon, IconName};
use crate::ui::molecules::{NavLink, NavLinks};
use leptos::prelude::*;

/// Header organism component
///
/// Main application header with logo, navigation, search, and user controls.
/// Uses global theme context for theme switching.
///
/// # Example
/// ```rust
/// view! {
///     <Header user_avatar_url="https://example.com/avatar.jpg".into() />
/// }
/// ```
#[component]
pub fn Header(
    /// User profile avatar image URL
    #[prop(into)]
    user_avatar_url: String,
) -> impl IntoView {
    // Get global theme context
    let theme = ThemeContext::use_context();

    // Theme toggle handler
    let toggle_theme = move |_| {
        theme.toggle();
    };

    // Navigation links
    let nav_links = vec![
        NavLink {
            label: "Dashboard".into(),
            href: path("/dashboard"),
        },
        NavLink {
            label: "Projects".into(),
            href: path("/projects"),
        },
        NavLink {
            label: "Settings".into(),
            href: path("/settings"),
        },
    ];

    view! {
        <header class="sticky top-0 z-50 flex shrink-0 items-center justify-between whitespace-nowrap
        border-b border-base-300
        bg-base-200 px-6 py-3 shadow-sm">

            // Left side: Logo + Nav
            <div class="flex items-center gap-8">
                // Logo
                <a href=path("/") class="flex items-center gap-3 text-base-content cursor-pointer select-none hover:opacity-80 transition-opacity">
                    <img
                        src=path("/logo.png")
                        alt="Dashboard Studio Logo"
                        class="size-8"
                    />
                    <h2 class="text-lg font-bold leading-tight tracking-[-0.015em]">
                        "Dashboard Studio RS"
                    </h2>
                </a>

                // Navigation links (hidden on mobile, visible on md+)
                <NavLinks links=nav_links />
            </div>

            // Right side: Actions + Avatar
            <div class="flex flex-1 justify-end gap-4 items-center">
                // Vertical divider
                <Divider orientation=DividerOrientation::Vertical class="h-6" />

                // Action buttons
                <div class="flex gap-2">
                    // Theme toggle button
                    <button
                        class="btn btn-ghost btn-sm h-9 px-3"
                        on:click=toggle_theme
                        title="Theme Switcher"
                    >
                        <Icon name=IconName::Contrast class="w-5 h-5" />
                    </button>
                </div>

                // User avatar
                <Avatar
                    src=user_avatar_url
                    alt="User profile avatar"
                    size=AvatarSize::Md
                    class="border-2 border-base-300 hover:border-primary transition-colors cursor-pointer"
                />
            </div>
        </header>
    }
}
