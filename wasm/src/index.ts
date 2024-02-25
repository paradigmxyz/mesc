import init, { greet } from 'mesc_wasm/mesc_wasm.js'

async function run() {
  await init()
  greet('mesc')
}

run()
