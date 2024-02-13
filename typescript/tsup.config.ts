import { defineConfig } from 'tsup'

export default defineConfig({
  dts: true,
  shims: true,
  clean: true,
  bundle: true,
  outDir: 'dist',
  format: ['esm'],
  target: ['node20'],
  treeshake: 'recommended',
  entry: ['./src/index.ts']
})
