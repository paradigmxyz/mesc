export type NetworkName = 'ethereum' | 'goerli' | 'optimism' | 'polygon' | 'holesky' | 'arbitrum'
export type ChainId = '1' | '5' | '10' | '137' | '17000' | '42161'

export interface RpcConfig {
  mesc_version: string
  default_endpoint: string | null
  network_defaults: Record<string, string>
  network_names: Record<string, string>
  endpoints: Record<string, string>
  profiles: Record<string, string>
  global_metadata: Record<string, string>
}

export interface Query {
  default_endpoint?: string
  endpoint_by_name?: string
  endpoint_by_network?: string
  user_input?: string
  multi_endpoint?: string
  global_metadata?: string
}
