
# Dashboard Studio

> Privacy-first, client-side data visualization powered by Rust + WebAssembly. Upload CSVs, create interactive dashboards with 11 chart types, drag-and-drop layout, auto-save, undo/redo—all running in your browser. Zero backend, 100% GDPR-compliant, <500KB bundle.

[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/leptos-0.8-blue.svg)](https://leptos.dev/)
[![TailwindCSS](https://img.shields.io/badge/tailwindcss-3.4.18-38bdf8.svg)](https://tailwindcss.com/)
[![DaisyUI](https://img.shields.io/badge/daisyui-4.12.24-5a67d8.svg)](https://daisyui.com/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Dashboard Studio** is an open-source, privacy-first data visualization platform built with Rust and WebAssembly. Upload CSV datasets, create professional dashboards with 11 interactive chart types, and visualize data entirely in your browser—no server required, no data ever leaves your device.

---

## Table of Contents

- [Features](#features)
- [Demo](#demo)
- [Technology Stack](#technology-stack)
- [Supported Chart Types](#supported-chart-types)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Development](#development)
  - [Production Build](#production-build)
- [Usage](#usage)
- [Project Architecture](#project-architecture)
- [Development Guide](#development-guide)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Features

### Core Capabilities

- **CSV Data Import** - Upload and parse CSV files with automatic type detection
- **11 Chart Types** - Line, Bar, Pie, Scatter, Area, Radar, Candlestick, Heatmap, Treemap, KPI, Table
- **Interactive Canvas** - 12-column grid layout with drag-and-drop positioning
- **Real-Time Configuration** - Live data mapping and style customization
- **Undo/Redo** - Complete history management with command pattern
- **Template System** - Save and load dashboard configurations
- **Auto-Save** - Automatic persistence to browser localStorage (2-second debounce)
- **Dark Mode** - Theme-aware UI with DaisyUI components
- **Responsive Design** - Mobile-first, works on all screen sizes
- **Zero Backend** - Fully client-side rendered (CSR) application

### Advanced Features

- **Multiple Dataset Management** - Switch between uploaded CSV files
- **Field Type Detection** - Automatic inference (Text, Numeric, Date, Boolean)
- **Data Aggregation** - Sum, Average, Min, Max, Count operations
- **Chart Variants** - 30+ chart style variations (stacked, grouped, smooth, etc.)
- **Layer Management** - Organize widgets in hierarchical layers
- **Activity Timeline** - Track recent dashboard changes
- **Export/Import** - Share dashboard templates as JSON

---

## Demo

<!-- TODO: Add screenshots/GIFs here -->

```
[Screenshot: Dashboard with multiple charts]
[Screenshot: Data configuration panel]
[Screenshot: Style customization]
```

---

## Technology Stack

### Core Framework
- **[Leptos 0.8](https://leptos.dev/)** - Reactive web framework for Rust (CSR mode)
- **[Trunk](https://trunkrs.dev/)** - WASM bundler and development server
- **Rust Nightly** - Required for Leptos compilation

### Styling & UI
- **[TailwindCSS 3.4.18](https://tailwindcss.com/)** - Utility-first CSS framework
- **[DaisyUI 4.12.24](https://daisyui.com/)** - Component library (WCAG AA accessible)
- **[Iconify Lucide](https://lucide.dev/)** - Icon system with 1000+ icons

### Data & Interop
- **serde/serde_json** - Serialization/deserialization
- **wasm-bindgen** - JavaScript interop layer
- **web-sys** - Web API bindings
- **csv** - CSV parsing
- **chrono** - Date/time handling
- **uuid** - Unique identifiers

### Visualization
- **ECharts** - JavaScript charting library (via wasm-bindgen)

---

## Supported Chart Types

Dashboard Studio supports **11 widget types** with **30+ variants**:

### Chart Families

| Type             | Variants                                 | Use Case                    |
| ---------------- | ---------------------------------------- | --------------------------- |
| **Line Chart**   | Basic, Smooth, Step, Stacked, Area       | Time-series data, trends    |
| **Bar Chart**    | Basic, Stacked, Grouped, Race, Waterfall | Comparisons, rankings       |
| **Pie Chart**    | Basic, Doughnut, Rose                    | Proportions, percentages    |
| **Scatter Plot** | Basic, Bubble                            | Correlations, distributions |
| **Area Chart**   | Basic, Stacked                           | Cumulative data, trends     |

### Advanced Charts

| Type            | Use Case                           |
| --------------- | ---------------------------------- |
| **Radar Chart** | Multi-dimensional data comparison  |
| **Candlestick** | OHLC financial data (stock prices) |
| **Heatmap**     | Matrix data, correlations          |
| **Treemap**     | Hierarchical data, proportions     |

### Data Display

| Type             | Use Case                        |
| ---------------- | ------------------------------- |
| **KPI Widget**   | Key performance indicators      |
| **Table Widget** | Structured tabular data display |

---

## Getting Started

### Prerequisites

Ensure you have the following installed:

- **Rust Nightly** (required for Leptos)
- **Node.js & npm** (for TailwindCSS)
- **Trunk** (WASM bundler)

### Installation

#### 1. Install Rust Nightly

```bash
# Install Rust nightly toolchain
rustup toolchain install nightly --allow-downgrade
rustup default nightly

# Add WebAssembly compilation target
rustup target add wasm32-unknown-unknown
```

#### 2. Install Trunk

```bash
cargo install trunk
```

#### 3. Clone the Repository

```bash
git clone https://github.com/yourusername/dashboard-studio-rs.git
cd dashboard-studio-rs
```

#### 4. Install Node Dependencies

```bash
npm install
```

This installs TailwindCSS, DaisyUI, and Iconify plugins.

### Development

Start the development server with hot reload:

```bash
trunk serve
```

The app will open at **http://localhost:3000/** with live reloading enabled.

**Note**:
- TailwindCSS compilation runs automatically via Trunk's `pre_build` hook (configured in `Trunk.toml`)
- Local development uses `public_url = "/"` for simplicity
- GitHub Actions deployment overrides this with `--public-url "/dashboard-studio-rs/"`

### Production Build

#### Local Build (Not Recommended)

Due to outdated `wasm-opt` version (108) in Ubuntu repositories, local release builds may fail with bulk-memory errors. If you need to build locally:

```bash
trunk build --release --public-url "/dashboard-studio-rs/"
```

Output files are in `dist/` folder.

#### GitHub Actions Deployment (Recommended)

The project uses GitHub Actions for automated deployment to GitHub Pages. The workflow:

1. **Triggers** on push to `main` branch
2. **Installs** latest Trunk (with modern wasm-opt)
3. **Builds** optimized WASM bundle with `--release` flag
4. **Deploys** to `docs/` folder for GitHub Pages

**Setup GitHub Pages**:
1. Push your code to GitHub
2. Go to Settings → Pages
3. Set source to "Deploy from a branch"
4. Select `main` branch and `/docs` folder
5. Save

The app will be available at: `https://yourusername.github.io/dashboard-studio-rs/`

**Note**: Production builds use aggressive optimizations (`opt-level='z'`, `lto=true`) which work correctly with modern wasm-opt in CI/CD but may fail with older local installations.

---

## Usage

### Quick Start Workflow

1. **Upload CSV Data**
   - Click "Upload Dataset" in the left sidebar
   - Select a CSV file from your computer
   - The dataset appears in the "Datasets" list

2. **Add a Chart Widget**
   - Click a chart type button (Line, Bar, Pie, etc.)
   - A new widget appears on the canvas with default configuration

3. **Configure Data Mapping**
   - Select the widget on the canvas
   - In the right sidebar, open "Data Configuration"
   - Map CSV columns to chart axes:
     - **X-Axis**: Category or time field
     - **Y-Axis**: Numeric values
     - **Series**: Grouping field (optional)

4. **Customize Styling**
   - Open "Style Configuration" in the right sidebar
   - Adjust colors, line styles, chart variants
   - Changes reflect in real-time

5. **Position & Organize**
   - Drag widgets to reposition on the grid
   - Use the Layer Panel to manage visibility
   - Resize widgets by editing grid properties

6. **Save & Export**
   - Dashboards auto-save to localStorage every 2 seconds
   - Export templates via "Export Dashboard" button
   - Import saved templates to restore configurations

### Example CSV Format

```csv
Date,Sales,Region,Category
2024-01-01,1200,North,Electronics
2024-01-01,800,South,Furniture
2024-01-02,1500,North,Electronics
2024-01-02,950,South,Furniture
```

**Recommended Structure**:
- **Dates**: ISO format (`YYYY-MM-DD`)
- **Numbers**: Plain integers or decimals
- **Categories**: Text labels
- **Headers**: First row contains column names

---

## Project Architecture

Dashboard Studio follows a **feature-based, layered architecture** with clear separation of concerns:

```
src/
├── lib.rs                          # App entry point, router
├── pages/                          # Top-level routes
│   ├── home.rs                    # Landing page
│   ├── dashboard.rs               # Main dashboard page
│   ├── projects.rs                # Project management
│   └── settings.rs                # User settings
│
├── features/dashboard/            # Dashboard domain logic
│   ├── models.rs                  # Core data structures
│   ├── context.rs                 # DashboardContext (state)
│   ├── components/                # Smart chart widgets
│   │   ├── line_chart_widget.rs
│   │   ├── bar_chart_widget.rs
│   │   ├── pie_chart_widget.rs
│   │   └── ... (11 widget types)
│   ├── config/                    # Chart configuration
│   │   ├── builders/              # Widget builders
│   │   └── style/                 # Style options
│   ├── data/                      # Data processing
│   │   ├── aggregation.rs
│   │   └── transform.rs
│   ├── csv_upload/                # CSV parsing
│   ├── drag_drop.rs               # Drag-and-drop manager
│   ├── history.rs                 # Undo/redo system
│   ├── export/                    # Template export/import
│   └── io.rs                      # localStorage integration
│
├── ui/                             # Reusable UI components
│   ├── atoms/                     # Basic elements
│   │   ├── button.rs
│   │   ├── badge.rs
│   │   └── ... (13 atoms)
│   ├── molecules/                 # Composite components
│   │   ├── data_config_panel.rs
│   │   ├── style_config_panel.rs
│   │   └── ... (15 molecules)
│   └── organisms/                 # Complex layouts
│       ├── header.rs
│       ├── left_sidebar.rs
│       ├── right_sidebar.rs
│       └── canvas_grid.rs
│
└── context/                       # Global contexts
    └── theme.rs                   # Theme management
```

### Key Design Patterns

1. **Signal-Based Reactivity** - Leptos signals for fine-grained reactivity
2. **Context API** - Global state via `DashboardContext`
3. **Smart vs Dumb Components** - Feature components manage state, UI components are pure presentational
4. **Command Pattern** - Undo/redo via `Command` enum
5. **Builder Pattern** - Chart configuration via `WidgetConfigBuilder` trait
6. **Data Transformation Pipeline** - CSV → Dataset → ECharts format
7. **Effect-Based Auto-Save** - Side effects trigger localStorage persistence

---

## Development Guide

### Project Structure Principles

This project follows the architectural patterns outlined in **[CLAUDE.md](./CLAUDE.md)**, which defines:

- **Bottom-Up Design**: Build from atoms → molecules → organisms → features → pages
- **DRY Principle**: Extract reusable components and utilities
- **KISS**: Keep components simple and focused (< 40 lines per `view!` macro)
- **Feature-Based Organization**: Group by domain, not technical type
- **Smart vs Dumb Components**: Separate state logic from presentation

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --target wasm32-unknown-unknown -- -D warnings

# Check compilation
cargo check --target wasm32-unknown-unknown

# Run tests
cargo test --target wasm32-unknown-unknown
```

### Adding a New Chart Type

1. **Define the widget type** in `src/features/dashboard/models.rs`:
   ```rust
   pub enum WidgetType {
       // ... existing types
       YourNewChart,
   }
   ```

2. **Create the widget component** in `src/features/dashboard/components/your_chart_widget.rs`:
   ```rust
   #[component]
   pub fn YourChartWidget(widget_id: Signal<String>) -> impl IntoView {
       // Implementation
   }
   ```

3. **Implement configuration builder** in `src/features/dashboard/config/builders/your_chart.rs`:
   ```rust
   impl WidgetConfigBuilder for YourChartConfig {
       fn build_echarts_options(&self, dataset: &Dataset, mapping: &DataMapping) -> Value {
           // Build ECharts options
       }
   }
   ```

4. **Add style configuration** in `src/features/dashboard/config/style/your_chart.rs`

5. **Register in WidgetSelector** (`src/ui/molecules/widget_selector.rs`)

6. **Update CanvasGrid** to render the new widget type

---

## Roadmap

### Phase 6: Advanced Features (Current)
- [ ] Real-time collaboration support
- [ ] Export to PNG/SVG/PDF
- [ ] Custom color palettes
- [ ] Advanced data transformations
- [ ] Formula/calculated fields

### Phase 7: Enhancement
- [ ] Backend integration (optional API mode)
- [ ] Database connectivity (PostgreSQL, MySQL)
- [ ] Scheduled data refresh
- [ ] User authentication
- [ ] Dashboard sharing via URL

### Phase 8: Enterprise
- [ ] Team collaboration
- [ ] Role-based access control
- [ ] Audit logging
- [ ] Custom branding
- [ ] SSO integration

### Completed Phases
- [x] Phase 1: Project setup (ECharts, Leptos, TailwindCSS)
- [x] Phase 2: ECharts integration
- [x] Phase 3: Chart widget components
- [x] Phase 4: Drag & drop functionality
- [x] Phase 5: Styling & polish

---

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Follow the architecture** outlined in [CLAUDE.md](./CLAUDE.md)
4. **Write clean code** - Run `cargo fmt` and `cargo clippy`
5. **Test your changes** - Ensure all tests pass
6. **Commit your changes** (`git commit -m 'Add amazing feature'`)
7. **Push to the branch** (`git push origin feature/amazing-feature`)
8. **Open a Pull Request**

### Development Setup

See [Getting Started](#getting-started) for installation instructions.

Refer to **[CLAUDE.md](./CLAUDE.md)** for:
- Detailed architecture guidelines
- Component patterns and examples
- State management best practices
- Styling conventions
- Common pitfalls to avoid

---

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- **[Leptos](https://leptos.dev/)** - Reactive web framework for Rust
- **[ECharts](https://echarts.apache.org/)** - Powerful charting library
- **[TailwindCSS](https://tailwindcss.com/)** & **[DaisyUI](https://daisyui.com/)** - Beautiful styling
- **[Trunk](https://trunkrs.dev/)** - WASM build tool
- **[Lucide Icons](https://lucide.dev/)** - Icon library

---

## Support

If you encounter issues or have questions:

- **Issues**: [GitHub Issues](https://github.com/yourusername/dashboard-studio-rs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/dashboard-studio-rs/discussions)
- **Leptos Discord**: [Join the Leptos community](https://discord.gg/leptos)

---

**Built with ❤️ using Rust and Leptos**
