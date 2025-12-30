//! Widget configuration panel organism
//!
//! Contextual configuration panel that shows different content based on
//! the selected widget type and active tab (Data/Style/AI).

use leptos::prelude::*;
use crate::features::dashboard::models::WidgetType;
use crate::ui::molecules::data_config_panel::DataConfigPanel;
use crate::ui::molecules::style_config_panel::StyleConfigPanel;
use crate::ui::molecules::ai_config_panel::AiConfigPanel;

/// Widget configuration panel component
///
/// Routes to the appropriate config panel based on the active tab.
/// This panel is contextual to the widget type.
///
/// # Example
/// ```rust
/// view! {
///     <WidgetConfigurationPanel
///         widget_type=Signal::derive(|| WidgetType::Line)
///         active_tab=Signal::derive(|| "data".to_string())
///     />
/// }
/// ```
#[component]
pub fn WidgetConfigurationPanel(
    /// The selected widget type (reactive)
    #[prop(into)]
    widget_type: Signal<WidgetType>,
    /// Currently active tab ("data" | "style" | "ai")
    #[prop(into)]
    active_tab: Signal<String>,
) -> impl IntoView {
    let tab_content = move || {
        let current_widget = widget_type.get();
        match active_tab.get().as_str() {
            "data" => {
                view! {
                    <DataConfigPanel widget_type=current_widget />
                }.into_any()
            }
            "style" => {
                view! {
                    <StyleConfigPanel widget_type=current_widget />
                }.into_any()
            }
            "ai" => {
                view! {
                    <AiConfigPanel widget_type=current_widget />
                }.into_any()
            }
            _ => {
                view! {
                    <div class="p-4 text-center text-base-content/50">
                        "Unknown tab"
                    </div>
                }.into_any()
            }
        }
    };

    view! {
        <div class="flex-1 overflow-y-auto">
            {tab_content}
        </div>
    }
}
