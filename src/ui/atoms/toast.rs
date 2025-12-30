use leptos::prelude::*;

/// Toast notification component
///
/// Displays a toast notification with icon, title, message, and close button.
///
/// # Example
/// ```rust
/// view! {
///     <Toast
///         id=toast.id
///         title=toast.title
///         message=toast.message
///         variant=toast.variant
///         on_close=close_callback
///     />
/// }
/// ```
#[component]
pub fn Toast(
    /// Unique identifier for this toast
    #[prop(into)]
    _id: String,
    /// Toast title (headline)
    #[prop(into)]
    title: String,
    /// Toast message (body text)
    #[prop(into)]
    message: String,
    /// Toast variant (success, error, warning, info)
    variant: crate::context::toast::ToastVariant,
    /// Callback when close button is clicked
    on_close: Callback<()>,
) -> impl IntoView {
    let on_close_click = move |_| {
        on_close.run(());
    };

    view! {
        <div
            class=format!(
                "alert shadow-lg rounded-lg animate-slide-in flex items-start gap-3 {}",
                variant.alert_class()
            )
            style="min-width: 300px; max-width: 500px;"
        >
            // Icon
            <span class=format!("text-xl shrink-0 {}", variant.icon_name())></span>

            // Content
            <div class="flex-1 min-w-0">
                <h3 class="font-bold text-sm">{title}</h3>
                <div class="text-xs opacity-90">{message}</div>
            </div>

            // Close button
            <button
                class="btn btn-ghost btn-xs btn-circle shrink-0"
                on:click=on_close_click
                aria-label="Close notification"
            >
                <span class="icon-[lucide--x] w-4 h-4"></span>
            </button>
        </div>
    }
}

/// Toast container component
///
/// Renders all active toasts in a fixed position container.
/// Should be placed once at the app root level.
///
/// # Example
/// ```rust
/// // In app root
/// view! {
///     <div class="app">
///         // ... other content
///         <ToastContainer />
///     </div>
/// }
/// ```
#[component]
pub fn ToastContainer() -> impl IntoView {
    let toast_context = crate::context::toast::ToastContext::use_context();

    let toasts = Signal::derive(move || toast_context.get_toasts());

    view! {
        <div class="toast toast-bottom toast-end z-50">
            {move || {
                toasts
                    .get()
                    .into_iter()
                    .map(|toast| {
                        let toast_id = toast.id.clone();
                        let toast_context_clone = toast_context;

                        let on_close = Callback::new(move |_| {
                            toast_context_clone.remove(toast_id.clone());
                        });

                        view! {
                            <Toast
                                _id=toast.id.clone()
                                title=toast.title.clone()
                                message=toast.message.clone()
                                variant=toast.variant
                                on_close=on_close
                            />
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </div>
    }
}
