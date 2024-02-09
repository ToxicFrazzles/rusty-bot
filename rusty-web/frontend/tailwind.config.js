/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        "text-accent": "#FFFFFF",
        "bg-accent": "#273043",
        "text-primary": "#000000",
        "bg-primary": "#9197AE",
        "bg-side": "#b8d09f"
      }
    },
  },
  plugins: [],
}

