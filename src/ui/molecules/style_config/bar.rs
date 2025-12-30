//! Bar chart style configuration UI

use crate::features::dashboard::config::style::BarStyleOptions;
use crate::ui::atoms::{Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Bar chart style configuration
#[derive(Clone, Copy)]
pub struct BarStyleConfig;

impl StyleConfigUI for BarStyleConfig {
    type Options = BarStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        view! {
            <div class="flex flex-col gap-4">
                // Titles Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Titles"
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

                    <div class="flex flex-col gap-1.5">
                        <label class="text-xs font-medium text-base-content/80">"X-Axis Title"</label>
                        <input
                            type="text"
                            class="input input-sm input-bordered w-full"
                            placeholder="Enter x-axis title..."
                            prop:value=move || options.get().x_axis_title.unwrap_or_default()
                            on:input=move |ev| {
                                let mut opts = options.get();
                                let value = event_target_value(&ev);
                                opts.x_axis_title = if value.is_empty() { None } else { Some(value) };
                                on_change.run(opts);
                            }
                        />
                    </div>

                    <div class="flex flex-col gap-1.5">
                        <label class="text-xs font-medium text-base-content/80">"Y-Axis Title"</label>
                        <input
                            type="text"
                            class="input input-sm input-bordered w-full"
                            placeholder="Enter y-axis title..."
                            prop:value=move || options.get().y_axis_title.unwrap_or_default()
                            on:input=move |ev| {
                                let mut opts = options.get();
                                let value = event_target_value(&ev);
                                opts.y_axis_title = if value.is_empty() { None } else { Some(value) };
                                on_change.run(opts);
                            }
                        />
                    </div>
                </div>

                // Bar Layout Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Bar Layout"
                    </h5>

                    <Toggle
                        label="Stacked"
                        description="Stack bars on top of each other"
                        checked=Signal::derive(move || options.get().stacked)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.stacked = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Horizontal"
                        description="Display bars horizontally instead of vertically"
                        checked=Signal::derive(move || options.get().horizontal)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.horizontal = val;
                            on_change.run(opts);
                        })
                    />

                    <Slider
                        label="Bar Width"
                        value=Signal::derive(move || options.get().bar_width)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.bar_width = val;
                            on_change.run(opts);
                        })
                        min=10
                        max=100
                        unit="%"
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

                    <Toggle
                        label="Show Labels"
                        description="Display value labels on bars"
                        checked=Signal::derive(move || options.get().show_labels)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_labels = val;
                            on_change.run(opts);
                        })
                    />
                </div>
            </div>
        }
    }
}
