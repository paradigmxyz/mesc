export * from '#/types.ts'
import { getRpcConfig } from '#/transform.ts'
import type { Endpoint, Maybe } from '#/types.ts'
import type { ChainId } from '#/schemas/rpc-config.ts'

/**
 * @see https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#reference-implementation
 */

interface MESC {
  getDefaultEndpoint({ profile }: { profile?: string }): Maybe<Endpoint>
  getEndpointByName({ name }: { name: string }): Endpoint
  getEndpointByQuery({ query, profile }: { query: string; profile?: string }): Maybe<Endpoint>
  getEndpointByNetwork({ chainId, profile }: { chainId: ChainId; profile?: string }): Maybe<Endpoint>
}

export const mescConfiguration = getRpcConfig()

export const mesc = {
  getDefaultEndpoint({ profile }: { profile: string }): Maybe<Endpoint> {
    const defaultEndpointName = mescConfiguration.profiles[profile]?.['default_endpoint']
    if (!defaultEndpointName) return
    return mescConfiguration.endpoints[defaultEndpointName]
  },

  getEndpointByName({ name }: { name: string }): Endpoint {
    throw new Error('Method not implemented.')
  },

  getEndpointByQuery({ query, profile }: { query: string; profile?: string }): Maybe<Endpoint> {
    throw new Error('Method not implemented.')
  },

  getEndpointByNetwork({ chainId, profile }: { chainId: ChainId; profile?: string }): Maybe<Endpoint> {
    throw new Error('Method not implemented.')
  },
} satisfies MESC
