use leptos::prelude::*;
use crate::ui::organisms::Header;
use crate::ui::atoms::{Button, ButtonVariant, Icon, IconName};

/// Settings page - Manage dashboard preferences, account security, and integrations
#[component]
pub fn Settings() -> impl IntoView {
    // State for settings
    let (language, set_language) = signal("English (US)".to_string());
    let (dark_mode, set_dark_mode) = signal(true);
    let (selected_view, set_selected_view) = signal("grid".to_string());
    let (full_name, set_full_name) = signal("John Doe".to_string());
    let (email, set_email) = signal("john.doe@company.com".to_string());

    // Derived classes for icons
    let grid_icon_class = Memo::new(move |_| {
        if selected_view.get() == "grid" {
            "text-primary".to_string()
        } else {
            "text-base-content/50".to_string()
        }
    });

    let table_icon_class = Memo::new(move |_| {
        if selected_view.get() == "table" {
            "text-primary".to_string()
        } else {
            "text-base-content/50".to_string()
        }
    });

    // Sidebar navigation items
    #[derive(Clone, Copy)]
    struct NavItem {
        id: &'static str,
        icon: IconName,
        label: &'static str,
        href: &'static str,
    }

    const NAV_ITEMS: &[NavItem] = &[
        NavItem { id: "general", icon: IconName::Settings, label: "General", href: "#general" },
        NavItem { id: "theme", icon: IconName::Palette, label: "Theme", href: "#theme" },
        NavItem { id: "account", icon: IconName::Person, label: "Account", href: "#account" },
        NavItem { id: "privacy", icon: IconName::Shield, label: "Privacy", href: "#privacy" },
        NavItem { id: "integrations", icon: IconName::Link, label: "Integrations", href: "#integrations" },
    ];

    let (active_section, set_active_section) = signal("general".to_string());

    // Derived classes for navigation
    let get_nav_icon_class = move |item_id: &'static str| -> String {
        let base = "text-[20px]";
        let color = if active_section.get() == item_id {
            "text-primary"
        } else {
            "text-base-content/50"
        };
        format!("{} {}", base, color)
    };

    let get_nav_text_class = move |item_id: &'static str| -> String {
        let base = "text-sm font-medium";
        let color = if active_section.get() == item_id {
            "text-base-content"
        } else {
            "text-base-content/50"
        };
        format!("{} {}", base, color)
    };

    let get_nav_link_class = move |item_id: &'static str| -> String {
        let base = "flex items-center gap-3 px-3 py-2 rounded-lg transition-colors group";
        let style = if active_section.get() == item_id {
            "bg-primary/20 border border-primary/20"
        } else {
            "hover:bg-base-300"
        };
        format!("{} {}", base, style)
    };

    // Avatar URL
    let avatar_url = "https://ui-avatars.com/api/?name=Data+Viz&background=1C4E80&color=fff".to_string();

    view! {
        <div class="min-h-screen bg-base-100 text-base-content">
            // Header (using same Header component as Dashboard and Projects)
            <Header user_avatar_url=avatar_url />

            <div class="flex flex-col md:flex-row max-w-[1440px] mx-auto">
                // Sidebar
                <aside class="hidden md:flex w-64 flex-col gap-4 border-r border-base-300 bg-base-200 p-4 sticky top-[65px] h-[calc(100vh-65px)] overflow-y-auto">
                    <div class="flex flex-col gap-6">
                        // User info
                        <div class="flex gap-3 items-center px-2">
                            <div class="size-12 rounded-full bg-gradient-to-br from-primary to-indigo-600"></div>
                            <div class="flex flex-col">
                                <h1 class="text-base-content text-base font-medium">John Doe</h1>
                                <p class="text-base-content/70 text-sm">Administrator</p>
                            </div>
                        </div>

                        // Navigation
                        <div class="flex flex-col gap-1">
                            {NAV_ITEMS.iter().map(|item| {
                                view! {
                                    <a
                                        href=item.href
                                        class=move || get_nav_link_class(item.id)
                                        on:click=move |_| set_active_section.set(item.id.to_string())
                                    >
                                        <span class=move || get_nav_icon_class(item.id)>
                                            <Icon name=item.icon />
                                        </span>
                                        <p class=move || get_nav_text_class(item.id)>
                                            {item.label}
                                        </p>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </aside>

                // Main content
                <main class="flex-1 p-6 md:p-10 lg:px-20 overflow-y-auto">
                    <div class="max-w-[800px] mx-auto flex flex-col gap-8">
                        // Page header
                        <div class="flex flex-col gap-2">
                            <h1 class="text-base-content text-4xl font-black tracking-tight">Settings</h1>
                            <p class="text-base-content/70 text-base">
                                "Manage your dashboard preferences, account security, and integrations."
                            </p>
                        </div>

                        // General Settings Section
                        <section class="flex flex-col gap-4" id="general">
                            <div class="border-b border-base-300 pb-2">
                                <h2 class="text-base-content text-xl font-bold">General Settings</h2>
                            </div>
                            <div class="bg-base-200 rounded-xl p-6 border border-base-300 flex flex-col gap-6">
                                // Language selector
                                <div class="flex flex-col gap-2">
                                    <label class="text-base-content text-sm font-medium">Language</label>
                                    <div class="relative">
                                        <select
                                            class="select select-bordered w-full"
                                            on:change=move |ev| {
                                                let value = event_target_value(&ev);
                                                set_language.set(value);
                                            }
                                        >
                                            <option selected=move || language.get() == "English (US)">"English (US)"</option>
                                            <option selected=move || language.get() == "Spanish">"Spanish"</option>
                                            <option selected=move || language.get() == "French">"French"</option>
                                            <option selected=move || language.get() == "German">"German"</option>
                                        </select>
                                        <Icon name=IconName::ExpandMore class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content/50 pointer-events-none" />
                                    </div>
                                </div>

                                // Default Dashboard View
                                <div class="flex flex-col gap-3">
                                    <label class="text-base-content text-sm font-medium">Default Dashboard View</label>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                        // Grid Layout option
                                        <label class="cursor-pointer relative">
                                            <input
                                                type="radio"
                                                name="default_view"
                                                class="radio radio-primary peer sr-only"
                                                checked=move || selected_view.get() == "grid"
                                                on:change=move |_| set_selected_view.set("grid".to_string())
                                            />
                                            <div class="rounded-lg border border-base-300 bg-base-100 p-4 hover:border-base-content/30 peer-checked:border-primary peer-checked:bg-primary/10 transition-all flex items-center gap-3">
                                                <span class=grid_icon_class>
                                                    <Icon name=IconName::GridView />
                                                </span>
                                                <span class="text-base-content text-sm">Grid Layout</span>
                                            </div>
                                        </label>

                                        // Table Layout option
                                        <label class="cursor-pointer relative">
                                            <input
                                                type="radio"
                                                name="default_view"
                                                class="radio radio-primary peer sr-only"
                                                checked=move || selected_view.get() == "table"
                                                on:change=move |_| set_selected_view.set("table".to_string())
                                            />
                                            <div class="rounded-lg border border-base-300 bg-base-100 p-4 hover:border-base-content/30 peer-checked:border-primary peer-checked:bg-primary/10 transition-all flex items-center gap-3">
                                                <span class=table_icon_class>
                                                    <Icon name=IconName::TableChart />
                                                </span>
                                                <span class="text-base-content text-sm">Table Layout</span>
                                            </div>
                                        </label>
                                    </div>
                                </div>
                            </div>
                        </section>

                        // Theme Preferences Section
                        <section class="flex flex-col gap-4" id="theme">
                            <div class="border-b border-base-300 pb-2">
                                <h2 class="text-base-content text-xl font-bold">Theme Preferences</h2>
                            </div>
                            <div class="bg-base-200 rounded-xl p-6 border border-base-300 flex flex-col gap-6">
                                // Dark Mode Toggle
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col">
                                        <span class="text-base-content font-medium text-sm">Dark Mode</span>
                                        <span class="text-base-content/70 text-sm">Use dark theme for low-light environments</span>
                                    </div>
                                    <input
                                        type="checkbox"
                                        class="toggle toggle-primary"
                                        checked=dark_mode
                                        on:change=move |_| set_dark_mode.update(|v| *v = !*v)
                                    />
                                </div>

                                // Accent Color
                                <div class="flex flex-col gap-3">
                                    <label class="text-base-content text-sm font-medium">Accent Color</label>
                                    <div class="flex gap-3">
                                        <button class="w-8 h-8 rounded-full bg-primary ring-2 ring-offset-2 ring-offset-base-200 ring-primary"></button>
                                        <button class="w-8 h-8 rounded-full bg-secondary hover:ring-2 hover:ring-offset-2 hover:ring-offset-base-200 hover:ring-secondary transition-all"></button>
                                        <button class="w-8 h-8 rounded-full bg-accent hover:ring-2 hover:ring-offset-2 hover:ring-offset-base-200 hover:ring-accent transition-all"></button>
                                        <button class="w-8 h-8 rounded-full bg-warning hover:ring-2 hover:ring-offset-2 hover:ring-offset-base-200 hover:ring-warning transition-all"></button>
                                    </div>
                                </div>
                            </div>
                        </section>

                        // Account Settings Section
                        <section class="flex flex-col gap-4" id="account">
                            <div class="border-b border-base-300 pb-2">
                                <h2 class="text-base-content text-xl font-bold">Account Settings</h2>
                            </div>
                            <div class="bg-base-200 rounded-xl p-6 border border-base-300 flex flex-col gap-6">
                                // Profile Picture
                                <div class="flex items-center gap-6">
                                    <div class="relative group cursor-pointer">
                                        <div class="size-20 rounded-full bg-gradient-to-br from-primary to-indigo-600 border-2 border-base-300"></div>
                                        <div class="absolute inset-0 bg-black/50 rounded-full flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                                            <Icon name=IconName::Settings class="text-base-content" />
                                        </div>
                                    </div>
                                    <div class="flex flex-col gap-1">
                                        <h3 class="text-base-content font-medium">Profile Picture</h3>
                                        <p class="text-base-content/70 text-xs">JPG, GIF or PNG. Max size of 800K</p>
                                    </div>
                                </div>

                                // Form fields
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <div class="flex flex-col gap-2">
                                        <label class="text-base-content text-sm font-medium">Full Name</label>
                                        <input
                                            type="text"
                                            class="input input-bordered w-full"
                                            prop:value=full_name
                                            on:input=move |ev| { set_full_name.set(event_target_value(&ev)); }
                                        />
                                    </div>
                                    <div class="flex flex-col gap-2">
                                        <label class="text-base-content text-sm font-medium">Email Address</label>
                                        <input
                                            type="email"
                                            class="input input-bordered w-full"
                                            prop:value=email
                                            on:input=move |ev| { set_email.set(event_target_value(&ev)); }
                                        />
                                    </div>
                                </div>

                                // Change Password button
                                <div class="pt-2">
                                    <Button variant=ButtonVariant::Ghost>
                                        "Change Password"
                                    </Button>
                                </div>
                            </div>
                        </section>

                        // Integrations Section
                        <section class="flex flex-col gap-4" id="integrations">
                            <div class="border-b border-base-300 pb-2">
                                <h2 class="text-base-content text-xl font-bold">Integrations & API</h2>
                            </div>
                            <div class="bg-base-200 rounded-xl border border-base-300 overflow-hidden">
                                // AWS S3 Integration
                                <div class="p-6 border-b border-base-300 flex flex-col md:flex-row md:items-center justify-between gap-4">
                                    <div class="flex items-center gap-4">
                                        <div class="w-12 h-12 rounded-lg bg-white flex items-center justify-center p-2">
                                            <div class="w-full h-full rounded bg-gradient-to-br from-orange-400 to-orange-600 flex items-center justify-center text-base-content font-bold text-xs">
                                                "AWS"
                                            </div>
                                        </div>
                                        <div>
                                            <h3 class="text-base-content font-medium">AWS S3 Storage</h3>
                                            <p class="text-base-content/70 text-sm">Connect your S3 buckets for data import</p>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-3">
                                        <span class="flex h-2 w-2 rounded-full bg-success"></span>
                                        <span class="text-success text-sm font-medium mr-2">Connected</span>
                                        <button class="text-base-content/70 hover:text-base-content p-2">
                                            <Icon name=IconName::Settings />
                                        </button>
                                    </div>
                                </div>

                                // Google Sheets Integration
                                <div class="p-6 border-b border-base-300 flex flex-col md:flex-row md:items-center justify-between gap-4">
                                    <div class="flex items-center gap-4">
                                        <div class="w-12 h-12 rounded-lg bg-white flex items-center justify-center p-2">
                                            <div class="w-full h-full rounded bg-gradient-to-br from-green-400 to-green-600 flex items-center justify-center text-base-content font-bold text-xs">
                                                "GS"
                                            </div>
                                        </div>
                                        <div>
                                            <h3 class="text-base-content font-medium">Google Sheets</h3>
                                            <p class="text-base-content/70 text-sm">Sync data directly from spreadsheets</p>
                                        </div>
                                    </div>
                                    <Button variant=ButtonVariant::Ghost>
                                        "Connect"
                                    </Button>
                                </div>

                                // API Key
                                <div class="p-6 bg-base-300/50">
                                    <label class="text-base-content text-sm font-medium mb-2 block">Your API Key</label>
                                    <div class="flex gap-2">
                                        <div class="relative flex-1">
                                            <input
                                                type="text"
                                                value="sk_live_51J9z...8h2k9"
                                                readonly
                                                class="input input-bordered w-full input-sm text-base-content/70 font-mono"
                                            />
                                            <button class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content/70 hover:text-base-content">
                                                <Icon name=IconName::Copy class="w-[18px] h-[18px]" />
                                            </button>
                                        </div>
                                        <Button variant=ButtonVariant::Primary class="whitespace-nowrap">
                                            "Regenerate"
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        </section>

                        // Sticky action buttons
                        <div class="sticky bottom-6 flex justify-end gap-4 pt-4">
                            <div class="bg-base-200/80 backdrop-blur-md p-4 rounded-xl border border-base-300 shadow-2xl flex gap-4 w-full md:w-auto justify-end">
                                <Button variant=ButtonVariant::Ghost>
                                    "Cancel"
                                </Button>
                                <Button variant=ButtonVariant::Primary>
                                    "Save Changes"
                                </Button>
                            </div>
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}
