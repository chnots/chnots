import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      // !! We need to install @types/node to avoid errors(cannot find papth or __dirname).
      "@": path.resolve(__dirname, "src"),
    },
  },
  build: {
    // Tauri supports es2021
    target: ["es2021", "chrome100", "safari13"],
    outDir: "../web-dist",
    emptyOutDir: true,
    // don't minify for debug builds
    minify: true,
    // produce sourcemaps for debug builds
    sourcemap: false,
  },
});
