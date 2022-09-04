/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.vue"],
  theme: {
    extend: {
      'colors': {
        'sc-gray': '#8d8d8d',
        'hr': '#9696961a',
        'nav-bar': '#1A1A1A',
      },
      'fontSize': {
        'xl': ['72px', '64px']
      },
    },
  },
  plugins: [],
}
