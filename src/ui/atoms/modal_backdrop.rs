use leptos::prelude::*;
use leptos::ev::MouseEvent;

/// Semi-transparent backdrop overlay for modals
///
/// # Props
/// - `on_click`: Optional callback when backdrop is clicked (typically to close modal)
#[component]
pub fn ModalBackdrop(
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    view! {
        <div
            class="fixed inset-0 bg-black/50 backdrop-blur-sm z-40 transition-opacity duration-200"
            on:click=move |ev| {
                if let Some(cb) = &on_click {
                    cb.run(ev);
                }
            }
        ></div>
    }
}
