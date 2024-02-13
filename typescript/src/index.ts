export * from '#/types.ts'
import { getRpcConfig } from '#/transform.ts'
import type { Endpoint, Maybe } from '#/types.ts'
import type { ChainId } from '#/schemas/rpc-config.ts'

/**
 * @see https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#reference-implementation
 */

interface MESC {
  getDefaultEndpoint(args?: { profile: string }): Maybe<Endpoint>
  getEndpointByName({ name }: { name: string }): Endpoint
  getEndpointByQuery({ query, profile }: { query: string; profile?: string }): Maybe<Endpoint>
  getEndpointByNetwork({
    chainId,
    profile
  }: { chainId: ChainId; profile?: string }): Maybe<Endpoint>
  findEndpoins(): Endpoint[]
}

export const mescConfiguration = getRpcConfig(process.env)

export const mesc = {
  getDefaultEndpoint: args => {
    throw new Error('Method not implemented.')
  },

  getEndpointByName: args => {
    throw new Error('Method not implemented.')
  },

  getEndpointByQuery: args => {
    throw new Error('Method not implemented.')
  },

  getEndpointByNetwork: args => {
    throw new Error('Method not implemented.')
  },
  findEndpoins: () => {
    throw new Error('Method not implemented.')
  }
} satisfies MESC
