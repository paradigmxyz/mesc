import * as v from 'valibot'
import { removeUndefined } from '#/utilities.ts'

/**
 * parse space-separated items of <profile>.<key>[.<chain_id]=<endpoint>into a mapping
 */
function parseSpaceSeparatedProfiles(input?: string) {
  if (!input) return {}
  return input.split(' ').reduce((accumulator, pair) => {
    const [_key, value] = pair.split('=') as [string, string]
    const [profile, key, chain_id] = _key.split('.') as [string, string, string | undefined]
    console.log({ profile, key, value, chain_id })
    return {
      ...accumulator,
      [profile]: {
        ...accumulator[profile],
        [key]: { ...accumulator[profile]?.[key], [`${chain_id}`]: value },
      },
    }
  }, {} as Record<string, Record<string, Record<string, string>>>)
}

/**
 * parse space-separated items of [<name>[:<chain_id>]=]<url> into a mapping
 * returns {Record<string, { url: string, chain_id?: string }>}
 */
function parseSpaceSeparatedEndpoints(input?: string) {
  if (!input) return {}
  return removeUndefined(
    input.split(' ').reduce((accumulator, item) => {
      const [name, url] = item.split('=') as [string, string]
      const [name_, chain_id] = name.split(':') as [string, string | undefined]
      return { ...accumulator, [name_]: { url, chain_id } }
    }, {})
  ) as Record<string, { url: string; chain_id?: string }>
}

/**
 * parse space-separated pairs of <key>=<value> into a mapping
 */
function parseSpaceSeparatedItems(input?: string) {
  if (!input) return {}
  return input.split(' ').reduce((accumulator, pair) => {
    const [key, value] = pair.split('=') as [string, string]
    return { ...accumulator, [key]: value }
  }, {})
}

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

export const environmentVariablesTransform = v.transform(
  environmentVariablesSchema,
  ({
    NODE_ENV,
    MESC_MODE,
    MESC_PATH,
    MESC_ENV,
    MESC_ENDPOINTS,
    MESC_DEFAULT_ENDPOINT,
    MESC_PROFILES,
    MESC_NETWORK_NAMES,
    MESC_NETWORK_DEFAULTS,
    MESC_GLOBAL_METADATA,
    MESC_ENDPOINT_METADATA,
  }) => {
    if (!MESC_MODE || MESC_MODE === 'DISABLED') {
      throw new Error('MESC_MODE must be set to PATH or ENV')
    }
    return {
      NODE_ENV,
      MESC_MODE,
      MESC_PATH,
      MESC_ENV,
      MESC_DEFAULT_ENDPOINT,
      MESC_PROFILES: parseSpaceSeparatedProfiles(MESC_PROFILES),
      MESC_ENDPOINTS: parseSpaceSeparatedEndpoints(MESC_ENDPOINTS),
      MESC_NETWORK_NAMES: parseSpaceSeparatedItems(MESC_NETWORK_NAMES),
      MESC_NETWORK_DEFAULTS: parseSpaceSeparatedItems(MESC_NETWORK_DEFAULTS),
      MESC_GLOBAL_METADATA: removeUndefined(MESC_GLOBAL_METADATA),
      MESC_ENDPOINT_METADATA: removeUndefined(MESC_ENDPOINT_METADATA),
    }
  }
)

export type EnvironmentVariables = v.Output<typeof environmentVariablesTransform>

export const environmentVariables = v.parse(environmentVariablesTransform, process.env, {
  abortEarly: false,
  abortPipeEarly: false,
})
