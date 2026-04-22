import { env } from 'node:process';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import tailwind from '@tailwindcss/vite';
import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
  plugins: [
    tailwind(),
    vue({ features: { optionsAPI: false } }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('src', import.meta.url)),
    },
  },
  clearScreen: false,
  publicDir: 'src/public',
  build: {
    outDir: 'dist',
    copyPublicDir: true,
    emptyOutDir: true,
    minify: true,
    sourcemap: false,
  },
  server: {
    port: 1420,
    strictPort: true,
    host: env.TAURI_DEV_HOST ?? false,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
});
