import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  base: '/crust/',
  build: {
    target: "es2022"
  },
  worker: {
    format: 'es'
  },
  plugins: [svelte()]
})
