import * as v from 'valibot'

/**
 * chain_id should be represented using either a decimal string or a hex string.
 * Strings are used because chain id's can be 256 bits and most languages do not have native 256 bit integer types.
 * For readability, decimal should be used for small chain id values and hex should be used for values that use the entire 256 bits.
 */
export const chainIdSchema = v.union([v.string(), v.number()])

export type ChainId = v.Output<typeof chainIdSchema>

export const rpcConfigSchema = v.object({
  mesc_version: v.literal('MESC 1.0'),
  default_endpoint: v.optional(v.string()),
  network_defaults: v.record(v.string()),
  network_names: v.record(v.string()),
  endpoints: v.record(
    v.object({
      name: v.string(),
      url: v.string(),
      chain_id: v.optional(v.string()),
      endpoint_metadata: v.any(),
    })
  ),
  profiles: v.record(
    v.object({
      default_endpoint: v.optional(v.nullable(v.string())),
      network_defaults: v.record(v.string()),
    })
  ),
  global_metadata: v.any(),
})

export type RpcConfig = v.Output<typeof rpcConfigSchema>
