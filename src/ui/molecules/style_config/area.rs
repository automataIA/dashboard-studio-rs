//! Area chart style configuration UI

use crate::features::dashboard::config::style::AreaStyleOptions;
use crate::ui::atoms::{Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Area chart style configuration
#[derive(Clone, Copy)]
pub struct AreaStyleConfig;

impl StyleConfigUI for AreaStyleConfig {
    type Options = AreaStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        view! {
            <div class="flex flex-col gap-4">
                // Area Style Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Area Style"
                    </h5>

                    <Toggle
                        label="Smooth Curves"
                        description="Enable Bezier interpolation"
                        checked=Signal::derive(move || options.get().smooth)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.smooth = val;
                            on_change.run(opts);
                        })
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

                    <Toggle
                        label="Stacked"
                        description="Stack areas on top of each other"
                        checked=Signal::derive(move || options.get().stacked)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.stacked = val;
                            on_change.run(opts);
                        })
                    />
                </div>

                // Border Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Border"
                    </h5>

                    <Toggle
                        label="Show Border"
                        description="Display area border"
                        checked=Signal::derive(move || options.get().show_border)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_border = val;
                            on_change.run(opts);
                        })
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
