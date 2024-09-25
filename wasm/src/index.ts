import init, { get_default_endpoint_wrapper, is_mesc_enabled_wrapper } from 'mesc_wasm/mesc_wasm.js'

const mescEnabledElement = document.querySelector('span#mesc-enabled')

async function run() {
  await init()
  const isMescEnabled = is_mesc_enabled_wrapper()
  console.log(`is mesc enabled: ${isMescEnabled}`)
  if (mescEnabledElement) mescEnabledElement.textContent = isMescEnabled.toString()
}

run()
