//! Line chart style configuration UI

use crate::features::dashboard::config::style::LineStyleOptions;
use crate::ui::atoms::{Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Line chart style configuration
#[derive(Clone, Copy)]
pub struct LineStyleConfig;

impl StyleConfigUI for LineStyleConfig {
    type Options = LineStyleOptions;

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

                // Line Style Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Line Style"
                    </h5>

                    <Toggle
                        label="Smooth Curves"
                        description="Enable Bezier interpolation for smoother lines"
                        checked=Signal::derive(move || options.get().smooth)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.smooth = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Area Fill"
                        description="Fill the area under the line"
                        checked=Signal::derive(move || options.get().area_fill)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.area_fill = val;
                            on_change.run(opts);
                        })
                    />

                    <Slider
                        label="Line Width"
                        value=Signal::derive(move || options.get().line_width)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.line_width = val;
                            on_change.run(opts);
                        })
                        min=1
                        max=10
                        unit="px"
                    />
                </div>

                // Data Points Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Data Points"
                    </h5>

                    <Toggle
                        label="Show Points"
                        description="Display data points on the line"
                        checked=Signal::derive(move || options.get().show_points)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_points = val;
                            on_change.run(opts);
                        })
                    />

                    <Slider
                        label="Point Size"
                        value=Signal::derive(move || options.get().point_size)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.point_size = val;
                            on_change.run(opts);
                        })
                        min=2
                        max=10
                        unit="px"
                    />
                </div>

                // Labels Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Labels"
                    </h5>

                    <Toggle
                        label="Show Labels"
                        description="Display value labels on data points"
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
