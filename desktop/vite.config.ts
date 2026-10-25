import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  define: { __MSHOGI_DESKTOP_VERIFICATION__: process.env.MSHOGI_FRONTEND_VERIFICATION === '1' },
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true, host: '127.0.0.1', watch: { ignored: ['**/src-tauri/**'] } },
  build: { target: ['es2022', 'safari15'], sourcemap: false },
});
