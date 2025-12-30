//! Heatmap chart style configuration UI

use crate::features::dashboard::config::style::HeatmapStyleOptions;
use crate::ui::atoms::{Select, SelectOption, Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Heatmap chart style configuration
#[derive(Clone, Copy)]
pub struct HeatmapStyleConfig;

impl StyleConfigUI for HeatmapStyleConfig {
    type Options = HeatmapStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        let color_scale_options = Signal::from(vec![
            SelectOption {
                label: "Gradient".to_string(),
                value: "gradient".to_string(),
            },
            SelectOption {
                label: "Ordinal".to_string(),
                value: "ordinal".to_string(),
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
                // Color Scale Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Color Scale"
                    </h5>

                    <Select
                        label="Scale Type"
                        options=color_scale_options
                        selected=Signal::derive(move || options.get().color_scale.clone())
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.color_scale = val;
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
                        description="Display cell values"
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
                        label="Gap Between Cells"
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

                    <Toggle
                        label="Interactive"
                        description="Enable hover and click effects"
                        checked=Signal::derive(move || options.get().interactive)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.interactive = val;
                            on_change.run(opts);
                        })
                    />
                </div>
            </div>
        }
    }
}
