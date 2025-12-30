//! AI configuration panel molecule
//!
//! Basic AI chart generation interface.
//! Full AI integration will be implemented in a later phase.

use leptos::prelude::*;
use crate::features::dashboard::models::WidgetType;
use crate::ui::atoms::{Icon, IconName};

/// AI configuration panel component
///
/// Shows AI prompt input and generate button.
/// This is a placeholder for future AI integration.
#[component]
pub fn AiConfigPanel(
    /// The widget type (for context-aware AI suggestions)
    widget_type: WidgetType,
) -> impl IntoView {
    let (ai_prompt, set_ai_prompt) = signal(String::new());
    let (is_generating, set_is_generating) = signal(false);

    let on_generate = move |_| {
        set_is_generating.set(true);
        // TODO: Implement AI chart generation in future phase
        log::info!("AI generation requested for {:?} with prompt: {}", widget_type, ai_prompt.get());
        set_is_generating.set(false);
    };

    view! {
        <div class="flex flex-col gap-4 p-4">
            // Panel header
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <Icon name=IconName::AskAi class="w-4 h-4 text-primary" />
                    <h4 class="text-sm font-semibold text-base-content">
                        "AI Chart Creator"
                    </h4>
                </div>
                <span class="text-xs text-base-content/50">
                    {widget_type.display_name()}
                </span>
            </div>

            // Prompt input
            <div class="flex flex-col gap-2">
                <label class="text-sm font-medium text-base-content">
                    "Describe your chart"
                </label>
                <p class="text-xs text-base-content/50">
                    "Tell AI what data to visualize and how"
                </p>
                <textarea
                    class="textarea textarea-bordered textarea-sm h-24"
                    placeholder="Example: Show me monthly sales trends by region, grouped by product category..."
                    prop:value=ai_prompt
                    on:input=move |ev| {
                        set_ai_prompt.set(event_target_value(&ev));
                    }
                ></textarea>
            </div>

            // Generate button
            <button
                class="btn btn-primary btn-sm gap-2"
                prop:disabled=move || ai_prompt.get().is_empty() || is_generating.get()
                on:click=on_generate
            >
                <Icon name=IconName::Sparkles class="w-4 h-4" />
                <span class="flex-1">
                    {move || if is_generating.get() { "Generating..." } else { "Generate Chart" }}
                </span>
            </button>

            // Example prompts
            <div class="flex flex-col gap-2">
                <p class="text-xs font-medium text-base-content/70">
                    "Example prompts:"
                </p>
                <div class="flex flex-col gap-1">
                    <button
                        class="text-xs text-left text-base-content/50 hover:text-base-content/80 transition-colors"
                        on:click=move |_| {
                            set_ai_prompt.set("Create a chart showing revenue trends over the last 12 months".to_string());
                        }
                    >
                        "• Revenue trends over 12 months"
                    </button>
                    <button
                        class="text-xs text-left text-base-content/50 hover:text-base-content/80 transition-colors"
                        on:click=move |_| {
                            set_ai_prompt.set("Compare sales performance across 4 regions".to_string());
                        }
                    >
                        "• Compare sales across 4 regions"
                    </button>
                    <button
                        class="text-xs text-left text-base-content/50 hover:text-base-content/80 transition-colors"
                        on:click=move |_| {
                            set_ai_prompt.set("Show top 10 products by revenue with profit margins".to_string());
                        }
                    >
                        "• Top 10 products by revenue"
                    </button>
                </div>
            </div>

            // Info banner
            <div class="alert alert-info text-xs py-2">
                <Icon name=IconName::Info class="w-4 h-4 shrink-0" />
                <span>
                    "AI integration coming soon! This feature will use natural language processing to generate charts from your descriptions."
                </span>
            </div>
        </div>
    }
}
