const plugin = require('tailwindcss/plugin');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    plugin(function ({ addUtilities }) {
      addUtilities({
            '.scrollbar-hide': {
              /* IE and Edge */
              '-ms-overflow-style': 'none',

              /* Firefox */
              'scrollbar-width': 'none',

              /* Safari and Chrome */
              '&::-webkit-scrollbar': {
                display: 'none'
              }
            }
          }
      )
    })
  ],
}
