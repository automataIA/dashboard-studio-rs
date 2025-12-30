use leptos::prelude::*;
use leptos_router::components::A;

/// Navigation link data
#[derive(Clone, PartialEq)]
pub struct NavLink {
    pub label: String,
    pub href: String,
}

/// Navigation links component
///
/// Displays a row of navigation links with automatic active state detection.
/// Uses Leptos Router Link component for client-side navigation.
///
/// # Example
/// ```rust
/// let links = vec![
///     NavLink {
///         label: "Dashboard".into(),
///         href: "/dashboard".into(),
///     },
///     NavLink {
///         label: "Projects".into(),
///         href: "/projects".into(),
///     },
/// ];
///
/// view! {
///     <NavLinks links=links />
/// }
/// ```
#[component]
pub fn NavLinks(
    /// List of navigation links
    links: Vec<NavLink>,
) -> impl IntoView {
    let location = leptos_router::hooks::use_location();

    view! {
        <nav class="hidden md:flex items-center gap-1 bg-base-300/30 p-1 rounded-lg">
            {links
                .into_iter()
                .map(|link| {
                    let href = link.href.clone();
                    let is_active = move || {
                        let path = location.pathname.get();
                        path == href
                    };

                    view! {
                        <A
                            href=link.href
                            attr:class=move || {
                                format!(
                                    "px-3 py-1.5 rounded-md text-sm font-medium transition-all {}",
                                    if is_active() {
                                        "bg-base-100 text-primary shadow-sm"
                                    } else {
                                        "text-base-content/60 hover:text-base-content hover:bg-base-100/30"
                                    },
                                )
                            }
                        >
                            {link.label}
                        </A>
                    }
                })
                .collect::<Vec<_>>()}
        </nav>
    }
}
