//! KPI widget style configuration UI

use crate::features::dashboard::config::style::KpiStyleOptions;
use crate::ui::atoms::{Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for KPI widget style configuration
#[derive(Clone, Copy)]
pub struct KpiStyleConfig;

impl StyleConfigUI for KpiStyleConfig {
    type Options = KpiStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        view! {
            <div class="flex flex-col gap-4">
                // Display Options Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Display Options"
                    </h5>

                    <Toggle
                        label="Show Trend"
                        description="Show trend indicator with arrow"
                        checked=Signal::derive(move || options.get().show_trend)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_trend = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Show Progress"
                        description="Show progress bar"
                        checked=Signal::derive(move || options.get().show_progress)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_progress = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Show Comparison"
                        description="Show comparison text"
                        checked=Signal::derive(move || options.get().show_comparison)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_comparison = val;
                            on_change.run(opts);
                        })
                    />
                </div>

                // Number Format Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Number Format"
                    </h5>

                    <Slider
                        label="Decimal Places"
                        value=Signal::derive(move || options.get().decimals)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.decimals = val;
                            on_change.run(opts);
                        })
                        min=0
                        max=4
                        unit="dec"
                    />
                </div>
            </div>
        }
    }
}
