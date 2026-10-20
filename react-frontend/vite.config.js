import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
  server: {
    proxy: {
      '/admin/api': 'http://127.0.0.1:18090',
      '/api': 'http://127.0.0.1:18090',
      '/invite': 'http://127.0.0.1:18090',
      '/media': 'http://127.0.0.1:18090',
      '/health': 'http://127.0.0.1:18090',
    },
  },
})
