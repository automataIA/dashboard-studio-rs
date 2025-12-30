//! Pie chart style configuration UI

use crate::features::dashboard::config::style::PieStyleOptions;
use crate::ui::atoms::{Select, SelectOption, Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Pie chart style configuration
#[derive(Clone, Copy)]
pub struct PieStyleConfig;

impl StyleConfigUI for PieStyleConfig {
    type Options = PieStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        // Create select options for inner radius
        let inner_radius_options = Signal::from(vec![
            SelectOption {
                label: "0% (Pie)".to_string(),
                value: "0%".to_string(),
            },
            SelectOption {
                label: "25%".to_string(),
                value: "25%".to_string(),
            },
            SelectOption {
                label: "50% (Donut)".to_string(),
                value: "50%".to_string(),
            },
            SelectOption {
                label: "75%".to_string(),
                value: "75%".to_string(),
            },
        ]);

        // Create select options for label position
        let label_position_options = Signal::from(vec![
            SelectOption {
                label: "Inside".to_string(),
                value: "inside".to_string(),
            },
            SelectOption {
                label: "Outside".to_string(),
                value: "outside".to_string(),
            },
        ]);

        view! {
            <div class="flex flex-col gap-4">
                // Title Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Title"
                    </h5>

                    <div class="flex flex-col gap-1.5">
                        <label class="text-xs font-medium text-base-content/80">"Chart Title"</label>
                        <input
                            type="text"
                            class="input input-sm input-bordered w-full"
                            placeholder="Enter chart title..."
                            prop:value=move || options.get().title.unwrap_or_default()
                            on:input=move |ev| {
                                let mut opts = options.get();
                                let value = event_target_value(&ev);
                                opts.title = if value.is_empty() { None } else { Some(value) };
                                on_change.run(opts);
                            }
                        />
                    </div>
                </div>

                // Pie Type Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Chart Type"
                    </h5>

                    <Select
                        label="Inner Radius"
                        options=inner_radius_options
                        selected=Signal::derive(move || options.get().inner_radius.clone())
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.inner_radius = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Rose Chart"
                        description="Enable Nightingale rose chart (radius varies by value)"
                        checked=Signal::derive(move || options.get().rose_type)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.rose_type = val;
                            on_change.run(opts);
                        })
                    />
                </div>

                // Labels Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Labels"
                    </h5>

                    <Toggle
                        label="Show Labels"
                        description="Display labels on pie slices"
                        checked=Signal::derive(move || options.get().show_labels)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_labels = val;
                            on_change.run(opts);
                        })
                    />

                    <Select
                        label="Label Position"
                        options=label_position_options
                        selected=Signal::derive(move || options.get().label_position.clone())
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.label_position = val;
                            on_change.run(opts);
                        })
                    />
                </div>

                // Style Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Style"
                    </h5>

                    <Slider
                        label="Corner Radius"
                        value=Signal::derive(move || options.get().border_radius)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.border_radius = val;
                            on_change.run(opts);
                        })
                        min=0
                        max=20
                        unit="px"
                    />
                </div>
            </div>
        }
    }
}
