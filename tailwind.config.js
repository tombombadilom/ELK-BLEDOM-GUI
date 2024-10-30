/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        'app-bg-start': '#1a1a1a',
        'app-bg-end': '#2d2d2d',
        'button': '#3a3a3a',
        'button-hover': '#4a4a4a',
        'card': 'rgba(255, 255, 255, 0.05)',
        'card-hover': 'rgba(255, 255, 255, 0.1)',
        'icon-red': '#ff4444',
      },
    },
  },
  plugins: [],
}
