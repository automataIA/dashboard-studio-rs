//! Treemap chart style configuration UI

use crate::features::dashboard::config::style::TreemapStyleOptions;
use crate::ui::atoms::{Select, SelectOption, Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Treemap chart style configuration
#[derive(Clone, Copy)]
pub struct TreemapStyleConfig;

impl StyleConfigUI for TreemapStyleConfig {
    type Options = TreemapStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        let visual_mode_options = Signal::from(vec![
            SelectOption {
                label: "Squarifying".to_string(),
                value: "squarifying".to_string(),
            },
            SelectOption {
                label: "Value".to_string(),
                value: "value".to_string(),
            },
        ]);

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
                // Layout Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Layout"
                    </h5>

                    <Select
                        label="Visual Mode"
                        options=visual_mode_options
                        selected=Signal::derive(move || options.get().visual_mode.clone())
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.visual_mode = val;
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
                        description="Display node labels"
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
                        label="Gap Between Nodes"
                        value=Signal::derive(move || options.get().gap)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.gap = val;
                            on_change.run(opts);
                        })
                        min=0
                        max=10
                        unit="px"
                    />

                    <Slider
                        label="Corner Radius"
                        value=Signal::derive(move || options.get().border_radius)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.border_radius = val;
                            on_change.run(opts);
                        })
                        min=0
                        max=10
                        unit="px"
                    />

                    <Slider
                        label="Color Depth"
                        value=Signal::derive(move || options.get().color_depth)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.color_depth = val;
                            on_change.run(opts);
                        })
                        min=0
                        max=10
                        unit="levels"
                    />

                    <Toggle
                        label="Show Breadcrumbs"
                        description="Display navigation breadcrumbs"
                        checked=Signal::derive(move || options.get().show_breadcrumbs)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_breadcrumbs = val;
                            on_change.run(opts);
                        })
                    />
                </div>
            </div>
        }
    }
}
