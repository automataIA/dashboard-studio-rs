//! Scatter chart style configuration UI

use crate::features::dashboard::config::style::ScatterStyleOptions;
use crate::ui::atoms::{Select, SelectOption, Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Scatter chart style configuration
#[derive(Clone, Copy)]
pub struct ScatterStyleConfig;

impl StyleConfigUI for ScatterStyleConfig {
    type Options = ScatterStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
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
                // Points Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Points"
                    </h5>

                    <Slider
                        label="Point Size"
                        value=Signal::derive(move || options.get().point_size)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.point_size = val;
                            on_change.run(opts);
                        })
                        min=2
                        max=20
                        unit="px"
                    />

                    <Slider
                        label="Opacity"
                        value=Signal::derive(move || options.get().opacity)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.opacity = val;
                            on_change.run(opts);
                        })
                        min=10
                        max=100
                        unit="%"
                    />
                </div>

                // Bubble Chart Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Bubble Mode"
                    </h5>

                    <Toggle
                        label="Enable Bubble Chart"
                        description="Map third dimension to bubble size"
                        checked=Signal::derive(move || options.get().show_bubble)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_bubble = val;
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
                        description="Display labels on points"
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
            </div>
        }
    }
}
