use crate::ui::atoms::{Icon, IconName};
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Preview type for template thumbnails
#[derive(Default, Clone, Copy, PartialEq)]
pub enum PreviewType {
    #[default]
    Bar,
    Pie,
    Line,
}

impl PreviewType {
    /// Returns SVG preview for this template type
    pub fn preview_svg(&self) -> &'static str {
        match self {
            Self::Bar => {
                r#"
                <div class="w-full h-full p-2 flex flex-col gap-1.5">
                    <div class="h-1.5 w-2/3 bg-base-300 rounded-full"></div>
                    <div class="flex-1 bg-base-100 rounded border border-base-300 flex items-end px-1 pb-1 gap-0.5">
                        <div class="w-1/3 h-1/2 bg-blue-400 rounded-t-[2px]"></div>
                        <div class="w-1/3 h-3/4 bg-indigo-500 rounded-t-[2px]"></div>
                        <div class="w-1/3 h-2/3 bg-purple-500 rounded-t-[2px]"></div>
                    </div>
                </div>
            "#
            }
            Self::Pie => {
                r#"
                <div class="w-full h-full p-2 grid grid-cols-2 gap-1.5">
                    <div class="bg-base-100 rounded-full border border-base-300 flex items-center justify-center">
                        <div class="size-4 rounded-full border-2 border-indigo-400 border-t-transparent rotate-45"></div>
                    </div>
                    <div class="bg-base-100 rounded border border-base-300"></div>
                </div>
            "#
            }
            Self::Line => {
                r#"
                <div class="w-full h-full p-2 flex flex-col gap-1.5">
                    <div class="h-1.5 w-2/3 bg-base-300 rounded-full"></div>
                    <div class="flex-1 bg-base-100 rounded border border-base-300 flex items-center justify-center">
                        <svg class="w-full h-full p-2" viewBox="0 0 50 30">
                            <path d="M5 25 Q 15 5, 25 15 T 45 10" fill="none" stroke="currentColor" stroke-width="2" class="text-blue-500"/>
                        </svg>
                    </div>
                </div>
            "#
            }
        }
    }
}

/// Template card component with hover overlay
///
/// Displays a template thumbnail with a hover overlay for selection.
/// Used in the sidebar to show available dashboard templates.
///
/// # Example
/// ```rust
/// let on_select = |template_id: String| {
///     // Handle template selection with the template ID
/// };
///
/// view! {
///     <TemplateCard
///         template_id="tpl_revenue_growth".into()
///         title="Revenue Growth".into()
///         preview_type=PreviewType::Bar
///         on_click=Some(on_select)
///     />
/// }
/// ```
#[component]
pub fn TemplateCard(
    /// Template ID
    #[prop(into)]
    template_id: String,
    /// Template title
    #[prop(into)]
    title: String,
    /// Type of preview to show
    #[prop(optional)]
    preview_type: PreviewType,
    /// Optional click handler
    #[prop(optional)]
    on_click: Option<Callback<String>>,
) -> impl IntoView {
    let template_id_clone = template_id.clone();
    let click_handler = move |ev: MouseEvent| {
        if let Some(cb) = &on_click {
            ev.prevent_default();
            cb.run(template_id_clone.clone());
        }
    };

    view! {
        <div class="group cursor-pointer" on:click=click_handler>
            <div class="aspect-[4/3] bg-base-200 rounded-lg border border-base-300 \
            group-hover:border-primary \
            group-hover:ring-1 group-hover:ring-primary/20 \
            transition-all overflow-hidden relative">

                // Hover overlay
                <div class="absolute inset-0 flex flex-col items-center justify-center opacity-0 \
                group-hover:opacity-100 bg-primary/10 backdrop-blur-[1px] \
                transition-all z-10">
                    <Icon
                        name=IconName::Add
                        class="w-4 h-4 text-primary drop-shadow-sm bg-base-100 \
                        rounded-full p-1"
                    />
                    <span class="text-[10px] font-bold text-primary mt-1">Use Template</span>
                </div>

                // Preview SVG
                <div
                    class="w-full h-full p-2 opacity-70 group-hover:opacity-100 transition-opacity"
                    inner_html=preview_type.preview_svg()
                ></div>
            </div>

            <p class="mt-1.5 text-xs text-base-content/80 font-medium truncate \
            group-hover:text-primary transition-colors">{title}</p>
        </div>
    }
}
