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
});
