//! Radar chart style configuration UI

use crate::features::dashboard::config::style::RadarStyleOptions;
use crate::ui::atoms::{Select, SelectOption, Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Radar chart style configuration
#[derive(Clone, Copy)]
pub struct RadarStyleConfig;

impl StyleConfigUI for RadarStyleConfig {
    type Options = RadarStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        let shape_options = Signal::from(vec![
            SelectOption {
                label: "Polygon".to_string(),
                value: "polygon".to_string(),
            },
            SelectOption {
                label: "Circle".to_string(),
                value: "circle".to_string(),
            },
        ]);

        view! {
            <div class="flex flex-col gap-4">
                // Radar Shape Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Radar Shape"
                    </h5>

                    <Select
                        label="Shape"
                        options=shape_options
                        selected=Signal::derive(move || options.get().shape.clone())
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.shape = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Show Axis Labels"
                        description="Display labels on radar axes"
                        checked=Signal::derive(move || options.get().show_axis_labels)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_axis_labels = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Split Area"
                        description="Split radar area by axis"
                        checked=Signal::derive(move || options.get().split_area)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.split_area = val;
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

                    <Slider
                        label="Border Width"
                        value=Signal::derive(move || options.get().border_width)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.border_width = val;
                            on_change.run(opts);
                        })
                        min=1
                        max=10
                        unit="px"
                    />
                </div>

                // Points Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Data Points"
                    </h5>

                    <Toggle
                        label="Show Points"
                        description="Display data points"
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
            </div>
        }
    }
}
