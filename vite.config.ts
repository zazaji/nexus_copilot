// vite.config.ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  // Set the project root to the 'frontend' directory.
  // This is the simplest and most common setup for this project structure.
  root: 'frontend',
  
  plugins: [vue()],
  
  clearScreen: false,
  server: {
    port: 1631,
    strictPort: true,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // The output directory is now relative to the project root (which is 'frontend').
    // We configure it to go up one level and into 'dist', so the final output is `dist/`.
    outDir: '../dist',
    // No need for rollupOptions.input, Vite will automatically find `frontend/index.html`.
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    emptyOutDir: true, // Good practice to clear the dist folder before building
  },
  resolve: {
    alias: {
      // The alias path is now relative to the file location, which is the project root.
      '@': path.resolve(__dirname, './frontend/src'),
    },
  },
})