import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    // This is the key part for proxying API calls
    proxy: {
      // Proxy any request that starts with '/api'
      '/api': {
        // The target is your Rust backend server
        target: 'http://localhost:3000',
        // Change the origin to the target URL
        changeOrigin: true,
        // Optional: Rewrite the path to remove the '/api' prefix
        // For example, '/api/hello' becomes '/hello' on the backend
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
})
