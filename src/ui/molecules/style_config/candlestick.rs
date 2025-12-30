//! Candlestick chart style configuration UI

use crate::features::dashboard::config::style::CandlestickStyleOptions;
use crate::ui::atoms::{Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Candlestick chart style configuration
#[derive(Clone, Copy)]
pub struct CandlestickStyleConfig;

impl StyleConfigUI for CandlestickStyleConfig {
    type Options = CandlestickStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        view! {
            <div class="flex flex-col gap-4">
                // Colors Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Colors"
                    </h5>

                    <div class="flex flex-col gap-2">
                        <label class="text-sm font-medium text-base-content">"Rise Color"</label>
                        <div class="flex gap-2">
                            <input
                                type="color"
                                class="w-12 h-8 rounded cursor-pointer"
                                value=Signal::derive(move || options.get().rise_color.clone())
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    let mut opts = options.get();
                                    opts.rise_color = value;
                                    on_change.run(opts);
                                }
                            />
                            <span class="text-xs text-base-content/60 py-1">
                                {move || options.get().rise_color.clone()}
                            </span>
                        </div>
                    </div>

                    <div class="flex flex-col gap-2">
                        <label class="text-sm font-medium text-base-content">"Fall Color"</label>
                        <div class="flex gap-2">
                            <input
                                type="color"
                                class="w-12 h-8 rounded cursor-pointer"
                                value=Signal::derive(move || options.get().fall_color.clone())
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    let mut opts = options.get();
                                    opts.fall_color = value;
                                    on_change.run(opts);
                                }
                            />
                            <span class="text-xs text-base-content/60 py-1">
                                {move || options.get().fall_color.clone()}
                            </span>
                        </div>
                    </div>

                    <Toggle
                        label="Custom Border Colors"
                        description="Use separate colors for borders"
                        checked=Signal::derive(move || options.get().custom_border_colors)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.custom_border_colors = val;
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
                        label="Candle Width"
                        value=Signal::derive(move || options.get().candle_width)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.candle_width = val;
                            on_change.run(opts);
                        })
                        min=1
                        max=20
                        unit="px"
                    />

                    <Toggle
                        label="Show Labels"
                        description="Display value labels on candles"
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
