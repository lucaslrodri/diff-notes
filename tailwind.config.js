const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        "things-gray": {
          background: '#F6F7F8',
          stroke: '#EDEFF2',
          divider: '#EBEDF0',
        },
        gray: {
          150: '#F2F3F5',
          250: '#D1D2D6'
        },
        app: {
          red: '#FFD6C9',
          "light-red": '#FFE7D1',
          "dark-red": '#FF6961',
          green: '#CAF2C2',
          "dark-green": '#48D22D',
          "light-green": '#E0FFCC',
          yellow: '#FFF8B8',
          "light-yellow": '#FEFFD6',
          "dark-yellow": '#FFE600'
        }
      },
      boxShadow: {
        'toolbar': '0 1px 2px 0 rgba(229,231,235,1)',
      }
    },
    fontFamily: {
      'sans': ['noto-sans', ...defaultTheme.fontFamily.sans],
      'mono': ['noto-sans-mono', ...defaultTheme.fontFamily.mono]
    }
  },
  plugins: [],
}

