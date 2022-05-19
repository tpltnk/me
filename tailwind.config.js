module.exports = {
  mode: "all",
  content: ['./dist/*.{html,js}', './pkg/*.{html,js}', "./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        'signika': ['"Signika Negative"', 'sans-serif']
      }
    },
  },
  plugins: [],
}
