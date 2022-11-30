import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import sveltePreprocess from "svelte-preprocess";

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    proxy: {
      "^/(api|auth)": {
        target: "http://localhost:8080",
      },
    },
  },
  plugins: [
    svelte({
      preprocess: sveltePreprocess(),
    }),
  ],
});
