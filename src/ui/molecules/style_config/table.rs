//! Table widget style configuration UI

use crate::features::dashboard::config::style::TableStyleOptions;
use crate::ui::atoms::{Slider, Toggle};
use super::StyleConfigUI;
use leptos::prelude::*;

/// Marker type for Table widget style configuration
#[derive(Clone, Copy)]
pub struct TableStyleConfig;

impl StyleConfigUI for TableStyleConfig {
    type Options = TableStyleOptions;

    fn render_controls(
        options: Signal<Self::Options>,
        on_change: Callback<Self::Options>,
    ) -> impl IntoView {
        view! {
            <div class="flex flex-col gap-4">
                // Table Features Section
                <div class="flex flex-col gap-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Table Features"
                    </h5>

                    <Toggle
                        label="Show Pagination"
                        description="Enable pagination for large datasets"
                        checked=Signal::derive(move || options.get().show_pagination)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_pagination = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Enable Sorting"
                        description="Allow column sorting"
                        checked=Signal::derive(move || options.get().show_sorting)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_sorting = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Hover Effects"
                        description="Highlight row on hover"
                        checked=Signal::derive(move || options.get().hover)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.hover = val;
                            on_change.run(opts);
                        })
                    />

                    <Toggle
                        label="Show Borders"
                        description="Show cell borders"
                        checked=Signal::derive(move || options.get().show_borders)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.show_borders = val;
                            on_change.run(opts);
                        })
                    />
                </div>

                // Pagination Options Section
                <div class="flex flex-col gap-3 border-t border-base-content/10 pt-3">
                    <h5 class="text-xs font-semibold text-base-content/70 uppercase tracking-wide">
                        "Pagination"
                    </h5>

                    <Slider
                        label="Rows Per Page"
                        value=Signal::derive(move || options.get().page_size as u8)
                        on_change=Callback::new(move |val| {
                            let mut opts = options.get();
                            opts.page_size = val as u16;
                            on_change.run(opts);
                        })
                        min=5
                        max=100
                        unit="rows"
                    />
                </div>
            </div>
        }
    }
}
