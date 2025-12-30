use leptos::prelude::*;

/// Theme type for selecting light/dark mode
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    /// Get the theme name as a string for data-theme attribute
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "business",
        }
    }

    /// Toggle between light and dark theme
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

/// Global theme context for managing application theme
///
/// Provides theme state and methods to change theme across the entire application.
/// Uses DaisyUI theme system with `data-theme` attribute.
///
/// # Example
/// ```rust
/// // In app root
/// let theme = ThemeContext::provide();
///
/// // In any component
/// let theme = ThemeContext::use_context();
/// view! {
///     <button on:click=move |_| theme.toggle()>"Toggle Theme"</button>
/// }
/// ```
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct ThemeContext {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

impl ThemeContext {
    /// Create a new theme context with default theme
    pub fn new() -> Self {
        let (theme, set_theme) = signal(Theme::Dark);

        // Apply theme to HTML element
        Effect::new(move |_| {
            if let Some(window) = leptos::web_sys::window()
                && let Some(document) = window.document()
                    && let Some(html) = document.document_element() {
                        let _ = html.set_attribute("data-theme", theme.get().as_str());
                    }
        });

        Self { theme, set_theme }
    }

    /// Provide theme context at component root
    ///
    /// Should be called once in the app root component
    pub fn provide() -> Self {
        let context = Self::new();
        provide_context(context);
        context
    }

    /// Get theme context in child components
    ///
    /// Will panic if called outside of a context provider
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    /// Get current theme
    #[allow(dead_code)]
    pub fn get(&self) -> Theme {
        self.theme.get()
    }

    /// Set theme to specific value
    #[allow(dead_code)]
    pub fn set(&self, theme: Theme) {
        self.set_theme.set(theme);
    }

    /// Toggle between light and dark theme
    pub fn toggle(&self) {
        self.set_theme.update(|theme| *theme = theme.toggle());
    }

    /// Check if current theme is dark
    #[allow(dead_code)]
    pub fn is_dark(&self) -> bool {
        self.theme.get() == Theme::Dark
    }

    /// Check if current theme is light
    #[allow(dead_code)]
    pub fn is_light(&self) -> bool {
        self.theme.get() == Theme::Light
    }
}
