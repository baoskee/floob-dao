/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: 'white',
        secondary: '#A9A9A9',
        cta: '#FF13CB'
      }
    },
  },
  plugins: [],
}
