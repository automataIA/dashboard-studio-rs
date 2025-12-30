const { addDynamicIconSelectors } = require('@iconify/tailwind');

module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      fontFamily: {
        display: ['Inter', 'system-ui', 'sans-serif'],
      },
      boxShadow: {
        'widget': '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)',
        'widget-hover': '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
        'widget-active': '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-in-out',
        'slide-in': 'slideIn 0.4s ease-out',
        'scale-in': 'scaleIn 0.2s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideIn: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        scaleIn: {
          '0%': { transform: 'scale(0.95)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
      },
    },
  },
  plugins: [
    require('daisyui'),
    addDynamicIconSelectors(),
  ],
  daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
          "primary": "#1C4E80",
          "secondary": "#7C909A",
          "accent": "oklch(70% 0.15 180)",
          "neutral": "#202020",
          "base-100": "#FFFFFF",
          "base-200": "#F8FAFC", // Previously slate-50
          "base-300": "#E2E8F0", // Previously slate-200
          "info": "#0091D5",
          "success": "#6BB187",
          "warning": "#DBAE59",
          "error": "#AC3E31",
          "--rounded-box": "0.5rem",
          "--rounded-btn": "0.3rem",
        },
      },
      {
        business: {
          ...require("daisyui/src/theming/themes")["business"],
          "primary": "#0091D5", // Light Blue for dark mode
          "secondary": "#7C909A",
          "accent": "oklch(75% 0.15 180)",
          "neutral": "#282E36",
          "base-100": "#202020", // Background dark
          "base-200": "#282E36", // Surface dark
          "base-300": "#3A424D", // Border dark
          "info": "#0091D5",
          "success": "#6BB187",
          "warning": "#DBAE59",
          "error": "#AC3E31",
          "--rounded-box": "0.5rem",
          "--rounded-btn": "0.3rem",
        },
      },
    ],
    darkTheme: "business",
  },
};
