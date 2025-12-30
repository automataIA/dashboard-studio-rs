// Organism-level components for the Dashboard
// Organisms are complex compositions of molecules and atoms

pub mod data;
pub mod header;
pub mod canvas_header;
pub mod canvas_grid;
pub mod right_sidebar;
pub mod widget_configuration_panel;
pub mod activity_sidebar;

// Re-export for convenience
pub use data::*;
pub use header::*;
pub use canvas_header::*;
pub use canvas_grid::*;
pub use right_sidebar::*;
pub use widget_configuration_panel::*;
pub use activity_sidebar::*;

// Note: LeftSidebar has been moved to src/features/dashboard/components/sidebar/
// This improves code organization and reduces coupling
