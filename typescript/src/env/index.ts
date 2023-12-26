import * as v from 'valibot'
import { removeUndefined } from '#/utilities.ts'
import { environmentVariablesSchema } from './schema.ts'
import { parseSpaceSeparatedPairs, parseSpaceSeparatedProfiles, parseSpaceSeparatedEndpoints } from './parsers.ts'

/**
 * @see https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md
 */

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
      MESC_NETWORK_NAMES: parseSpaceSeparatedPairs(MESC_NETWORK_NAMES),
      MESC_NETWORK_DEFAULTS: parseSpaceSeparatedPairs(MESC_NETWORK_DEFAULTS),
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
