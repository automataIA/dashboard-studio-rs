# Leptos 0.8 CSR Project - Development Guide

**Template Version**: 1.0.1
**Last Updated**: 2025-12-25
**Stack**: Leptos 0.8 (CSR) + Trunk + TailwindCSS 3.4.18 + DaisyUI 4.12.24 + Iconify Lucide

> ⚠️ **IMPORTANT**: This project uses **DaisyUI 4.12.24** (compatible with Tailwind CSS 3.4.18).
> DaisyUI 5.x requires Tailwind CSS 4 and is **NOT compatible** with this setup.

> **Note**: This is a general template for Leptos 0.8 CSR projects. Customize the "Project-Specific Context" section at the end for your specific application.

---

## Overview

This is a **Leptos 0.8 Client-Side Rendered (CSR)** web application built with modern Rust web technologies. This document serves as the architectural foundation and development guide for the project.

### Technology Stack

#### Core Framework
- **Leptos 0.8**: Reactive web framework for Rust
  - Mode: CSR (Client-Side Rendering)
  - Compilation target: `wasm32-unknown-unknown`
  - Requires: Rust nightly toolchain

#### Build System
- **Trunk**: WASM web application bundler
  - Handles WASM compilation
  - Manages assets and index.html
  - Development server with hot reload

#### Styling
- **TailwindCSS 3.4.18**: Utility-first CSS framework
  - JIT (Just-In-Time) compiler
  - Scans `*.rs` files for class names
  - Mobile-first responsive design

- **DaisyUI 4.12.24**: Component library for Tailwind
  - Semantic component classes
  - Theme system (light/dark)
  - WCAG AA accessible
  - ⚠️ **CRITICAL**: Use version 4.x, NOT 5.x (DaisyUI 5 requires Tailwind CSS 4)

- **Iconify Lucide**: Icon system
  - Format: `icon-[lucide--icon-name]`
  - Dynamic icon loading via `@iconify/tailwind`
  - Vector icons with CSS sizing/coloring

#### Development Dependencies
- **Node.js/npm**: For CSS tooling (Tailwind, DaisyUI)
- **cargo-leptos** (optional): Additional build utilities
- **wasm-bindgen**: JavaScript interop

---

## Project Structure

### Recommended Directory Layout

```
project-root/
├── .claude/                          # Claude Code tooling (optional)
│   ├── skills/                      # Custom skills for component generation
│   ├── agents/                      # Sub-agents for specialized tasks
│   └── hooks/                       # Git hooks (pre-commit, pre-push)
│
├── src/
│   ├── lib.rs                       # App entry point, router setup
│   │
│   ├── pages/                       # Top-level pages/routes
│   │   ├── home.rs
│   │   ├── about.rs
│   │   └── mod.rs
│   │
│   ├── ui/                          # Dumb/Presentational components
│   │   ├── atoms/                  # Single-purpose components
│   │   │   ├── button.rs           # Button, Badge, Input, Icon
│   │   │   └── mod.rs
│   │   ├── molecules/              # Compositions of atoms
│   │   │   ├── card.rs             # Card, FormField, SearchBar
│   │   │   └── mod.rs
│   │   ├── organisms/              # Complex compositions
│   │   │   ├── navbar.rs           # Navbar, Footer, Sidebar
│   │   │   └── mod.rs
│   │   └── mod.rs
│   │
│   ├── features/                    # Feature modules (domain-based)
│   │   ├── {feature_name}/
│   │   │   ├── components/         # Smart components for this feature
│   │   │   │   ├── feature_form.rs
│   │   │   │   └── mod.rs
│   │   │   ├── context.rs          # Feature-specific state (Context API)
│   │   │   ├── api.rs              # Backend integration
│   │   │   └── mod.rs
│   │   └── mod.rs
│   │
│   ├── context/                     # Global contexts (optional)
│   │   ├── theme.rs                # Theme context
│   │   ├── auth.rs                 # Authentication context
│   │   └── mod.rs
│   │
│   └── utils/                       # Shared utilities (optional)
│       ├── format.rs
│       └── mod.rs
│
├── public/                          # Static assets
│   └── assets/
│
├── index.html                       # HTML template (used by Trunk)
├── Cargo.toml                       # Rust dependencies
├── package.json                     # Node.js dependencies (Tailwind, DaisyUI)
├── tailwind.config.js               # Tailwind configuration
├── rust-toolchain.toml              # Rust nightly version
└── Trunk.toml                       # Trunk configuration
```

---

## Architecture Principles

### 1. Bottom-Up Design

Build from simple to complex:

```
Atoms (ui/atoms/)
  ↓
Molecules (ui/molecules/)
  ↓
Organisms (ui/organisms/)
  ↓
Features (features/)
  ↓
Pages (pages/)
```

**Example**:
- **Atom**: `Button` component
- **Molecule**: `Card` component (uses Button)
- **Organism**: `Navbar` component (uses Card, Button)
- **Feature**: `Auth` feature (uses Navbar, custom forms)
- **Page**: `HomePage` (composes features and organisms)

**Rule**: Never start with page-level components. Always identify and build the atomic pieces first.

### 2. DRY (Don't Repeat Yourself)

- Extract repeated patterns into reusable components
- Use variant enums for type-safe styling
- Create shared utilities for common operations
- Avoid copy-pasting code blocks
- If you write the same code pattern 3+ times, extract it

**Bad**:
```rust
// In component A
<button class="btn btn-primary">

// In component B
<button class="btn btn-primary">

// Repeated 10+ times...
```

**Good**:
```rust
// Create Button component in ui/atoms/
<Button variant=ButtonVariant::Primary>
```

### 3. KISS (Keep It Simple, Stupid)

- **view! macro**: Keep under 40 lines per component
- **Single responsibility**: One component does one thing well
- **No premature abstraction**: Don't create abstractions until you have 3+ uses
- **Readable code**: Favor clarity over cleverness

**Example**:
```rust
// GOOD: Simple, focused component
#[component]
pub fn Badge(
    variant: BadgeVariant,
    children: Children,
) -> impl IntoView {
    view! {
        <span class=move || format!("badge {}", variant.to_class())>
            {children()}
        </span>
    }
}

// BAD: Over-engineered
#[component]
pub fn Badge<F, G>(
    variant: BadgeVariant,
    render_fn: F,
    transform: G,
    // ... 15 more props
) where
    F: Fn() -> View,
    G: Fn(String) -> String,
{
    // Complex logic...
}
```

### 4. Feature-Based Organization

Organize by **domain/feature**, not by technical type:

**Good** (Feature-based):
```
features/
├── authentication/
│   ├── components/
│   ├── context.rs
│   └── api.rs
└── user_profile/
    ├── components/
    ├── context.rs
    └── api.rs
```

**Bad** (Technical grouping):
```
src/
├── components/        # All components mixed
├── contexts/          # All contexts mixed
└── api/               # All API calls mixed
```

### 5. Smart vs Dumb Components

#### Dumb Components (Presentational)
**Location**: `src/ui/`

**Characteristics**:
- Accept data via **props**
- Emit events via **callbacks**
- No API calls
- No Context mutations
- Pure presentation
- Reusable across features

**Example**:
```rust
// src/ui/atoms/button.rs
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=move || format!("btn {}", variant.to_class())
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.call(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
```

#### Smart Components (Container)
**Location**: `src/features/{feature}/components/`

**Characteristics**:
- Manage **state** (signals, context)
- Make **API calls**
- Contain **business logic**
- Use Context for shared state
- Compose dumb components
- Feature-specific behavior

**Example**:
```rust
// src/features/todos/components/todo_list.rs
use crate::ui::atoms::Button;
use crate::features::todos::{TodoContext, api};

#[component]
pub fn TodoList() -> impl IntoView {
    // State management
    let todo_context = TodoContext::use_context();
    let (loading, set_loading) = signal(false);

    // API interaction
    let load_todos = move || {
        spawn_local(async move {
            set_loading.set(true);
            match api::fetch_todos().await {
                Ok(todos) => todo_context.set_todos(todos),
                Err(e) => log::error!("Failed to load todos: {}", e),
            }
            set_loading.set(false);
        });
    };

    // Compose dumb components
    view! {
        <div>
            <Button on_click=move |_| load_todos()>
                "Reload"
            </Button>
            // ... render todos using dumb components
        </div>
    }
}
```

---

## Component Patterns

### Pattern 1: Atom with Variants

Use enums for type-safe variants:

```rust
use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Neutral,
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
}

impl BadgeVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Neutral => "",
            Self::Primary => "badge-primary",
            Self::Secondary => "badge-secondary",
            Self::Success => "badge-success",
            Self::Warning => "badge-warning",
            Self::Error => "badge-error",
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional)] variant: BadgeVariant,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        format!(
            "badge {} {}",
            variant.to_class(),
            class.clone().unwrap_or_default()
        )
    };

    view! {
        <span class=classes>
            {children()}
        </span>
    }
}
```

### Pattern 2: Molecule Composition

Compose atoms into molecules:

```rust
use crate::ui::atoms::{Badge, Button};

#[component]
pub fn Card(
    #[prop(into)] title: String,
    #[prop(optional)] badge_text: Option<String>,
    #[prop(optional)] on_action: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
                <div class="flex justify-between items-start">
                    <h2 class="card-title">{title}</h2>
                    {badge_text.map(|text| view! {
                        <Badge variant=BadgeVariant::Primary>{text}</Badge>
                    })}
                </div>

                <div class="mt-4">
                    {children()}
                </div>

                {on_action.map(|cb| view! {
                    <div class="card-actions justify-end">
                        <Button on_click=move |_| cb.call(())>
                            "Action"
                        </Button>
                    </div>
                })}
            </div>
        </div>
    }
}
```

### Pattern 3: Smart Component with Context

Use Context for shared feature state:

```rust
use leptos::prelude::*;

// Context definition
#[derive(Clone, Copy)]
pub struct AuthContext {
    user: ReadSignal<Option<User>>,
    set_user: WriteSignal<Option<User>>,
}

impl AuthContext {
    pub fn new() -> Self {
        let (user, set_user) = signal(None);
        Self { user, set_user }
    }

    pub fn provide() -> Self {
        let context = Self::new();
        provide_context(context);
        context
    }

    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    pub fn login(&self, user: User) {
        self.set_user.set(Some(user));
    }

    pub fn logout(&self) {
        self.set_user.set(None);
    }

    pub fn user(&self) -> Option<User> {
        self.user.get()
    }
}

// Smart component using context
#[component]
pub fn LoginForm() -> impl IntoView {
    let auth = AuthContext::use_context();
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            match api::login(&email.get(), &password.get()).await {
                Ok(user) => auth.login(user),
                Err(e) => log::error!("Login failed: {}", e),
            }
        });
    };

    view! {
        <form on:submit=handle_submit>
            // Form fields using dumb components
        </form>
    }
}
```

---

## State Management

### Local State (Signals)

Use for component-only state:

```rust
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Increment"
            </button>
        </div>
    }
}
```

### Derived State (Memo)

Use for computed values:

```rust
#[component]
pub fn TodoStats() -> impl IntoView {
    let (todos, set_todos) = signal(vec![]);

    // Derived state - only recomputes when todos change
    let completed_count = Memo::new(move |_| {
        todos.get().iter().filter(|t| t.completed).count()
    });

    let total_count = Memo::new(move |_| {
        todos.get().len()
    });

    view! {
        <div>
            <p>"Total: " {total_count}</p>
            <p>"Completed: " {completed_count}</p>
        </div>
    }
}
```

### Shared State (Context API)

Use for feature-wide or global state:

```rust
// Define context
#[derive(Clone, Copy)]
pub struct TodoContext {
    todos: ReadSignal<Vec<Todo>>,
    set_todos: WriteSignal<Vec<Todo>>,
}

impl TodoContext {
    pub fn new() -> Self {
        let (todos, set_todos) = signal(vec![]);
        Self { todos, set_todos }
    }

    pub fn provide() -> Self {
        let context = Self::new();
        provide_context(context);
        context
    }

    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    pub fn add_todo(&self, todo: Todo) {
        self.set_todos.update(|todos| todos.push(todo));
    }
}

// Use in components
#[component]
pub fn SomeComponent() -> impl IntoView {
    let todo_context = TodoContext::use_context();
    // Use context...
}
```

### Async State (Resources)

Use for data fetching:

```rust
#[component]
pub fn UserProfile(user_id: Signal<u32>) -> impl IntoView {
    let user_resource = Resource::new(
        move || user_id.get(),
        |id| async move {
            api::fetch_user(id).await
        },
    );

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || {
                user_resource.get().map(|result| {
                    match result {
                        Ok(user) => view! { <p>{user.name}</p> },
                        Err(e) => view! { <p>"Error: " {e}</p> },
                    }
                })
            }}
        </Suspense>
    }
}
```

---

## Styling with TailwindCSS + DaisyUI

### Setup Requirements

**⚠️ CRITICAL VERSION NOTE**:
- **DaisyUI 4.12.24** is required for Tailwind CSS 3.4.x compatibility
- **DaisyUI 5.x requires Tailwind CSS 4** (not compatible with 3.4.18)
- Always use DaisyUI 4.x with Tailwind 3.4.x

**package.json**:
```json
{
  "devDependencies": {
    "tailwindcss": "^3.4.18",
    "daisyui": "^4.12.24"  // IMPORTANT: Use 4.x, NOT 5.x
  },
  "dependencies": {
    "@iconify-json/lucide": "^1.2.16",
    "@iconify/tailwind": "^1.1.3"
  }
}
```

**Trunk.toml** (CSS Pre-build Hook):
```toml
[[hooks]]
stage = "pre_build"
command = "npm"
command_arguments = ["run", "build:css"]

[build]
target = "index.html"
# ... rest of config
```

**Why the hook?** Trunk needs to compile Tailwind CSS before building WASM. The pre_build hook ensures `npm run build:css` runs automatically before each build, generating the compiled CSS with all DaisyUI classes.

**tailwind.config.js**:
```javascript
const { addDynamicIconSelectors } = require('@iconify/tailwind');

module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",  // Scan Rust files for Tailwind classes
  ],
  plugins: [
    require('daisyui'),
    addDynamicIconSelectors(),  // Iconify plugin for Lucide icons
  ],
  daisyui: {
    themes: ["light", "dark", "business"],  // Choose your themes
    darkTheme: "dark",  // Default dark theme
  },
}
```

### DaisyUI Component Classes

Use semantic DaisyUI classes:

```rust
// Button
view! {
    <button class="btn btn-primary">"Primary"</button>
    <button class="btn btn-secondary btn-outline">"Outline"</button>
    <button class="btn btn-sm">"Small"</button>
}

// Card
view! {
    <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
            <h2 class="card-title">"Card Title"</h2>
            <p>"Card content"</p>
        </div>
    </div>
}

// Alert
view! {
    <div class="alert alert-success">
        <span>"Success message"</span>
    </div>
}

// Badge
view! {
    <span class="badge badge-primary">"Badge"</span>
}

// Input
view! {
    <input type="text" class="input input-bordered w-full" />
}
```

### Iconify Lucide Icons

Format: `icon-[lucide--icon-name]`

```rust
view! {
    // Basic icon
    <span class="icon-[lucide--home] w-6 h-6"></span>

    // Icon with button
    <button class="btn btn-primary">
        <span class="icon-[lucide--download] w-4 h-4 mr-2"></span>
        "Download"
    </button>

    // Colored icons
    <span class="icon-[lucide--check] w-5 h-5 text-success"></span>
    <span class="icon-[lucide--x] w-5 h-5 text-error"></span>

    // Animated icon
    <span class="icon-[lucide--loader-2] w-5 h-5 animate-spin"></span>
}
```

**Common Icons**:
- UI: `menu`, `x`, `chevron-down`, `chevron-up`, `search`
- Actions: `plus`, `minus`, `edit`, `trash-2`, `save`, `copy`
- Status: `check`, `x`, `alert-circle`, `info`, `help-circle`
- Files: `file`, `folder`, `download`, `upload`
- User: `user`, `users`, `log-in`, `log-out`
- Settings: `settings`, `sliders`, `filter`

### Responsive Design (Mobile-First)

```rust
view! {
    <div class="
        // Mobile (default, <640px)
        px-4 py-2
        text-sm
        grid-cols-1

        // Tablet (sm: 640px+)
        sm:px-6 sm:py-4
        sm:text-base
        sm:grid-cols-2

        // Desktop (lg: 1024px+)
        lg:px-12
        lg:text-lg
        lg:grid-cols-3

        // Large desktop (xl: 1280px+)
        xl:grid-cols-4
    ">
        "Responsive content"
    </div>
}

// Hide/show by screen size
view! {
    // Visible on mobile only
    <div class="lg:hidden">
        "Mobile menu"
    </div>

    // Visible on desktop only
    <div class="hidden lg:block">
        "Desktop menu"
    </div>
}
```

### Dark Mode

DaisyUI handles dark mode via `data-theme` attribute:

```rust
// Set theme on HTML element
view! {
    <Html attr:data-theme="dark" />
}

// Components automatically adapt
view! {
    <div class="bg-base-100 text-base-content">
        "Adapts to theme"
    </div>
}

// Custom dark mode styles
view! {
    <div class="bg-white dark:bg-gray-800">
        "Custom dark mode"
    </div>
}
```

---

## Coding Standards

### File Naming

- **Rust files**: `snake_case.rs`
- **Components**: Match component name (e.g., `LoginForm` → `login_form.rs`)

### Component Naming

- **PascalCase**: Component names
- **snake_case**: Function names, variables, props

```rust
// Component: PascalCase
#[component]
pub fn UserProfile() -> impl IntoView { }

// Props: snake_case
#[component]
pub fn Button(
    button_variant: ButtonVariant,
    on_click: Callback,
) -> impl IntoView { }
```

### Import Organization

```rust
// 1. Standard library
use std::collections::HashMap;

// 2. External crates
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// 3. Internal modules
use crate::ui::atoms::Button;
use crate::features::auth::AuthContext;
```

### Props Best Practices

```rust
#[component]
pub fn MyComponent(
    // 1. Required props first
    children: Children,
    title: String,

    // 2. Optional styling
    #[prop(optional)] variant: Variant,
    #[prop(optional, into)] class: Option<String>,

    // 3. Optional state
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,

    // 4. Optional callbacks
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    // Implementation
}
```

### Error Handling

```rust
// Use Result types for fallible operations
async fn fetch_data() -> Result<Data, ApiError> {
    let response = Request::get("/api/data")
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?;

    if !response.ok() {
        return Err(ApiError::HttpError(response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))
}

// Display errors in UI
view! {
    {move || error.get().map(|err| view! {
        <div class="alert alert-error">
            <span>{err.to_string()}</span>
        </div>
    })}
}
```

---

## Common Pitfalls to Avoid

### 1. ❌ Don't use `unwrap()` in production code

```rust
// BAD
let value = some_option.unwrap();  // Panics if None

// GOOD
let value = some_option.unwrap_or_default();
let value = some_option.expect("Expected value to exist");
match some_option {
    Some(v) => v,
    None => {
        log::error!("Value missing");
        return;
    }
}
```

### 2. ❌ Don't use `println!` or `dbg!` in production

```rust
// BAD
println!("Debug info: {:?}", data);
dbg!(data);

// GOOD
use log::{info, debug, error};
debug!("Debug info: {:?}", data);
```

### 3. ❌ Don't mutate signals in effects that depend on them

```rust
// BAD - Infinite loop!
let (count, set_count) = signal(0);

Effect::new(move |_| {
    let current = count.get();  // Depends on count
    set_count.set(current + 1);  // Modifies count → triggers effect again!
});

// GOOD - Use conditions
Effect::new(move |_| {
    let current = count.get();
    if current < 10 {
        set_count.set(current + 1);
    }
});
```

### 4. ❌ Don't create smart components in `src/ui/`

```rust
// BAD - API call in ui/atoms/
// src/ui/atoms/user_card.rs
#[component]
pub fn UserCard() -> impl IntoView {
    let user = Resource::new(|| (), |_| fetch_user());  // Smart logic!
    // ...
}

// GOOD - Move to features/
// src/features/users/components/user_card.rs
#[component]
pub fn UserCard() -> impl IntoView {
    let user = Resource::new(|| (), |_| fetch_user());
    // ...
}
```

### 5. ❌ Don't forget to declare modules in `mod.rs`

```rust
// If you create src/ui/atoms/button.rs
// You MUST add to src/ui/atoms/mod.rs:

pub mod button;

// Re-export for convenience
pub use button::*;
```

### 6. ❌ Don't use non-WASM compatible crates

```toml
# BAD - Not WASM compatible
[dependencies]
tokio = "1.0"      # Use gloo-timers instead
reqwest = "0.11"   # Use gloo-net instead

# GOOD - WASM compatible
[dependencies]
gloo-net = "0.5"
gloo-timers = "0.3"
gloo-storage = "0.3"
```

### 7. ❌ Don't create large `view!` macros

```rust
// BAD - 100+ line view! macro
#[component]
pub fn HugePage() -> impl IntoView {
    view! {
        <div>
            // 100 lines of markup...
        </div>
    }
}

// GOOD - Split into sub-components
#[component]
pub fn HugePage() -> impl IntoView {
    view! {
        <div>
            <PageHeader />
            <PageContent />
            <PageFooter />
        </div>
    }
}
```

---

## Development Workflow

### 1. Setup

```bash
# Install Rust nightly
rustup toolchain install nightly
rustup default nightly

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Install Node dependencies
npm install
```

### 2. Development

```bash
# Start dev server (http://localhost:8080)
trunk serve

# Build for production
trunk build --release
```

### 3. Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --target wasm32-unknown-unknown -- -D warnings

# Check compilation
cargo check --target wasm32-unknown-unknown
```

### 4. Tailwind CSS

```bash
# Watch and rebuild CSS (if separate from Trunk)
npx tailwindcss -i ./input.css -o ./public/output.css --watch

# Or use package.json scripts:
npm run dev:css
npm run build:css
```

---

## Testing

### Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_button_renders() {
        let _ = create_runtime();

        let button = view! {
            <Button variant=ButtonVariant::Primary>
                "Click me"
            </Button>
        };

        // Assert component structure
    }
}
```

### WASM Tests

```rust
#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_in_browser() {
        // Tests that run in actual browser environment
    }
}
```

---

## Performance Best Practices

### 1. Use Memo for Expensive Computations

```rust
// BAD - Recomputes every render
let filtered = move || {
    items.get()
        .into_iter()
        .filter(|item| item.active)
        .collect::<Vec<_>>()
};

// GOOD - Only recomputes when items change
let filtered = Memo::new(move |_| {
    items.get()
        .into_iter()
        .filter(|item| item.active)
        .collect::<Vec<_>>()
});
```

### 2. Avoid Unnecessary Clones in Closures

```rust
// BAD - Clones on every render
view! {
    <button on:click=move |_| {
        let data = expensive_data.clone();
        process(data);
    }>
        "Click"
    </button>
}

// GOOD - Use signals
let (data, set_data) = signal(expensive_data);

view! {
    <button on:click=move |_| {
        process(data.get());
    }>
        "Click"
    </button>
}
```

### 3. Batch Signal Updates

```rust
// BAD - Three separate updates
set_name.set(new_name);
set_email.set(new_email);
set_age.set(new_age);

// GOOD - Single struct update
#[derive(Clone)]
struct UserData {
    name: String,
    email: String,
    age: u32,
}

let (user, set_user) = signal(UserData::default());

set_user.set(UserData {
    name: new_name,
    email: new_email,
    age: new_age,
});
```

---

## Accessibility (WCAG AA)

### 1. Semantic HTML

```rust
// GOOD
view! {
    <nav>
        <ul>
            <li><a href="/">"Home"</a></li>
        </ul>
    </nav>

    <button type="button">"Click"</button>

    <main>
        <article>
            <h1>"Title"</h1>
        </article>
    </main>
}

// BAD
view! {
    <div on:click=|_| {}>  // Not accessible
        "Click me"
    </div>
}
```

### 2. ARIA Attributes

```rust
view! {
    <button
        aria-label="Close dialog"
        aria-pressed=is_pressed
    >
        <span class="icon-[lucide--x]"></span>
    </button>

    <div role="alert" class="alert alert-error">
        "Error message"
    </div>

    <input
        type="text"
        aria-required="true"
        aria-invalid=has_error
        aria-describedby="error-msg"
    />
}
```

### 3. Focus States

```rust
view! {
    <button class="
        btn btn-primary
        focus:outline-none
        focus:ring-2
        focus:ring-primary
        focus:ring-offset-2
    ">
        "Accessible Button"
    </button>
}
```

### 4. Color Contrast

Use DaisyUI semantic colors (automatically WCAG AA compliant):

```rust
view! {
    <div class="bg-primary text-primary-content">
        "High contrast text"
    </div>
}
```

---

## Troubleshooting

### Issue: WASM compilation fails

```bash
# Check Rust version
rustup show

# Ensure nightly and WASM target
rustup default nightly
rustup target add wasm32-unknown-unknown

# Clear cache
cargo clean
trunk clean
```

### Issue: Tailwind classes not applying

```bash
# Rebuild Tailwind CSS
npx tailwindcss -i ./input.css -o ./public/output.css

# Check tailwind.config.js content paths include *.rs files
content: ["./index.html", "./src/**/*.rs"]

# Restart Trunk dev server
trunk serve
```

### Issue: Icons not showing

```bash
# Ensure @iconify packages installed
npm install @iconify-json/lucide @iconify/tailwind

# Check tailwind.config.js includes plugin
plugins: [
    addDynamicIconSelectors(),
]

# Verify icon format: icon-[lucide--icon-name]
```

### Issue: Module not found

```rust
// Check mod.rs declarations
// If you have src/ui/atoms/button.rs
// Ensure src/ui/atoms/mod.rs has:
pub mod button;
pub use button::*;

// And src/ui/mod.rs has:
pub mod atoms;
pub use atoms::*;

// And src/lib.rs has:
pub mod ui;
```

---

## Additional Resources

### Official Documentation
- [Leptos Book](https://book.leptos.dev)
- [Leptos GitHub](https://github.com/leptos-rs/leptos)
- [TailwindCSS Docs](https://tailwindcss.com/docs)
- [DaisyUI Components](https://daisyui.com/components/)
- [Iconify Icon Sets](https://icon-sets.iconify.design/lucide/)

### Community
- [Leptos Discord](https://discord.gg/leptos)
- [Awesome Leptos](https://github.com/leptos-rs/awesome-leptos)

### Tools
- **leptosfmt**: Code formatter for Leptos
- **trunk**: WASM bundler
- **cargo-leptos**: Build tool with SSR support

---

## Project-Specific Context

> **Customize this section for your specific project**

### This Project Is
<!-- Describe your application's purpose -->
Example: A task management application for teams

### Key Features
<!-- List main features -->
Example:
- Real-time collaboration
- Task assignment and tracking
- Team communication
- File attachments

### Design Goals
<!-- List design principles -->
Example:
- **Simplicity**: Intuitive interface for non-technical users
- **Performance**: Sub-second response times
- **Accessibility**: WCAG 2.1 AA compliance
- **Mobile-first**: Optimized for smartphones

### Color Palette
<!-- Define your color scheme -->
Example:
- **Primary**: #3B82F6 (Blue)
- **Secondary**: #10B981 (Green)
- **Accent**: #F59E0B (Amber)
- **Neutral**: #6B7280 (Gray)
- **Background**: #F9FAFB (Light), #111827 (Dark)

### Custom Tailwind Configuration
<!-- Document any custom Tailwind settings -->
Example:
```javascript
theme: {
  extend: {
    colors: {
      brand: {
        50: '#eff6ff',
        // ... custom color scale
      }
    }
  }
}
```

### API Integration
<!-- Document backend APIs -->
Example:
- **Base URL**: `https://api.example.com/v1`
- **Authentication**: JWT tokens
- **Rate Limiting**: 100 requests/minute

### Deployment
<!-- Document deployment process -->
Example:
- **Hosting**: Netlify / Vercel / CloudFlare Pages
- **CI/CD**: GitHub Actions
- **Environment Variables**: See `.env.example`

---

**Last Updated**: 2024-12-24
**Leptos Version**: 0.8
**Rust Version**: nightly (see rust-toolchain.toml)
