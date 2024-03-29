import * as v from 'valibot'
import {
  parseStringJSON,
  parseMescVariables,
  parseSpaceSeparatedPairs,
  parseSpaceSeparatedProfiles,
  parseSpaceSeparatedEndpoints
} from '#/parsers.ts'
import { raise } from '#/utilities.ts'
import { mescConfigSchema } from '#/schemas/mesc-config.ts'

/**
 * TODO: check if MESC_NETWORK_DEFAULTS is supposed to be merged with existing network_defaults or override it
 * if the latter, then:
 * 'Object.assign(rpcConfig.network_defaults, rpcConfig.network_defaults, mescNetworkDefaults)'
 */

/** @see https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md */
export const mescConfigurationTransform = v.transform(
  mescConfigSchema,
  ({
    MESC_MODE,
    MESC_PATH,
    MESC_ENV,
    MESC_ENDPOINTS,
    MESC_DEFAULT_ENDPOINT,
    MESC_PROFILES,
    MESC_NETWORK_NAMES,
    MESC_NETWORK_DEFAULTS,
    MESC_GLOBAL_METADATA,
    MESC_ENDPOINT_METADATA
  }) => {
    // MESC is not enabled
    if (MESC_MODE === 'DISABLED' || [MESC_MODE, MESC_ENV, MESC_PATH].filter(Boolean).length === 0)
      return null
    const {
      success,
      output: rpcConfig,
      issues
    } = parseMescVariables({ MESC_MODE, MESC_PATH, MESC_ENV })
    if (!success)
      raise(`Failed to parse MESC variables: ${JSON.stringify(v.flatten(issues), undefined, 2)}`)

    if (MESC_DEFAULT_ENDPOINT?.length) {
      Object.assign(rpcConfig, { default_endpoint: MESC_DEFAULT_ENDPOINT })
    }

    const mescNetworkDefaults = parseSpaceSeparatedPairs(MESC_NETWORK_DEFAULTS)
    if (mescNetworkDefaults) {
      Object.assign(rpcConfig, { network_defaults: mescNetworkDefaults })
    }

    const mescNetworkNames = parseSpaceSeparatedPairs(MESC_NETWORK_NAMES)
    if (mescNetworkNames) {
      Object.assign(rpcConfig, { network_names: mescNetworkNames })
    }

    const mescEndpoints = parseSpaceSeparatedEndpoints(MESC_ENDPOINTS)
    if (mescEndpoints) {
      Object.assign(rpcConfig, { endpoints: mescEndpoints })
    }

    const mescProfiles = parseSpaceSeparatedProfiles(MESC_PROFILES)
    if (mescProfiles) {
      Object.assign(rpcConfig, { profiles: mescProfiles })
    }

    const mescGlobalMetadata = parseStringJSON(MESC_GLOBAL_METADATA)
    if (mescGlobalMetadata) {
      Object.assign(rpcConfig.global_metadata, rpcConfig.global_metadata, mescGlobalMetadata)
    }

    const mescEndpointMetadata = parseStringJSON(MESC_ENDPOINT_METADATA)
    if (mescEndpointMetadata) {
      for (const endpoint in mescEndpointMetadata) {
        const metadata = mescEndpointMetadata[endpoint]
        if (!rpcConfig.endpoints[endpoint]) continue
        Object.assign(rpcConfig.endpoints[endpoint]?.endpoint_metadata, metadata)
      }
    }

    return rpcConfig
  }
)

export type MESCConfiguration = v.Output<typeof mescConfigurationTransform>

export function getRpcConfig(env: Record<string, unknown> = process.env): MESCConfiguration {
  const { output, success, issues } = v.safeParse(mescConfigurationTransform, env, {
    abortEarly: false,
    abortPipeEarly: false
  })

  if (!success) raise(`Failed to parse MESC configuration: ${JSON.stringify(issues, undefined, 2)}`)

  return output
}
