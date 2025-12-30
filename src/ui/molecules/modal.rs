use leptos::prelude::*;
use leptos::ev::{KeyboardEvent, MouseEvent};
use wasm_bindgen::JsCast;
use crate::ui::atoms::{ModalBackdrop, Button, ButtonVariant, IconName};

/// Reusable modal dialog component
///
/// A centered modal with backdrop, close button, ESC key support,
/// and click-outside-to-close functionality. Responsive on mobile.
///
/// # Props
/// - `show`: Signal controlling modal visibility
/// - `on_close`: Callback when modal should close
/// - `title`: Modal header title
/// - `children`: Modal content
///
/// # Example
/// ```rust
/// let (show_modal, set_show_modal) = signal(false);
///
/// view! {
///     <button on:click=move |_| set_show_modal.set(true)>
///         "Open Modal"
///     </button>
///
///     <Modal
///         show=show_modal.into()
///         on_close=Callback::new(move |_| set_show_modal.set(false))
///         title="My Modal".into()
///     >
///         <p>"Modal content goes here"</p>
///     </Modal>
/// }
/// ```
#[component]
pub fn Modal(
    /// Signal controlling modal visibility
    show: Signal<bool>,
    /// Callback when modal should close
    on_close: Callback<MouseEvent>,
    /// Modal header title
    #[prop(into)]
    title: String,
    /// Modal content
    children: Children,
) -> impl IntoView {
    // Store title for use in closures
    let title = StoredValue::new(title);

    // Clone callback for backdrop
    let on_close_backdrop = on_close.clone();

    // ESC key handler - needs to capture on_close
    let on_close_esc = on_close.clone();
    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" && show.get() {
            // Convert KeyboardEvent to MouseEvent for callback
            let mouse_ev: MouseEvent = ev.unchecked_into();
            on_close_esc.run(mouse_ev);
        }
    };

    let display_class = move || {
        if show.get() {
            "block"
        } else {
            "hidden"
        }
    };

    view! {
        <div class=display_class>
            // Backdrop
            <ModalBackdrop on_click=on_close_backdrop />

            // Modal container
            <div
                class="fixed inset-0 z-50 flex items-center justify-center p-4"
                on:keydown=on_keydown
                tabindex=-1
            >
                // Modal content - responsive sizing
                <div class="
                    bg-base-100 rounded-xl shadow-2xl
                    w-full max-w-2xl
                    max-h-[80vh] overflow-hidden
                    flex flex-col
                    animate-in fade-in slide-in-from-bottom-4 duration-200
                    sm:max-w-xl
                    lg:max-w-2xl
                    sm:w-[95%]
                    lg:w-[90%]
                ">
                    // Header
                    <div class="flex items-center justify-between p-6 border-b border-base-300">
                        <h2 class="text-xl font-bold text-base-content">{title.get_value()}</h2>
                        <Button
                            variant=ButtonVariant::Ghost
                            icon_left=IconName::Close
                            on_click=on_close
                            title="Close".to_string()
                        />
                    </div>

                    // Body (scrollable)
                    <div class="flex-1 overflow-y-auto p-6">
                        {children()}
                    </div>
                </div>
            </div>
        </div>
    }
}
