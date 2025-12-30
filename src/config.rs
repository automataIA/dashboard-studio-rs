/// Application configuration constants
///
/// This module contains configuration values that may need to be changed
/// between different deployment environments (local, production, etc.).

/// Base path for the application when deployed in a subdirectory
///
/// - Local development: use "" (empty string, app is at root)
/// - GitHub Pages: use "/dashboard-studio-rs" (app is in subdirectory)
///
/// When changing this value, make sure to also update:
/// - Router base attribute in lib.rs
/// - Trunk --public-url flag in .github/workflows/deploy.yml
///
/// Use `--features dev-mode` with Trunk to enable local development mode
#[cfg(feature = "dev-mode")]
pub const BASE_PATH: &str = "";

/// Production build uses GitHub Pages subdirectory
#[cfg(not(feature = "dev-mode"))]
pub const BASE_PATH: &str = "/dashboard-studio-rs";

/// Helper function to build a full path with BASE_PATH
///
/// # Example
/// ```
/// use crate::config::path;
///
/// assert_eq!(path("/dashboard"), "/dashboard-studio-rs/dashboard");
/// assert_eq!(path("/"), "/dashboard-studio-rs/");
/// ```
pub fn path(relative_path: &str) -> String {
    format!("{}{}", BASE_PATH, relative_path)
}
