import type { ChainId } from '#/schemas/rpc-config.ts'
import type { NetworkName, RpcConfig } from '#/types.ts'

export const knownNetworks = {
  ethereum: '1',
  goerli: '5',
  optimism: '10',
  polygon: '137',
  holesky: '17000',
  arbitrum: '42161'
} satisfies Record<NetworkName, ChainId>

export function networkNameToChainId({
  networkName,
  config
}: {
  networkName: string
  config?: RpcConfig
}) {
  networkName = networkName.toLowerCase()
  if (!stringIsNetworkName(networkName)) {
    throw new Error(`Unknown network name: ${networkName}`)
  }
  if (!config) return knownNetworks[networkName]
  return config['network_names'][networkName]
}

function stringIsNetworkName(input: string): input is NetworkName {
  return input in knownNetworks
}
