import type { Endpoint, Maybe } from '#/types.ts'

/**
 * @see https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#reference-implementation
 */

interface MESC {
  getEnpointByName({ name }: { name: string }): Endpoint
  getDefaultEndpoint({ profile }?: { profile: string }): Maybe<Endpoint>
  getEndpointByQuery({ query, profile }: { query: string; profile?: string }): Maybe<Endpoint>
  getEndpointByNetwork({ chainId, profile }: { chainId: string | number; profile?: string }): Maybe<Endpoint>
}
