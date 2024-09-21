import { defineConfig } from '@rsbuild/core';
import { pluginReact } from "@rsbuild/plugin-react";
import path from "path";

export default defineConfig({
  plugins: [pluginReact()],
  resolve: {
    alias: {
      // !! We need to install @types/node to avoid errors(cannot find papth or __dirname).
      "@": path.resolve(__dirname, "src"),
    },
  },
  html: {
    template: './index.html',
  },
  source: {
    entry: {
      index: './src/main.tsx',
    },
  },
});
