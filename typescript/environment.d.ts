interface EnvironmentVariables {
  readonly NODE_ENV: 'development' | 'production' | 'test'
  readonly MESC_MODE: string
  readonly MESC_PATH: string
  readonly MESC_ENV: string
  readonly MESC_ENDPOINT_METADATA: string
  readonly MESC_GLOBAL_METADATA: string
  readonly MESC_ENDPOINTS: string
  readonly MESC_NETWORK_NAMES: string
  readonly MESC_DEFAULT_ENDPOINT: string
  readonly MESC_NETWORK_DEFAULTS: string
  readonly MESC_PROFILES: string
}

declare namespace NodeJS {
  interface ProcessEnv extends EnvironmentVariables {}
}
