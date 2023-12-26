import { raise } from '#/utilities.ts'

export const mescEnv = Object.freeze({
  NODE_ENV: getEnvVariable('NODE_ENV'),
  MESC_MODE: getEnvVariable('MESC_MODE'),
  MESC_PATH: getEnvVariable('MESC_PATH'),
  MESC_ENV: getEnvVariable('MESC_ENV'),
  MESC_ENDPOINT_METADATA: getEnvVariable('MESC_ENDPOINT_METADATA'),
  MESC_GLOBAL_METADATA: getEnvVariable('MESC_GLOBAL_METADATA'),
  MESC_ENDPOINTS: getEnvVariable('MESC_ENDPOINTS'),
  MESC_NETWORK_NAMES: getEnvVariable('MESC_NETWORK_NAMES'),
  MESC_DEFAULT_ENDPOINT: getEnvVariable('MESC_DEFAULT_ENDPOINT'),
  MESC_NETWORK_DEFAULTS: getEnvVariable('MESC_NETWORK_DEFAULTS'),
  MESC_PROFILES: getEnvVariable('MESC_PROFILES'),
}) as Readonly<EnvironmentVariables>

function getEnvVariable<T extends keyof EnvironmentVariables>(name: T) {
  return process.env[name] ?? raise(`environment variable ${name} not found`)
}
