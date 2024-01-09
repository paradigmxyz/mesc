import type { ChainId } from '#/schemas/rpc-config.ts'

export type NetworkName = 'ethereum' | 'goerli' | 'optimism' | 'polygon' | 'holesky' | 'arbitrum'

export interface RpcConfig {
  mesc_version: string
  default_endpoint?: string
  endpoints: Record<string, Endpoint>
  network_defaults: Record<ChainId, string>
  network_names: Record<string, ChainId>
  profiles: Record<string, Profile>
  global_metadata: Record<string, Json>
}

export interface Endpoint {
  name: string
  url: string
  chain_id?: ChainId
  endpoint_metadata: Record<string, Json>
}

export interface Profile {
  name: string
  default_endpoint?: string
  network_defaults: Record<ChainId, string>
}

export interface Query {
  default_endpoint?: string
  endpoint_by_name?: string
  endpoint_by_network?: string
  user_input?: string
  multi_endpoint?: string
  global_metadata?: string
}

export type Maybe<T> = T | undefined

type Json = string | number | boolean | null | Json[] | { [key: string]: Json }

/**
 * This type utility is used to unwrap complex types so you can hover over them in your editor
 */
export type Pretty<T> = {
  [K in keyof T]: T[K]
} & {}
