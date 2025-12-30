//! Sidebar components for the dashboard
//!
//! Modular sidebar implementation with separate components for:
//! - Dataset management
//! - Template library with categories
//! - Template export functionality
//! - Workspace (Dashboard) export/import functionality

pub mod dataset_section;
pub mod template_section;
pub mod export_actions;
pub mod workspace_actions;
pub mod left_sidebar;

// Re-export main component
pub use left_sidebar::LeftSidebar;
