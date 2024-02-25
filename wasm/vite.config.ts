import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import tsconfigPaths from 'vite-tsconfig-paths'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [wasm(), topLevelAwait(), tsconfigPaths()],
  worker: {
    plugins: () => [wasm(), topLevelAwait(), tsconfigPaths()],
  },
})
