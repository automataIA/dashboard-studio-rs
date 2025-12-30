use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Toast notification data
#[derive(Clone, Debug, PartialEq)]
pub struct ToastData {
    pub id: String,
    pub title: String,
    pub message: String,
    pub variant: ToastVariant,
}

/// Toast notification variant
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastVariant {
    /// Get DaisyUI alert class for this variant
    pub fn alert_class(&self) -> &'static str {
        match self {
            Self::Success => "alert-success",
            Self::Error => "alert-error",
            Self::Warning => "alert-warning",
            Self::Info => "alert-info",
        }
    }

    /// Get icon name for this variant
    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Success => "icon-[lucide--check-circle]",
            Self::Error => "icon-[lucide--alert-circle]",
            Self::Warning => "icon-[lucide--alert-triangle]",
            Self::Info => "icon-[lucide--info]",
        }
    }
}

/// Toast context for managing notifications
///
/// # Example
/// ```rust
/// // In app root
/// let toast = ToastContext::provide();
///
/// // In any component
/// let toast = ToastContext::use_context();
/// toast.show_success("Success!", "Operation completed");
/// ```
#[derive(Clone, Copy)]
pub struct ToastContext {
    toasts: ReadSignal<Vec<ToastData>>,
    set_toasts: WriteSignal<Vec<ToastData>>,
}

impl ToastContext {
    /// Create new toast context
    fn new() -> Self {
        let (toasts, set_toasts) = signal(Vec::new());
        Self { toasts, set_toasts }
    }

    /// Provide context at app root
    pub fn provide() -> Self {
        let context = Self::new();
        provide_context(context);
        context
    }

    /// Get context in child components
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    /// Show success toast
    pub fn show_success(&self, title: &str, message: &str) {
        self.show(ToastVariant::Success, title, message);
    }

    /// Show error toast
    pub fn show_error(&self, title: &str, message: &str) {
        self.show(ToastVariant::Error, title, message);
    }

    /// Show warning toast
    #[allow(dead_code)]
    pub fn show_warning(&self, title: &str, message: &str) {
        self.show(ToastVariant::Warning, title, message);
    }

    /// Show info toast
    #[allow(dead_code)]
    pub fn show_info(&self, title: &str, message: &str) {
        self.show(ToastVariant::Info, title, message);
    }

    /// Show toast with variant
    fn show(&self, variant: ToastVariant, title: &str, message: &str) {
        let id = format!("toast_{}", uuid::Uuid::new_v4());
        let toast = ToastData {
            id: id.clone(),
            title: title.to_string(),
            message: message.to_string(),
            variant,
        };

        // Add to list (max 5 toasts)
        self.set_toasts.update(|toasts| {
            toasts.push(toast);
            if toasts.len() > 5 {
                toasts.remove(0);
            }
        });

        // Auto-dismiss after 5 seconds using web_sys setTimeout
        let set_toasts = self.set_toasts;
        let id_clone = id.clone();
        let timeout_closure = wasm_bindgen::closure::Closure::once(move || {
            set_toasts.update(|toasts| {
                toasts.retain(|t| t.id != id_clone);
            });
        });

        let window = web_sys::window().expect("no global `window` exists");
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_closure.as_ref().unchecked_ref(),
                5000,
            )
            .expect("should register `setTimeout` OK");
        timeout_closure.forget();
    }

    /// Get all active toasts (for rendering)
    pub fn get_toasts(&self) -> Vec<ToastData> {
        self.toasts.get()
    }

    /// Remove toast by ID
    pub fn remove(&self, id: String) {
        self.set_toasts.update(|toasts| {
            toasts.retain(|t| t.id != id);
        });
    }
}
