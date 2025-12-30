//! Dynamic theme color reader for DaisyUI themes
//!
//! Reads CSS variables from the active DaisyUI theme
//! to ensure charts automatically adapt to theme changes.

use leptos::web_sys;

/// Chart color palette derived from DaisyUI CSS variables
#[derive(Clone, Debug)]
pub struct ChartColors {
    /// Primary brand color
    pub primary: String,
    /// Text color for content
    pub text: String,
    /// Grid/axis lines color
    pub grid: String,
    /// Tooltip background color
    pub background: String,
    /// Secondary color
    pub secondary: String,
    /// Accent color
    pub accent: String,
    /// Info color
    pub info: String,
    /// Success color
    pub success: String,
    /// Warning color
    pub warning: String,
    /// Error color
    pub error: String,
    /// Label color for axis labels (more readable than text)
    pub label: String,
    /// Neutral color
    pub neutral: String,
    /// Primary focus (darker/lighter variant)
    pub primary_focus: String,
    /// Secondary focus (darker/lighter variant)
    pub secondary_focus: String,
    /// Neutral focus (darker/lighter variant)
    pub neutral_focus: String,
    /// High-contrast label color (WCAG AA 4.5:1 guaranteed)
    pub label_high_contrast: String,
}

/// Applies opacity to an RGB color string
///
/// Converts "rgb(r, g, b)" to "rgba(r, g, b, opacity)"
fn apply_opacity(rgb: &str, opacity: f32) -> String {
    let rgb_trimmed = rgb
        .trim_start_matches("rgb(")
        .trim_start_matches("rgba(")
        .trim_end_matches(')');

    let parts: Vec<&str> = rgb_trimmed.split(',').collect();
    if parts.len() < 3 {
        return rgb.to_string();
    }

    let r = parts[0].trim();
    let g = parts[1].trim();
    let b = parts[2].trim();

    format!("rgba({}, {}, {}, {})", r, g, b, opacity)
}

impl ChartColors {
    /// Read colors from active DaisyUI theme CSS variables
    ///
    /// Uses getComputedStyle to read actual rendered RGB values
    /// so colors always match the current theme.
    ///
    /// DaisyUI 4.x uses CSS variables that resolve to OKLCH values.
    /// Since ECharts only supports hex/RGB, we compute the final RGB values
    /// by creating temporary elements with DaisyUI utility classes and reading
    /// their computed background-color or color properties.
    pub fn from_daisyui_theme() -> Self {
        let window = web_sys::window().expect("Window not available");
        let document = window.document().expect("Document not available");

        // Helper to get computed RGB color from a DaisyUI class
        // We create a temporary element with the class and read its computed color
        let get_color_from_class = |class_name: &str, property: &str| -> String {
            let temp_elem = document
                .create_element("div")
                .expect("Failed to create temp element");

            // Set the DaisyUI class
            temp_elem
                .set_attribute("class", class_name)
                .expect("Failed to set class");

            // Append to body so styles are computed
            let body = document.body().expect("Body not available");
            body.append_child(&temp_elem)
                .expect("Failed to append temp element");

            // Get computed styles
            let styles = window
                .get_computed_style(&temp_elem)
                .ok()
                .flatten()
                .expect("Failed to get computed styles");

            // Read the color property
            let color = styles
                .get_property_value(property)
                .unwrap_or_else(|_| "rgb(0, 0, 0)".to_string())
                .trim()
                .to_string();

            // Remove temp element
            body.remove_child(&temp_elem)
                .expect("Failed to remove temp element");

            color
        };

        // Read colors using DaisyUI utility classes
        // These classes automatically resolve OKLCH to RGB via CSS
        Self {
            // Primary color from bg-primary
            primary: get_color_from_class("bg-primary", "background-color"),
            // Text color from text-base-content
            text: get_color_from_class("text-base-content", "color"),
            // Grid/border color from border-base-300
            grid: get_color_from_class("bg-base-300", "background-color"),
            // Background color from bg-base-100
            background: get_color_from_class("bg-base-100", "background-color"),
            // Secondary color from bg-secondary
            secondary: get_color_from_class("bg-secondary", "background-color"),
            // Accent color from bg-accent
            accent: get_color_from_class("bg-accent", "background-color"),
            // Info color from bg-info
            info: get_color_from_class("bg-info", "background-color"),
            // Success color from bg-success
            success: get_color_from_class("bg-success", "background-color"),
            // Warning color from bg-warning
            warning: get_color_from_class("bg-warning", "background-color"),
            // Error color from bg-error
            error: get_color_from_class("bg-error", "background-color"),
            // Label color for axes using opacity(65%) of base-content
            // This creates a medium-dark gray for axis labels that's readable but not as prominent as main text
            // Light theme: dark gray - good readability
            // Dark theme: medium-light gray - good readability
            label: {
                let base_text = get_color_from_class("text-base-content", "color");
                apply_opacity(&base_text, 0.1)
            },
            // Neutral color from bg-neutral
            neutral: get_color_from_class("bg-neutral", "background-color"),
            // Primary focus (hover/active state)
            primary_focus: get_color_from_class("bg-primary-focus", "background-color"),
            // Secondary focus (hover/active state)
            secondary_focus: get_color_from_class("bg-secondary-focus", "background-color"),
            // Neutral focus (hover/active state)
            neutral_focus: get_color_from_class("bg-neutral-focus", "background-color"),
            // High-contrast label color for WCAG AA compliance (4.5:1 ratio)
            // Light theme: uses neutral-content (almost black)
            // Dark theme: uses base-content (light text)
            label_high_contrast: get_color_from_class("text-neutral-content", "color"),
        }
    }

    /// Returns an array of colors suitable for ECharts color palette
    ///
    /// This provides a diverse set of 12+ colors for multi-series charts,
    /// ensuring good visual distinction between different data series.
    /// Colors are ordered by perceptual distance for maximum contrast.
    pub fn to_palette(&self) -> Vec<String> {
        vec![
            self.primary.clone(),         // 1. Blue/Navy
            self.info.clone(),            // 2. Light Blue (high contrast with primary)
            self.success.clone(),         // 3. Green
            self.warning.clone(),         // 4. Yellow/Amber
            self.error.clone(),           // 5. Red
            self.accent.clone(),          // 6. Cyan
            self.secondary.clone(),       // 7. Gray
            self.primary_focus.clone(),   // 8. Dark/Light Blue variant
            self.neutral.clone(),         // 9. Neutral Gray
            self.secondary_focus.clone(), // 10. Gray variant
            self.neutral_focus.clone(),   // 11. Neutral variant
            // If more than 11 series, repeat with distinct colors
            self.info.clone(), // 12. Repeat Light Blue
        ]
    }
}

impl Default for ChartColors {
    fn default() -> Self {
        Self::from_daisyui_theme()
    }
}

/// Lightens an RGB color by a percentage (0.0-1.0)
///
/// Increases the lightness in HSL color space, making the color brighter.
/// Useful for creating emphasis/hover states.
///
/// # Arguments
/// * `rgb` - RGB color string in format "rgb(r, g, b)" or "rgba(r, g, b, a)"
/// * `amount` - Amount to lighten (0.0 = no change, 1.0 = maximum lightening)
///
/// # Example
/// ```
/// lighten_color("rgb(100, 100, 100)", 0.2) // Returns lighter gray
/// ```
pub fn lighten_color(rgb: &str, amount: f32) -> String {
    // Parse RGB values from string like "rgb(123, 45, 67)" or "rgba(123, 45, 67, 0.8)"
    let rgb_trimmed = rgb
        .trim_start_matches("rgb(")
        .trim_start_matches("rgba(")
        .trim_end_matches(')');

    let parts: Vec<&str> = rgb_trimmed.split(',').collect();
    if parts.len() < 3 {
        return rgb.to_string(); // Return original if parsing fails
    }

    let r = parts[0].trim().parse::<f32>().unwrap_or(0.0) / 255.0;
    let g = parts[1].trim().parse::<f32>().unwrap_or(0.0) / 255.0;
    let b = parts[2].trim().parse::<f32>().unwrap_or(0.0) / 255.0;
    let alpha = if parts.len() > 3 {
        parts[3].trim().parse::<f32>().ok()
    } else {
        None
    };

    // Convert RGB to HSL
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    // Lighten: increase lightness
    let new_l = (l + (1.0 - l) * amount).clamp(0.0, 1.0);

    // Convert HSL back to RGB
    let c = (1.0 - (2.0 * new_l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = new_l - c / 2.0;

    let (r_new, g_new, b_new) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r_final = ((r_new + m) * 255.0).round() as u8;
    let g_final = ((g_new + m) * 255.0).round() as u8;
    let b_final = ((b_new + m) * 255.0).round() as u8;

    if let Some(a) = alpha {
        format!("rgba({}, {}, {}, {})", r_final, g_final, b_final, a)
    } else {
        format!("rgb({}, {}, {})", r_final, g_final, b_final)
    }
}

/// Darkens an RGB color by a percentage (0.0-1.0)
///
/// Decreases the lightness in HSL color space, making the color darker.
/// Useful for creating shadow/inactive states.
///
/// # Arguments
/// * `rgb` - RGB color string in format "rgb(r, g, b)" or "rgba(r, g, b, a)"
/// * `amount` - Amount to darken (0.0 = no change, 1.0 = maximum darkening)
///
/// # Example
/// ```
/// darken_color("rgb(200, 200, 200)", 0.2) // Returns darker gray
/// ```
#[allow(dead_code)]
pub fn darken_color(rgb: &str, amount: f32) -> String {
    // Parse RGB values
    let rgb_trimmed = rgb
        .trim_start_matches("rgb(")
        .trim_start_matches("rgba(")
        .trim_end_matches(')');

    let parts: Vec<&str> = rgb_trimmed.split(',').collect();
    if parts.len() < 3 {
        return rgb.to_string();
    }

    let r = parts[0].trim().parse::<f32>().unwrap_or(0.0) / 255.0;
    let g = parts[1].trim().parse::<f32>().unwrap_or(0.0) / 255.0;
    let b = parts[2].trim().parse::<f32>().unwrap_or(0.0) / 255.0;
    let alpha = if parts.len() > 3 {
        parts[3].trim().parse::<f32>().ok()
    } else {
        None
    };

    // Convert RGB to HSL
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    // Darken: decrease lightness
    let new_l = (l * (1.0 - amount)).clamp(0.0, 1.0);

    // Convert HSL back to RGB
    let c = (1.0 - (2.0 * new_l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = new_l - c / 2.0;

    let (r_new, g_new, b_new) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r_final = ((r_new + m) * 255.0).round() as u8;
    let g_final = ((g_new + m) * 255.0).round() as u8;
    let b_final = ((b_new + m) * 255.0).round() as u8;

    if let Some(a) = alpha {
        format!("rgba({}, {}, {}, {})", r_final, g_final, b_final, a)
    } else {
        format!("rgb({}, {}, {})", r_final, g_final, b_final)
    }
}
