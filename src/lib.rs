use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod pages;
mod context;
mod features;  // Dashboard feature modules
mod config;     // Application configuration
pub mod ui; // UI component library

// Top-Level pages
use crate::pages::{home::Home, dashboard::Dashboard, projects::Projects, settings::Settings};
use crate::config::BASE_PATH;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provide global theme context
    context::ThemeContext::provide();

    view! {
        <Html attr:lang="en" attr:dir="ltr" />

        // sets the document title
        <Title text="Dashboard Studio" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router base=BASE_PATH>
            <Routes fallback=|| view! { <crate::pages::not_found::NotFound /> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/home") view=Home />
                <Route path=path!("/dashboard") view=Dashboard />
                <Route path=path!("/projects") view=Projects />
                <Route path=path!("/settings") view=Settings />
            </Routes>
        </Router>
    }
}
