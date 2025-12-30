use leptos::prelude::*;

/// Avatar size variants
#[derive(Default, Clone, Copy, PartialEq)]
pub enum AvatarSize {
    Sm = 32,  // 32px
    #[default]
    Md = 36,  // 36px
    Lg = 40,  // 40px
    Xl = 48,  // 48px
}

impl AvatarSize {
    /// Returns Tailwind size classes
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Sm => "size-8",
            Self::Md => "size-9",
            Self::Lg => "size-10",
            Self::Xl => "size-12",
        }
    }
}

/// Avatar component for displaying user profile images
///
/// Shows a circular image with optional hover effects and border styling.
///
/// # Example
/// ```rust
/// view! {
///     <Avatar
///         src="https://example.com/avatar.jpg".into()
///         alt="User avatar".into()
///     />
///
///     <Avatar
///         src="https://example.com/avatar.jpg".into()
///         alt="User avatar".into()
///         size=AvatarSize::Lg
///         class="ring-2 ring-primary".into()
///     />
/// }
/// ```
#[component]
pub fn Avatar(
    /// Image source URL
    #[prop(into)]
    src: String,
    /// Alt text for accessibility
    #[prop(into)]
    alt: String,
    /// Size of the avatar
    #[prop(optional)]
    size: AvatarSize,
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let size_class = size.to_class();
    let combined_class = move || {
        format!(
            "bg-center bg-no-repeat aspect-square bg-cover rounded-full cursor-pointer transition-all ring-2 ring-transparent hover:ring-primary/10 {} {}",
            size_class,
            class.clone().unwrap_or_default()
        )
    };

    view! {
        <div
            class=combined_class
            style:background-image=move || format!("url(\"{}\")", src)
            role="img"
            aria-label=alt
        ></div>
    }
}
