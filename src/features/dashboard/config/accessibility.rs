//! Accessibility configuration for dashboard widgets
//!
//! Provides WCAG AA/AAA compliance options for charts and visualizations.

#![allow(dead_code)]  // Module prepared for future use

use serde::{Deserialize, Serialize};

/// Accessibility configuration for ECharts widgets
///
/// Controls accessibility features like colorblind patterns,
/// contrast ratios, and high-contrast modes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    /// Enable WCAG AAA patterns for colorblind users
    ///
    /// When enabled, charts use SVG patterns (stripes, dots, etc.)
    /// in addition to colors, making them distinguishable for users
    /// with color vision deficiencies.
    #[serde(default)]
    pub enable_patterns: bool,

    /// Minimum contrast ratio for text and UI elements
    ///
    /// - 4.5:1 = WCAG AA compliance (normal text)
    /// - 7.0:1 = WCAG AAA compliance (enhanced contrast)
    /// - 3.0:1 = WCAG AA for large text and UI components
    #[serde(default = "default_min_contrast_ratio")]
    pub min_contrast_ratio: f32,

    /// Enable high-contrast mode
    ///
    /// When enabled, uses maximum contrast colors (pure black/white text)
    /// and bolder borders for better visibility.
    #[serde(default)]
    pub high_contrast_mode: bool,
}

fn default_min_contrast_ratio() -> f32 {
    4.5  // WCAG AA standard
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            enable_patterns: false,  // Opt-in to avoid changing existing UX
            min_contrast_ratio: default_min_contrast_ratio(),
            high_contrast_mode: false,
        }
    }
}

impl AccessibilityConfig {
    /// Create a new accessibility config with WCAG AA compliance
    pub fn wcag_aa() -> Self {
        Self {
            enable_patterns: false,
            min_contrast_ratio: 4.5,
            high_contrast_mode: false,
        }
    }

    /// Create a new accessibility config with WCAG AAA compliance
    pub fn wcag_aaa() -> Self {
        Self {
            enable_patterns: true,   // Patterns for colorblind support
            min_contrast_ratio: 7.0, // Enhanced contrast
            high_contrast_mode: true,
        }
    }

    /// Check if the config meets WCAG AA standards
    pub fn is_wcag_aa_compliant(&self) -> bool {
        self.min_contrast_ratio >= 4.5
    }

    /// Check if the config meets WCAG AAA standards
    pub fn is_wcag_aaa_compliant(&self) -> bool {
        self.min_contrast_ratio >= 7.0 && self.enable_patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AccessibilityConfig::default();
        assert!(!config.enable_patterns);
        assert_eq!(config.min_contrast_ratio, 4.5);
        assert!(!config.high_contrast_mode);
        assert!(config.is_wcag_aa_compliant());
        assert!(!config.is_wcag_aaa_compliant());
    }

    #[test]
    fn test_wcag_aa_config() {
        let config = AccessibilityConfig::wcag_aa();
        assert!(config.is_wcag_aa_compliant());
    }

    #[test]
    fn test_wcag_aaa_config() {
        let config = AccessibilityConfig::wcag_aaa();
        assert!(config.is_wcag_aa_compliant());
        assert!(config.is_wcag_aaa_compliant());
        assert!(config.enable_patterns);
        assert_eq!(config.min_contrast_ratio, 7.0);
    }
}
