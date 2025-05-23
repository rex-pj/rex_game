import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { enhancedImages } from "@sveltejs/enhanced-img";

export default defineConfig({
  plugins: [enhancedImages(), sveltekit()],
  server: {
    https: {
      key: "./.cert/localhost-key.pem",
      cert: "./.cert/localhost.pem",
    },
    host: "localhost",
    port: 5173,
  },
});
