import * as v from 'valibot'

export const environmentVariablesSchema = v.object({
  NODE_ENV: v.union([v.literal('development'), v.literal('production'), v.literal('test')]),
  MESC_MODE: v.union([v.literal('PATH'), v.literal('ENV'), v.literal('DISABLED')]),
  MESC_PATH: v.string(),
  MESC_ENV: v.string(),
  /** JSON mapping of {"endpoint_name": {<ENDPOINT_METADATA>}} */
  MESC_ENDPOINT_METADATA: v.optional(v.string()),
  /** JSON formatted global metadata */
  MESC_GLOBAL_METADATA: v.optional(v.string()),
  /** space-separated items of [<name>[:<chain_id>]=]<url> */
  MESC_ENDPOINTS: v.optional(v.string([v.regex(/(\w+(\.\w+)?(:\d+)?=[\w\d]+)+/)])),
  /** space-separated pairs of <name>=<chain_id> */
  MESC_NETWORK_NAMES: v.optional(v.string([v.regex(/(\w+=[\w\d]+)+/)])),
  /** url, endpoint name, or network name	 */
  MESC_DEFAULT_ENDPOINT: v.optional(v.string()),
  /** space-separated pairs of <profile>.<key>[.<chain_id]=<endpoint>	 */
  MESC_PROFILES: v.optional(v.string([v.regex(/(\w+\.\w+(\.\d+)?)=[\w\d]+/)])),
  /** space-separated pairs of <chain_id>=<endpoint> */
  MESC_NETWORK_DEFAULTS: v.optional(v.string([v.regex(/(\d+=[\w\d]+)+/)])),
})
