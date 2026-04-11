/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    "./index.html"
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#ecfdf5',
          100: '#d1fae5',
          200: '#a7f3d0',
          300: '#6ee7b7',
          400: '#34d399',
          500: '#10b981',
          600: '#059669',
          700: '#047857',
          800: '#065f46',
          900: '#064e3b',
        },
        accent: {
          50: '#fef7ed',
          100: '#fed7aa',
          200: '#fdba74',
          300: '#fb923c',
          400: '#f97316',
          500: '#ea580c',
          600: '#dc2626',
          700: '#c2410c',
          800: '#9a3412',
          900: '#7c2d12',
        }
      }
    }
  },
  plugins: [],
}
