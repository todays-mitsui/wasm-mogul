import { defineConfig } from "@solidjs/start/config";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  vite: {
    plugins: [wasm(), topLevelAwait()],
  },
});
