import { defineConfig } from 'vite'
import { resolve } from 'path'

export default defineConfig({
  plugins: [],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    allowedHosts: true
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: process.env.TAURI_ENV_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
    minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    outDir: 'dist',
    rollupOptions: {
      output: {
        entryFileNames: `assets/[name].js`,
        chunkFileNames: `assets/[name]-[hash].js`,
        assetFileNames: `assets/[name].[ext]`
      }
    }
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src')
    }
  }
})
