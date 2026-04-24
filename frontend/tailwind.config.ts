import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './app/**/*.{ts,tsx}',
    './components/**/*.{ts,tsx}',
    './lib/**/*.{ts,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        deep: '#0a0a0f',
        surface: '#11111a',
        elevated: '#1a1a25',
        ember: '#ff6b35',
        iron: '#7c8394',
        soul: '#6d5dfc',
        muted: '#8888a0',
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'monospace'],
      },
      boxShadow: {
        'glow-ember': '0 0 20px rgba(255, 107, 53, 0.3)',
        'glow-soul': '0 0 25px rgba(109, 93, 252, 0.25)',
      },
    },
  },
  plugins: [],
}

export default config
